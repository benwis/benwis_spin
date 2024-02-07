use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub password: String,
    pub created_at: i64,
    pub created_at_pretty: String,
    pub updated_at: i64,
    pub updated_at_pretty: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: -1,
            username: "Guest".into(),
            created_at: 0,
            created_at_pretty: "".to_string(),
            updated_at: 0,
            updated_at_pretty: "".to_string(),
            password: "".into(),
            display_name: "Guest".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeUser {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub created_at: i64,
    pub created_at_pretty: String,
}
impl Default for SafeUser {
    fn default() -> Self {
        Self {
            id: -1,
            username: "Guest".into(),
            created_at: 0,
            created_at_pretty: "".to_string(),
            display_name: "Guest".into(),
        }
    }
}

impl From<User> for SafeUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
            created_at: user.created_at,
            created_at_pretty: user.created_at_pretty,
        }
    }
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use chrono::naive::NaiveDateTime;


#[derive(Debug, Clone)]
pub struct SqlUser {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub password: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl SqlUser {
    #[tracing::instrument(level = "info", fields(error))]
    pub fn into_user(self) -> User {
        User {
            id: self.id,
            username: self.username,
            display_name: self.display_name,
            password: self.password,
            created_at: self.created_at,
            created_at_pretty: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default().to_string(),
            updated_at: self.updated_at,
            updated_at_pretty: NaiveDateTime::from_timestamp_opt(self.created_at, 0).unwrap_or_default().to_string(),
        }
    }
}

    impl User {
        #[tracing::instrument(level = "info", fields(error))]
        pub async fn get(id: i64, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await
                .ok()?;

            Some(sqluser.into_user())
        }

        #[tracing::instrument(level = "info", fields(error))]
        pub async fn get_from_username(name: String, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE username = ?")
                .bind(name)
                .fetch_one(pool)
                .await
                .ok()?;

            Some(sqluser.into_user())
        }
    }
    impl SafeUser {
        #[tracing::instrument(level = "info", fields(error))]
        pub async fn get(id: i64, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await
                .ok()?;

            Some(sqluser.into_user().into())
        }

        pub async fn get_from_username(name: String, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE username = ?")
                .bind(name)
                .fetch_one(pool)
                .await
                .ok()?;

            Some(sqluser.into_user().into())
        }
    }
}
}
