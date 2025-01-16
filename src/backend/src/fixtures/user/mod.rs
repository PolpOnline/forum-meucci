use color_eyre::{eyre::eyre, Result};
use indicatif::ProgressBar;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserData {
    last_name: String,
    first_name: String,
    email: String,
    class: i32,
    section: String,
}

pub async fn seed(db: PgPool) -> Result<()> {
    let mut rdr = csv::Reader::from_path("./src/fixtures/user/studenti_24_25.csv")?;

    let data = rdr
        .deserialize()
        .map(|result| {
            let record = result?;
            Ok(record)
        })
        .collect::<Result<Vec<UserData>>>()?;

    info!("Seeding the user table...");

    let bar = ProgressBar::new(data.len() as u64);

    let mut txn = db.begin().await?;

    for user_data in data {
        bar.inc(1);

        match sqlx::query!(
            // language=PostgreSQL
            r#"
            INSERT INTO "user" (name, email, class, section)
            VALUES ($1, $2, $3, $4)
            "#,
            format!("{} {}", user_data.first_name, user_data.last_name),
            user_data.email,
            user_data.class,
            user_data.section
        )
        .execute(&mut *txn)
        .await
        {
            Ok(_) => (),
            Err(e) => {
                return Err(eyre!(
                    "Error inserting user: {:?} (email: {})",
                    e,
                    user_data.email
                ));
            }
        }
    }

    bar.finish_and_clear();
    info!("User table seeded");

    txn.commit().await?;

    Ok(())
}
