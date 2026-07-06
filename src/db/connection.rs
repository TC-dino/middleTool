use super::{ConnectionConfig, DatabaseConnection};
use super::sqlite::SqliteConnection;
use std::sync::Arc;

/// Wrapper for database connection with Debug support
pub struct DbConnection {
    inner: Arc<dyn DatabaseConnection>,
    db_type: String,
}

impl std::fmt::Debug for DbConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbConnection")
            .field("db_type", &self.db_type)
            .finish()
    }
}

impl Clone for DbConnection {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            db_type: self.db_type.clone(),
        }
    }
}

impl DbConnection {
    pub fn new(conn: Arc<dyn DatabaseConnection>, db_type: &str) -> Self {
        Self {
            inner: conn,
            db_type: db_type.to_string(),
        }
    }

    pub async fn execute(&self, query: &str) -> Result<super::QueryResult, String> {
        self.inner.execute(query).await
    }

    pub async fn list_tables(&self) -> Result<Vec<String>, String> {
        self.inner.list_tables().await
    }

    pub async fn describe_table(&self, table: &str) -> Result<super::TableSchema, String> {
        self.inner.describe_table(table).await
    }

    pub fn close(&self) {
        self.inner.close();
    }
}

/// Connect to a database based on configuration
pub async fn connect(config: &ConnectionConfig) -> Result<DbConnection, String> {
    match config {
        ConnectionConfig::SQLite { path, .. } => {
            let conn = SqliteConnection::connect(path).await?;
            Ok(DbConnection::new(Arc::new(conn), "SQLite"))
        }
        ConnectionConfig::MySQL { .. } => {
            Err("MySQL support not yet implemented".to_string())
        }
        ConnectionConfig::Postgres { .. } => {
            Err("PostgreSQL support not yet implemented".to_string())
        }
        ConnectionConfig::Redis { .. } => {
            Err("Redis support not yet implemented".to_string())
        }
    }
}
