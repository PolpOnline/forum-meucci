use ahash::AHashMap;
use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    app::openapi::ACTIVITIES_TAG,
    models::user::UserType,
    users::AuthSession,
    web::schemas::activity::{Activity, ActivityWithoutDate},
};

#[derive(Serialize, ToSchema)]
pub struct SelectedActivityResponse {
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
        (status = 425, description = "Bookings have not started yet"),
        (status = GONE, description = "Bookings have ended"),
    ),
    security(
        ("session" = [])
    ),
    tag = ACTIVITIES_TAG,
)]
pub async fn selected(auth_session: AuthSession) -> impl IntoResponse {
    let user = match auth_session.user {
        None => return StatusCode::UNAUTHORIZED.into_response(),
        Some(user) if user.r#type == UserType::Admin || user.r#type == UserType::Host => {
            return StatusCode::FORBIDDEN.into_response()
        }
        Some(user) => user,
    };

    let activities = match sqlx::query_as!(
        ActivityWithoutDate,
        r#"
        SELECT activity.id                         AS id,
               activity_user.round                 AS round,
               activity.name                       AS name,
               activity.description                AS description,
               activity.room                       AS "room!: String",
               COUNT(activity_user.user_id)        AS "used_seats!: i64",
               round_max_users.max_users           AS total_seats,
               activity_user.joined_at IS NOT NULL AS "present!: bool"
        FROM activity_user
                 JOIN activity ON activity_user.activity_id = activity.id
                 JOIN round_max_users
                      ON activity.id = round_max_users.activity_id AND activity_user.round = round_max_users.round
        WHERE activity_user.user_id = $1
        GROUP BY activity.id, activity_user.round, activity.name, activity.description, activity.room,
                 round_max_users.max_users,
                 activity_user.joined_at
        ORDER BY activity_user.round;
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

    Json(SelectedActivityResponse { activities }).into_response()
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
