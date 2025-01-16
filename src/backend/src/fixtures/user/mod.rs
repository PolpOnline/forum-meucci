use color_eyre::Result;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::error;

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

    let insert_fut = data.into_iter().map(|record| {
        sqlx::query!(
            // language=PostgreSQL
            r#"
            INSERT INTO "user" (name, email, class, section)
            VALUES ($1, $2, $3, $4)
            "#,
            format!("{} {}", record.first_name, record.last_name),
            record.email,
            record.class,
            record.section
        )
        .execute(&db)
    });

    let results = futures::future::join_all(insert_fut).await;

    for result in results {
        match result {
            Ok(_) => (),
            Err(e) => error!("Error inserting user: {:?}", e),
        }
    }

    Ok(())
}
