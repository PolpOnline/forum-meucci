use color_eyre::Result;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{debug, info};

#[derive(Debug, Deserialize)]
struct AdminsData {
    email_admin: String,
}

pub async fn seed(db: &PgPool) -> Result<()> {
    info!("Setting admins...");

    let mut rdr = csv::Reader::from_path("./src/fixtures/admin/admin.csv")?;

    let data = rdr
        .deserialize()
        .map(|result| {
            let record = result?;
            Ok(record)
        })
        .collect::<Result<Vec<AdminsData>>>()?;

    let admins = data
        .iter()
        .map(|admin| admin.email_admin.clone())
        .collect::<Vec<_>>();

    debug!("Admins: {:?}", admins);

    sqlx::query!(
        r#"
        UPDATE "user"
        SET type = 'admin'
        WHERE email = ANY($1)
        "#,
        &admins
    )
    .execute(db)
    .await?;

    info!("Admins set successfully");

    Ok(())
}
