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
    info!("Seeding the user table...");

    let mut rdr = csv::Reader::from_path("./src/fixtures/user/studenti_24_25.csv")?;

    let users = rdr
        .deserialize()
        .map(|result| {
            let record = result?;
            Ok(record)
        })
        .collect::<Result<Vec<UserData>>>()?;

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        r#"
        INSERT INTO "user" (name, email, class, section)
        "#,
    );

    query_builder.push_values(users, |mut b, user| {
        b.push_bind(format!("{} {}", user.first_name, user.last_name))
            .push_bind(user.email)
            .push_bind(user.class)
            .push_bind(user.section);
    });

    query_builder.build().execute(&db).await?;

    info!("User table seeded");

    Ok(())
}
