use color_eyre::Result;
use serde::Deserialize;
use sqlx::{PgPool, Postgres, QueryBuilder};
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

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
        INSERT INTO "user" (name, email, class, section)
        "#,
    );

    query_builder.push_values(data, |mut b, user_data| {
        b.push_bind(format!("{} {}", user_data.first_name, user_data.last_name))
            .push_bind(user_data.email)
            .push_bind(user_data.class)
            .push_bind(user_data.section);
    });

    let mut txn = db.begin().await?;

    query_builder.build().execute(&mut *txn).await?;

    info!("User table seeded");

    txn.commit().await?;

    Ok(())
}
