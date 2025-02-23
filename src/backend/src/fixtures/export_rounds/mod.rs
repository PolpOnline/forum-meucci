use chrono_tz::Europe::Rome;
use rust_xlsxwriter::{Format, FormatBorder, Workbook, Worksheet, XlsxError};
use sqlx::PgPool;

use crate::{app::config::Config, web::schemas::activity::round_to_date};

const ROUND_NUMBER: i32 = 4;

struct ActivityExcelUserInfo {
    name: String,
    section: Option<String>,
    class: i32,
    randomized: bool,
}

pub async fn export_rounds(db: &PgPool, config: &Config) -> color_eyre::Result<()> {
    let mut workbook = Workbook::new();

    let format_header = Format::new().set_bold().set_border(FormatBorder::Medium);
    let format_users = Format::new().set_border(FormatBorder::Thin);

    // Query all the activities
    let activities = sqlx::query!(
        // language=PostgreSQL
        r#"
        SELECT id, name, description, room FROM activity WHERE should_display = TRUE ORDER BY id
        "#,
    )
    .fetch_all(db)
    .await?;

    for activity in activities.iter() {
        'round_loop: for round in 0..ROUND_NUMBER {
            let users = sqlx::query_as!(
                ActivityExcelUserInfo,
                // language=PostgreSQL
                r#"
                SELECT "user".section,
                       "user".class,
                       COALESCE("user".name, "user".email) AS "name!: String",
                       activity_user.randomized
                FROM activity_user
                         JOIN "user" ON activity_user.user_id = "user".id
                WHERE activity_user.activity_id = $1
                  AND activity_user.round = $2
                ORDER BY name;
                "#,
                activity.id,
                round
            )
            .fetch_all(db)
            .await?;

            if users.is_empty() {
                continue 'round_loop;
            }

            let base_name = format!("{}_T{}", activity.name, round + 1);
            let sanitized: String = base_name
                .chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() || c == ' ' {
                        c
                    } else {
                        '_'
                    }
                })
                .collect();
            let sanitized = sanitized.replace(' ', "_");
            let sanitized = sanitized.trim_matches('_');
            let sheet_name = truncate_sheet_name(sanitized, 31);

            let worksheet = workbook.add_worksheet().set_name(&sheet_name)?;

            // Write activity info
            worksheet.write(0, 0, activity.name.clone())?;
            worksheet.write(0, 1, activity.room.clone())?;
            let round_date = round_to_date(config, round)?;
            worksheet.write(0, 2, round_date.with_timezone(&Rome).to_string())?;
            worksheet.write(0, 3, activity.description.clone())?;

            write_worksheet(worksheet, users, &format_header, &format_users)?;
        }
    }

    workbook.save("./AttivitàTurni.xlsx")?;

    Ok(())
}

fn truncate_sheet_name(name: &str, max_length: usize) -> String {
    let mut result = name.chars().take(max_length).collect::<String>();
    if name.len() > max_length {
        result.truncate(max_length - 3);
        result.push_str("...");
    }
    result
}

fn write_worksheet(
    worksheet: &mut Worksheet,
    users: Vec<ActivityExcelUserInfo>,
    format_header: &Format,
    format_users: &Format,
) -> Result<(), XlsxError> {
    // Write headers
    worksheet.write_with_format(2, 0, "Name", format_header)?;
    worksheet.write_with_format(2, 1, "Classe", format_header)?;
    worksheet.write_with_format(2, 2, "Casuale?", format_header)?;

    // Write user data
    for (row_idx, user) in users.into_iter().enumerate() {
        let row = (row_idx + 3) as u32;
        let class_and_section =
            format!("{} {}", user.class, user.section.unwrap_or("".to_string()));

        worksheet.write_with_format(row, 0, user.name, format_users)?;
        worksheet.write_with_format(row, 1, class_and_section, format_users)?;
        worksheet.write_with_format(row, 2, user.randomized, format_users)?;
    }

    Ok(())
}
