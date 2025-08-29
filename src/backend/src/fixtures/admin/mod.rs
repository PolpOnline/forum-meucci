use color_eyre::Result;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{debug, info};

#[derive(Debug, Deserialize)]
struct AdminsData {
    email_admin: String,
}

pub async fn seed(db: &PgPool, write: bool) -> Result<()> {
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

    let mut txn = db.begin().await?;

    sqlx::query!(
        r#"
        UPDATE "user"
        SET forum_role = 'admin'
        WHERE email = ANY($1)
        "#,
        &admins
    )
    .execute(&mut *txn)
    .await?;

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
