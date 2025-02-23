use chrono::{DateTime, Utc};
use chrono_tz::Europe::Rome;
use color_eyre::Result;
use rust_xlsxwriter::{Format, FormatBorder, Workbook};
use sqlx::PgPool;
use tracing::info;

use crate::{app::config::Config, web::schemas::activity::round_to_date};

const ROUND_NUMBER: i32 = 4;

struct PresencesColumns {
    name: String,
    class: i32,
    section: Option<String>,
    activity: Option<String>,
    scheduled_at: Option<DateTime<Utc>>,
    joined_at: Option<DateTime<Utc>>,
    joined_at_edited_by: Option<String>,
    randomized: Option<bool>,
}

pub async fn export_presences(db: &PgPool, config: &Config) -> color_eyre::Result<()> {
    let mut workbook = Workbook::new();

    let format_header = Format::new().set_bold().set_border(FormatBorder::Medium);
    let format_users = Format::new().set_border(FormatBorder::Thin);

    let user_infos_futs = (0..ROUND_NUMBER)
        .map(|round| query_users_info(db, round))
        .collect::<Vec<_>>();

    let user_infos = futures::future::try_join_all(user_infos_futs).await?;

    for (round, users_info) in user_infos.into_iter().enumerate() {
        let worksheet_name = round_to_date(config, round as i32)?
            .with_timezone(&Rome)
            // Like this hh:mm dd/MM/yyyy
            .format("%H.%M %d-%m-%Y")
            .to_string();

        let worksheet = workbook.add_worksheet().set_name(&worksheet_name)?;

        write_worksheet(worksheet, users_info, &format_header, &format_users)?;
    }

    workbook.save("./Presenze.xlsx")?;

    info!("Presences exported to ./Presenze.xlsx");

    Ok(())
}

async fn query_users_info(db: &PgPool, round: i32) -> Result<Vec<PresencesColumns>> {
    let presences = sqlx::query_as!(
        PresencesColumns,
        r#"
        SELECT u.name          AS "name!: String",
               u.class         AS class,
               u.section       AS section,
               a.name          AS "activity!: Option<String>",
               au.scheduled_at AS "scheduled_at!: Option<DateTime<Utc>>",
               au.joined_at    AS "joined_at!: Option<DateTime<Utc>>",
               editor.name     AS joined_at_edited_by,
               au.randomized   AS "randomized!: Option<bool>"
        FROM "user" u
                 LEFT JOIN activity_user au
                           ON u.id = au.user_id
                               AND au.round = $1
                 LEFT JOIN activity a
                           ON au.activity_id = a.id
                 LEFT JOIN "user" editor
                           ON au.joined_at_last_edited_by = editor.id
        ORDER BY u.name;
        "#,
        round
    )
    .fetch_all(db)
    .await?;

    Ok(presences)
}

fn write_worksheet(
    worksheet: &mut rust_xlsxwriter::Worksheet,
    users: Vec<PresencesColumns>,
    format_header: &Format,
    format_users: &Format,
) -> Result<(), rust_xlsxwriter::XlsxError> {
    // Write headers (all PresencesColumns fields)
    worksheet.write_with_format(0, 0, "Nome", format_header)?;
    worksheet.write_with_format(0, 1, "Classe", format_header)?;
    worksheet.write_with_format(0, 2, "Sezione", format_header)?;
    worksheet.write_with_format(0, 3, "Attività", format_header)?;
    worksheet.write_with_format(0, 4, "Pianificata", format_header)?;
    worksheet.write_with_format(0, 5, "Entrata", format_header)?;
    worksheet.write_with_format(0, 6, "Entrata modificata da", format_header)?;
    worksheet.write_with_format(0, 7, "Casuale?", format_header)?;

    // Write user data
    for (row_idx, user) in users.into_iter().enumerate() {
        let row = (row_idx + 1) as u32;

        let scheduled_at = user
            .scheduled_at
            .map(|dt| dt.with_timezone(&Rome).to_string());

        let joined_at = user.joined_at.map(|dt| dt.with_timezone(&Rome).to_string());

        worksheet.write_with_format(row, 0, user.name, format_users)?;
        worksheet.write_with_format(row, 1, user.class, format_users)?;
        worksheet.write_with_format(row, 2, user.section, format_users)?;
        worksheet.write_with_format(row, 3, user.activity, format_users)?;
        worksheet.write_with_format(row, 4, scheduled_at, format_users)?;
        worksheet.write_with_format(row, 5, joined_at, format_users)?;
        worksheet.write_with_format(row, 6, user.joined_at_edited_by, format_users)?;
        worksheet.write_with_format(row, 7, user.randomized, format_users)?;
    }

    Ok(())
}
