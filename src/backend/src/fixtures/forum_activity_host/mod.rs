use color_eyre::eyre::eyre;
use indicatif::ProgressBar;
use sqlx::PgPool;
use tracing::info;

//noinspection DuplicatedCode
pub async fn seed(db: &PgPool, write: bool) -> color_eyre::Result<()> {
    info!("Seeding the hosts...");

    let mut rdr = csv::Reader::from_path("../forum_activity/Attività_Forum_24_25_template.csv")?;

    let data = rdr
        .deserialize()
        .map(|result| {
            let record = result?;
            Ok(record)
        })
        .collect::<color_eyre::Result<Vec<crate::fixtures::forum_activity::ActivityData>>>()?;

    let bar = ProgressBar::new(data.len() as u64);

    // Start a transaction
    let mut txn = db.begin().await?;

    for activity_data in data {
        bar.inc(1);

        // Get the host ids without a transaction because only one txn can be active at
        // a time
        let host_ids_fut = sqlx::query!(
            r#"
            SELECT id
            FROM "user"
            WHERE email = ANY($1)
            "#,
            &activity_data.email_host
        )
        .fetch_all(db);

        // Get the event id
        let event_id_fut = sqlx::query!(
            r#"
            SELECT id FROM forum_activity
            WHERE name = $1
            "#,
            activity_data.nome
        )
        .fetch_one(db);

        let (host_ids, event_id) = futures::future::try_join(host_ids_fut, event_id_fut).await?;

        // Check if the number of hosts is the same as the number of emails
        if activity_data.email_host.len() != host_ids.len() {
            return Err(eyre!(
                "Some hosts were not found in the database: {:?}",
                activity_data.email_host
            ));
        }

        // Add hosts to the forum_activity
        for event_admin in host_ids {
            sqlx::query!(
                r#"
                INSERT INTO forum_activity_host (activity_id, user_id)
                VALUES ($1, $2)
                "#,
                event_id.id,
                event_admin.id
            )
            .execute(&mut *txn)
            .await?;

            sqlx::query!(
                r#"
                UPDATE "user" SET forum_role = 'host' WHERE id = $1
                "#,
                event_admin.id
            )
            .execute(&mut *txn)
            .await?;
        }
    }

    bar.finish_and_clear();

    if write {
        txn.commit().await?;
    } else {
        txn.rollback().await?;
    }

    info!(
        "Admins set ({})",
        if write { "Committed" } else { "Rolled Back" }
    );

    Ok(())
}
