use axum_login::AuthUser;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: String,
    pub interactive_done: bool,
    pub section: i32,
    pub class: Option<String>,
    pub admin: bool,
}

impl User {
    pub async fn get_or_create_user_by_email(
        db: &PgPool,
        email: String,
        name: String,
    ) -> Result<Self, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT * FROM "user" WHERE email = $1 
            "#,
            email
        )
        .fetch_optional(db)
        .await?;

        match user {
            Some(user) => Ok(user),
            None => {
                let user = sqlx::query_as!(
                    User,
                    // language=PostgreSQL
                    r#"
                    INSERT INTO "user" (email, name) VALUES ($1, $2) RETURNING *
                    "#,
                    email,
                    name
                )
                .fetch_one(db)
                .await?;

                Ok(user)
            }
        }
    }

    pub async fn get_user_by_id(db: &PgPool, id: &i32) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT *
            FROM "user"
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(db)
        .await?;

        Ok(user)
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.email.as_bytes()
    }
}
