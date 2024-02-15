use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
use async_session::{Session,Result,chrono::Utc, SessionStore};
use spin_sdk::sqlite::{Connection, Value::{Integer,Text, Null}};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Clone)]
pub struct SqliteStore{
    connection: Arc<Connection>,
    table_name: String,
}
#[derive(Debug, Clone)]
pub struct SessionRow{
    id: String,
    expiry: Option<i64>,
    session: String,
}
impl SqliteStore {
    pub fn from_connection(con: Arc<Connection>) -> Self {
    Self{
    connection: con,
    table_name: "async_sessions".to_string(),
    }
    }

    pub fn from_connection_with_table_name(con: Arc<Connection>, table_name: impl AsRef<str>) -> Self {
    Self{
    connection: con,
    table_name: table_name.as_ref().to_string(),
    }
    }

    pub fn with_table_name(mut self, table_name: impl AsRef<str>)-> Self{
        let table_name = table_name.as_ref();
        if table_name.is_empty()
            || !table_name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            panic!(
                "table name must be [a-zA-Z0-9_-]+, but {} was not",
                table_name
            );
        }

        self.table_name = table_name.to_owned();
        self
    }
    pub async fn migrate(&self) -> Result<()> {
        info!("migrating sessions on `{}`", self.table_name);

        let _ = &self.connection.execute(&self.substitute_table_name(
            r#"
            CREATE TABLE IF NOT EXISTS %%TABLE_NAME%% (
                id TEXT PRIMARY KEY NOT NULL,
                expiry INTEGER NULL,
                session TEXT NOT NULL
            )
            "#,
        ), &[])?;
        Ok(())
    }
    // private utility function because sqlite does not support
    // parametrized table names
    fn substitute_table_name(&self, query: &str) -> String {
        query.replace("%%TABLE_NAME%%", &self.table_name)
    }


 /// Performs a one-time cleanup task that clears out stale
    /// (expired) sessions. You may want to call this from cron.
    pub async fn cleanup(&self) -> Result<()> {
        let _ = self.connection.execute(&self.substitute_table_name(
            r#"
            DELETE FROM %%TABLE_NAME%%
            WHERE expiry < ?
            "#
        ),&[Integer(Utc::now().timestamp())])?;

        Ok(())
    }

    /// retrieves the number of sessions currently stored, including
    /// expired sessions

    pub async fn count(&self) -> Result<i32> {
        let rowset =
            &self.connection.execute(&self.substitute_table_name("SELECT COUNT(*) FROM %%TABLE_NAME%%"),&[])?;
            let count = rowset.rows().nth(0).map(|row| row.get::<i32>("COUNT(*)").unwrap()).unwrap();

        Ok(count)
    }
}
#[async_trait]
impl SessionStore for SqliteStore{
 async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        let id = Session::id_from_cookie_value(&cookie_value)?;
        let rowset = &self.connection.execute(&self.substitute_table_name(
            r#"
            SELECT * FROM %%TABLE_NAME%%
              WHERE id = ? AND (expiry IS NULL OR expiry > ?)
            "#,
        ), &[Text(id.to_string()), Integer(Utc::now().timestamp())])?;
        let session_row = rowset.rows().nth(0).map(|row| SessionRow{
            id: row.get::<&str>("id").unwrap().to_string(),
        expiry: row.get::<i64>("expiry"),
        session: row.get::<&str>("session").unwrap().to_string(),
        });
        let session: Option<String> = match session_row{
        Some(s) => Some(s.session),
        None => None
        };

        Ok(session
            .map(|session| serde_json::from_str(&session))
            .transpose()?)
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        let id = session.id();
        let string = serde_json::to_string(&session)?;
        let expiry = match session.expiry().map(|expiry| expiry.timestamp()) {
        Some(e) => Integer(e),
        None => Null
        };
        let _ = &self.connection.execute(&self.substitute_table_name(
            r#"
            INSERT INTO %%TABLE_NAME%%
              (id, session, expiry) VALUES (?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
              expiry = excluded.expiry,
              session = excluded.session
            "#,
        ),&[Text((&id).to_string()),Text(string),expiry ])?;

        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result {
        let id = session.id();
        println!("ID: {id}");
        let _ = &self.connection.execute(&self.substitute_table_name(
            r#"
            DELETE FROM %%TABLE_NAME%% WHERE id = ?
            "#,
        ), &[Text(id.to_string())])?;

        Ok(())
    }

    async fn clear_store(&self) -> Result {
        let _ = &self.connection.execute(&self.substitute_table_name(
            r#"
            DELETE FROM %%TABLE_NAME%%
            "#,
        ), &[])?;

        Ok(())
    }
}
}}
