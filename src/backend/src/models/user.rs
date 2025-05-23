use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: String,
    pub section: Option<String>,
    pub class: i32,
    pub r#type: UserType,
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type, ToSchema, Eq, Hash, PartialEq, Copy)]
#[sqlx(type_name = "user_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    Normal = 0,
    Host = 1,
    Admin = 2,
}

impl User {
    pub async fn get_or_create_user_by_email(
        db: &PgPool,
        email: String,
        name: String,
    ) -> Result<Self, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email, section, class, type AS "type!: UserType" FROM "user" WHERE email = $1
            "#,
            email
        )
        .fetch_optional(db)
        .await?;

        match user {
            Some(mut user) => {
                if user.name.is_none() {
                    user = sqlx::query_as!(
                            User,
                            r#"
                            UPDATE "user" SET name = $1 WHERE email = $2 RETURNING id, name, email, section, class, type AS "type!: UserType"
                            "#,
                            name,
                            email
                    )
                        .fetch_one(db)
                        .await?;
                }

                Ok(user)
            }
            None => {
                let user = sqlx::query_as!(
                    User,
                    r#"
                    INSERT INTO "user" (email, name) VALUES ($1, $2) RETURNING  id, name, email, section, class, type AS "type!: UserType"
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
            r#"
            SELECT id, name, email, section, class, type AS "type!: UserType"
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
