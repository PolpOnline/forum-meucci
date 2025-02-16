use sqlx::PgPool;

pub(super) struct UnselectedUser {
    pub(super) user_id: i32,
    pub(super) user_email: String,
}

/// Returns a list of users that have not been chosen for a given round.
pub(super) async fn get_unselected_users(
    db: &PgPool,
    round: i32,
) -> color_eyre::Result<Vec<UnselectedUser>> {
    let unselected_users = sqlx::query_as!(
        UnselectedUser,
        r#"
        SELECT u.id    AS user_id,
               u.email AS user_email
        FROM "user" u
        WHERE NOT EXISTS (SELECT 1
                          FROM activity_user au
                          WHERE au.user_id = u.id
                            AND au.round = $1 -- Filter for the input round
        );
        "#,
        round
    )
    .fetch_all(db)
    .await?;

    Ok(unselected_users)
}
