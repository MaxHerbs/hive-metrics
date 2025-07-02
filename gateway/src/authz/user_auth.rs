use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_or_create_user(pool: &PgPool, email: &str) -> Result<Uuid, sqlx::Error> {
    if let Some(id) = sqlx::query_scalar!("SELECT id FROM users WHERE email = $1", email)
        .fetch_optional(pool)
        .await?
    {
        return Ok(id);
    }

    let new_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (id, email) VALUES ($1, $2)",
        new_id,
        email
    )
    .execute(pool)
    .await?;

    Ok(new_id)
}

pub async fn user_owns_device(
    pool: &PgPool,
    user_id: Uuid,
    external_device_id: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM user_devices
            JOIN devices ON user_devices.device_id = devices.id
            WHERE user_devices.user_id = $1 AND devices.device_id = $2
        )
        "#,
        user_id,
        external_device_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result.unwrap_or(false))
}
