mod activities_map;
mod available_activity;
mod unselected_users;

use ahash::AHashMap;
use color_eyre::owo_colors::OwoColorize;
use indicatif::ProgressStyle;
use sqlx::{PgPool, Postgres, Transaction};
use tracing::{debug, info, info_span, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt;

use crate::fixtures::sort_out::{
    activities_map::get_activities_map,
    available_activity::get_available_activities,
    unselected_users::{get_unselected_users, UnselectedUser},
};

const ROUND_NUMBER: i32 = 4;

struct RoundData {
    available_activity_ids: Vec<i32>,
    unselected_users: Vec<UnselectedUser>,
    round: i32,
}

pub async fn sort_out_users(db: &PgPool) -> color_eyre::Result<()> {
    info!("Sorting out users...");

    let activities_map = get_activities_map(&db).await?;

    // Get the available activities and unselected users for each round
    let round_data = (0..ROUND_NUMBER)
        .map(|round| get_round_data(db, round))
        .collect::<Vec<_>>();

    // Wait for all the futures to complete
    let round_data = futures::future::try_join_all(round_data).await?;

    for round_data in &round_data {
        info!(
            "Round {} has {} available activities and {} unselected users",
            round_data.round.green(),
            round_data.available_activity_ids.len().blue(),
            round_data.unselected_users.len().red()
        );
    }

    // Start a transaction to insert the users
    let mut txn = db.begin().await?;

    let mut num_inserts = 0;

    // Sort out the users for each round
    for round_data in round_data {
        info!("Sorting out users for round {}", round_data.round);
        num_inserts += sort_out_users_round(
            &mut txn,
            &activities_map,
            round_data.available_activity_ids,
            round_data.unselected_users,
            round_data.round,
        )
        .await?;
    }

    info!("Committing transaction ({} inserts)...", num_inserts);

    txn.commit().await?;

    info!("Users sorted out successfully");

    Ok(())
}

/// Returns the available activities and unselected users for a given round.
async fn get_round_data(db: &PgPool, round: i32) -> color_eyre::Result<RoundData> {
    let available_activity_ids = get_available_activities(db, round).await?;
    let unselected_users = get_unselected_users(db, round).await?;

    Ok(RoundData {
        available_activity_ids,
        unselected_users,
        round,
    })
}

/// Assigns users to activities for a given round.
/// Returns the number of expected inserts.
async fn sort_out_users_round(
    txn: &mut Transaction<'_, Postgres>,
    activities_map: &AHashMap<i32, String>,
    available_activity_ids: Vec<i32>,
    unselected_users: Vec<UnselectedUser>,
    round: i32,
) -> color_eyre::Result<usize> {
    let users_activities_iter = unselected_users.iter().zip(available_activity_ids.iter());

    let num_inserts = users_activities_iter.len();

    let header_span = info_span!("header");
    header_span.pb_set_length(users_activities_iter.len() as u64);
    header_span.pb_set_style(&ProgressStyle::default_bar());

    let header_span_enter = header_span.enter();

    for (unselected_user, available_activity_id) in users_activities_iter {
        Span::current().pb_inc(1);

        debug!(
            "Assigning user {} ({}) to activity {} ({}) round {}",
            unselected_user.user_id,
            unselected_user.user_email.red(),
            available_activity_id,
            activities_map[available_activity_id].blue(),
            round.green()
        );

        sqlx::query!(
            r#"
            INSERT INTO activity_user (activity_id, user_id, round, randomized)
            VALUES ($1, $2, $3, TRUE)
            "#,
            available_activity_id,
            unselected_user.user_id,
            round
        )
        .execute(&mut **txn)
        .await?;
    }

    drop(header_span_enter);
    drop(header_span);

    Ok(num_inserts)
}
