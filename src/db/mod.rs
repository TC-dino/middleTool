pub mod connection;
pub mod sqlite;

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Connection configuration for different database types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionConfig {
    SQLite {
        name: String,
        path: String,
    },
    MySQL {
        name: String,
        host: String,
        port: u16,
        user: String,
        password: String,
        database: String,
    },
    Postgres {
        name: String,
        host: String,
        port: u16,
        user: String,
        password: String,
        database: String,
    },
    Redis {
        name: String,
        host: String,
        port: u16,
        password: Option<String>,
        db: Option<u8>,
    },
}

impl ConnectionConfig {
    pub fn name(&self) -> &str {
        match self {
            ConnectionConfig::SQLite { name, .. } => name,
            ConnectionConfig::MySQL { name, .. } => name,
            ConnectionConfig::Postgres { name, .. } => name,
            ConnectionConfig::Redis { name, .. } => name,
        }
    }

    pub fn db_type(&self) -> &str {
        match self {
            ConnectionConfig::SQLite { .. } => "SQLite",
            ConnectionConfig::MySQL { .. } => "MySQL",
            ConnectionConfig::Postgres { .. } => "PostgreSQL",
            ConnectionConfig::Redis { .. } => "Redis",
        }
    }
}

/// Unified value type for query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "NULL"),
            Value::Bool(v) => write!(f, "{}", v),
            Value::Int(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Text(v) => write!(f, "{}", v),
            Value::Blob(v) => write!(f, "[{} bytes]", v.len()),
        }
    }
}

/// Column information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub type_name: String,
}

/// Unified query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<ColumnInfo>,
    pub rows: Vec<Vec<Value>>,
    pub affected_rows: u64,
    pub execution_time: Duration,
    pub message: Option<String>,
}

impl QueryResult {
    pub fn empty(message: &str) -> Self {
        Self {
            columns: vec![],
            rows: vec![],
            affected_rows: 0,
            execution_time: Duration::ZERO,
            message: Some(message.to_string()),
        }
    }
}

/// Table schema info
#[derive(Debug, Clone)]
pub struct TableSchema {
    pub columns: Vec<ColumnSchema>,
}

#[derive(Debug, Clone)]
pub struct ColumnSchema {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
}

/// Database connection trait
pub trait DatabaseConnection: Send + Sync {
    fn execute<'a>(&'a self, query: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<QueryResult, String>> + Send + 'a>>;
    fn list_tables<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<String>, String>> + Send + 'a>>;
    fn describe_table<'a>(&'a self, table: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<TableSchema, String>> + Send + 'a>>;
    fn close(&self);
}
