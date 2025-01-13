//! The database ([PostgreSQL](https://postgresql.org/)) configuration.

/// The database ([PostgreSQL](https://postgresql.org/)) configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    /// The host to connect to the database with.
    /// Defaults to `"localhost"`
    pub host: String,

    /// The port the database is on.
    /// Defaults to `5432`
    pub port: u16,

    /// The user to login to the database with.
    /// Defaults to `"modhost"`
    pub user: String,

    /// The database user's password.
    /// Defaults to `"changeme"`
    pub pass: Option<String>,

    /// The database name to use.
    /// Defaults to `"modhost"`
    pub database: String,
}

impl PostgresConfig {
    /// Get the `[user]:[password]` (or just `[user]` if no password) part of the connection URI.
    pub fn user(&self) -> String {
        match &self.pass {
            Some(it) => format!("{}:{}", self.user, it),
            None => self.user.clone(),
        }
    }

    /// Get the full connection URI.
    pub fn uri(&self) -> String {
        format!(
            "postgresql://{}@{}:{}/{}",
            self.user(),
            self.host,
            self.port,
            self.database
        )
    }
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 5432,
            user: "modhost".into(),
            pass: Some("changeme".into()),
            database: "modhost".into(),
        }
    }
}
