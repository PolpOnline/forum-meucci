use axum::{extract::Path, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    app::{config::Config, openapi::ADMIN_TAG},
    models::user::UserType,
    users::AuthSession,
    web::{
        endpoints::protected::admin::shared::get_activity,
        schemas::activity::{round_to_date, RoundConversionError},
    },
};

#[derive(Deserialize, IntoParams)]
pub struct AdminRoundRequest {
    /// The ID of the activity
    #[param(example = 1)]
    activity_id: i32,
}

#[derive(Serialize, ToSchema)]
pub struct AdminRoundResponse {
    #[schema(example = "Activity 1")]
    name: String,
    #[schema(example = "This is the description of activity 1")]
    description: String,
    #[schema(example = "Room 1")]
    room: String,
    rounds: Vec<AdminRound>,
}

#[derive(Serialize, ToSchema)]
pub struct AdminRound {
    #[schema(example = 0, minimum = 0)]
    round: i32,
    date: DateTime<Utc>,
    #[schema(example = 10)]
    used_seats: Option<i64>,
    #[schema(example = 20)]
    total_seats: i64,
}

impl AdminRound {
    fn from_without_date(
        activity: AdminRoundWithoutDate,
        config: &Config,
    ) -> Result<Self, RoundConversionError> {
        Ok(AdminRound {
            round: activity.round,
            date: round_to_date(config, activity.round)?,
            used_seats: activity.used_seats,
            total_seats: activity.total_seats,
        })
    }
}

pub struct AdminRoundWithoutDate {
    round: i32,
    used_seats: Option<i64>,
    total_seats: i64,
}

#[utoipa::path(
    get,
    path = "/rounds/{activity_id}",
    summary = "Rounds List",
    params(AdminRoundRequest),
    responses(
        (status = OK, description = "List of the rounds for an activity the user has access to", body = AdminRoundResponse),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = FORBIDDEN, description = "Not an admin or host"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
        (status = 425, description = "Registrations have not started yet"),
    ),
    security(
        ("session" = [])
    ),
    tag = ADMIN_TAG,
)]
pub async fn rounds(
    auth_session: AuthSession,
    Path(req): Path<AdminRoundRequest>,
) -> impl IntoResponse {
    let (user_type, user_id) = match auth_session.user {
        Some(ref user) => (user.r#type, user.id),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if user_type == UserType::Normal {
        return StatusCode::FORBIDDEN.into_response();
    }

    let activity = get_activity(&auth_session, user_type, user_id, req.activity_id).await;

    let activity = match activity {
        Ok(Some(r)) => r,
        // User does not have access to the activity
        Ok(None) => return StatusCode::FORBIDDEN.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let rounds = match sqlx::query_as!(
        AdminRoundWithoutDate,
        r#"
        SELECT round_max_users.round,
               COUNT(activity_user.user_id) AS used_seats,
               round_max_users.max_users    AS total_seats
        FROM round_max_users
                 LEFT JOIN activity_user
                           ON round_max_users.activity_id = activity_user.activity_id
                               AND round_max_users.round = activity_user.round
        WHERE round_max_users.activity_id = $1
          AND round_max_users.max_users > 0
        GROUP BY round_max_users.round, round_max_users.max_users
        ORDER BY round_max_users.round
        "#,
        req.activity_id
    )
    .fetch_all(&auth_session.backend.db)
    .await
    {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let rounds: Result<Vec<_>, _> = rounds
        .into_iter()
        .map(|activity| AdminRound::from_without_date(activity, &auth_session.backend.config))
        .collect();

    let rounds = match rounds {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(AdminRoundResponse {
        name: activity.name,
        description: activity.description,
        room: activity.room,
        rounds,
    })
    .into_response()
}
