use ahash::AHashMap;
use axum::response::IntoResponse;
use axum_serde::Sonic;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    app::openapi::ACTIVITIES_TAG,
    models::user::ForumUserRole,
    users::AuthSession,
    web::schemas::activity::{Activity, ActivityWithoutDate},
};

#[derive(Serialize, ToSchema)]
pub struct SelectedActivityResponse {
    registrations_end_date: DateTime<Utc>,
    activities: Vec<Activity>,
}

#[utoipa::path(
    get,
    path = "/selected",
    summary = "Selected Activities",
    responses(
        (status = OK, description = "Returns the selected activities", body = SelectedActivityResponse),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
        (status = FORBIDDEN, description = "You are an admin"),
        (status = 425, description = "Registrations have not started yet"),
        (status = GONE, description = "Registrations have ended"),
    ),
    security(
        ("session" = [])
    ),
    tag = ACTIVITIES_TAG,
)]
pub async fn selected(auth_session: AuthSession) -> impl IntoResponse {
    let user = match auth_session.user {
        None => return StatusCode::UNAUTHORIZED.into_response(),
        Some(user)
            if user.forum_role == ForumUserRole::Admin
                || user.forum_role == ForumUserRole::Host =>
        {
            return StatusCode::FORBIDDEN.into_response();
        }
        Some(user) => user,
    };

    let activities = match sqlx::query_as!(
        ActivityWithoutDate,
        r#"
        SELECT forum_activity.id                            AS id,
               forum_activity_user.round                    AS round,
               forum_activity.name                          AS name,
               forum_activity.description                   AS description,
               forum_activity.room                          AS "room!: String",
               (SELECT COUNT(*)
                FROM forum_activity_user au
                WHERE au.activity_id = forum_activity.id
                  AND au.round = forum_activity_user.round) AS "used_seats!: i64",
               forum_round_max_users.max_users              AS total_seats,
               forum_activity_user.joined_at IS NOT NULL    AS "present!: bool"
        FROM forum_activity_user
                 JOIN forum_activity ON forum_activity_user.activity_id = forum_activity.id
                 JOIN forum_round_max_users
                      ON forum_activity.id = forum_round_max_users.activity_id
                          AND forum_activity_user.round = forum_round_max_users.round
        WHERE forum_activity_user.user_id = $1
        GROUP BY forum_activity.id, forum_activity_user.round, forum_activity.name, forum_activity.description, forum_activity.room,
                 forum_round_max_users.max_users,
                 forum_activity_user.joined_at
        ORDER BY forum_activity_user.round;
        "#,
        user.id
    )
        .fetch_all(&auth_session.backend.db)
        .await
    {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let num_rounds = auth_session.backend.config.get_num_rounds();

    let activities = fill_gaps(activities, num_rounds);

    let activities: Result<Vec<_>, _> = activities
        .into_iter()
        .map(|activity| Activity::from_without_date(activity, &auth_session.backend.config))
        .collect();

    let activities = match activities {
        Ok(a) => a,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let registrations_end_date = auth_session.backend.config.registrations_end_date;

    Sonic(SelectedActivityResponse {
        activities,
        registrations_end_date,
    })
    .into_response()
}

fn fill_gaps(activities: Vec<ActivityWithoutDate>, num_rounds: usize) -> Vec<ActivityWithoutDate> {
    let round_map: AHashMap<i32, &ActivityWithoutDate> = activities
        .iter()
        .map(|activity| (activity.round, activity))
        .collect();

    (0..num_rounds as i32)
        .map(|round| {
            if let Some(existing) = round_map.get(&round) {
                ActivityWithoutDate {
                    id: existing.id,
                    round: existing.round,
                    name: existing.name.clone(),
                    description: existing.description.clone(),
                    room: existing.room.clone(),
                    used_seats: existing.used_seats,
                    total_seats: existing.total_seats,
                    present: existing.present,
                }
            } else {
                ActivityWithoutDate {
                    round,
                    ..Default::default()
                }
            }
        })
        .collect()
}
