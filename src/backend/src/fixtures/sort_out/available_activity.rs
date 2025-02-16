use sqlx::PgPool;

/// Returns a list of available ids (repeated as many times as there are
/// available activities) for a given round. Prioritizes activities needing more
/// activities.
pub(super) async fn get_available_activities(
    db: &PgPool,
    round: i32,
) -> color_eyre::Result<Vec<i32>> {
    let available_activities = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"
        WITH slot_counts AS (SELECT rm.activity_id,
                                    GREATEST(rm.max_users - COUNT(au.user_id), 0) AS available_slots
                             FROM round_max_users rm
                                      LEFT JOIN activity_user au
                                                ON rm.activity_id = au.activity_id
                                                    AND rm.round = au.round
                                      JOIN activity a
                                           ON rm.activity_id = a.id
                                               AND a.should_display
                             WHERE rm.round = $1 -- Input round parameter
                             GROUP BY rm.activity_id,
                                      rm.max_users)
        SELECT activity_id -- Only return activity_id
        FROM slot_counts
                 CROSS JOIN LATERAL GENERATE_SERIES(1, available_slots)
        ORDER BY available_slots DESC, -- Prioritize activities needing more slots
                 activity_id;
        "#,
        round
    )
    .fetch_all(db)
    .await?;

    Ok(available_activities)
}
