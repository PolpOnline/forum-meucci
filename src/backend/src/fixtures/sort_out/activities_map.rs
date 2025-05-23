use ahash::AHashMap;
use sqlx::PgPool;

/// Returns a map of activity ids to activity names.
pub(super) async fn get_activities_map(db: &PgPool) -> color_eyre::Result<AHashMap<i32, String>> {
    let activities_map = sqlx::query!(
        r#"
        SELECT
            id,
            name
        FROM
            activity
        "#,
    )
    .fetch_all(db)
    .await?;

    let activities_map = activities_map
        .iter()
        .map(|activity| (activity.id, activity.name.clone()))
        .collect::<AHashMap<_, _>>();

    Ok(activities_map)
}
