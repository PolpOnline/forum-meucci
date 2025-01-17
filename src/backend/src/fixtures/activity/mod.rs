mod space_deserialize;

use color_eyre::{eyre::eyre, Result};
use indicatif::ProgressBar;
use serde::Deserialize;
use space_deserialize::space_deserialize;
use sqlx::PgPool;
use tracing::info;

#[derive(Debug, Deserialize)]
struct ActivityData {
    nome: String,
    descrizione: String,
    stanza: String,
    classe_minima: i32,
    #[serde(deserialize_with = "space_deserialize")]
    email_host: Vec<String>,
    #[serde(deserialize_with = "space_deserialize")]
    massimo_utenti_round: Vec<i32>,
}

pub async fn seed(db: PgPool) -> Result<()> {
    info!("Seeding the activity table...");

    let mut rdr =
        csv::Reader::from_path("./src/fixtures/activity/Attività_Forum_24_25_template.csv")?;

    let data = rdr
        .deserialize()
        .map(|result| {
            let record = result?;
            Ok(record)
        })
        .collect::<Result<Vec<ActivityData>>>()?;

    // Check that the number of rounds is the same for all activities
    let rounds = data
        .iter()
        .map(|activity| activity.massimo_utenti_round.len())
        .collect::<Vec<_>>();

    if !(rounds.iter().all(|round| *round == rounds[0])) {
        return Err(eyre!(
            "The number of rounds is different for some activities: {:?}",
            rounds
        ));
    }

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
        .fetch_all(&db);

        // Insert the activity basic information
        let event_id_fut = sqlx::query!(
            r#"
            INSERT INTO activity (name, description, room, minimum_class)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            activity_data.nome,
            activity_data.descrizione,
            activity_data.stanza,
            activity_data.classe_minima
        )
        .fetch_one(&mut *txn);

        let (host_ids, event_id) = futures::future::try_join(host_ids_fut, event_id_fut).await?;

        // Check if the number of hosts is the same as the number of emails
        if activity_data.email_host.len() != host_ids.len() {
            return Err(eyre!(
                "Some hosts were not found in the database: {:?}",
                activity_data.email_host
            ));
        }

        // Add hosts to the activity
        for event_admin in host_ids {
            sqlx::query!(
                r#"
                INSERT INTO activity_admin (activity_id, user_id)
                VALUES ($1, $2)
                "#,
                event_id.id,
                event_admin.id
            )
            .execute(&mut *txn)
            .await?;

            sqlx::query!(
                r#"
                UPDATE "user" SET type = 'host' WHERE id = $1
                "#,
                event_admin.id
            )
            .execute(&mut *txn)
            .await?;
        }

        // Add the maximum users per round
        for (idx, max_users) in activity_data.massimo_utenti_round.into_iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO round_max_users (round, activity_id, max_users)
                VALUES ($1, $2, $3)
                "#,
                idx as i32,
                event_id.id,
                max_users
            )
            .execute(&mut *txn)
            .await?;
        }
    }

    bar.finish_and_clear();
    info!("Activity table seeded");

    txn.commit().await?;

    Ok(())
}
