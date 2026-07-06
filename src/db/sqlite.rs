use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{Column, Row, TypeInfo};
use std::time::Instant;

use super::{ColumnInfo, ColumnSchema, DatabaseConnection, QueryResult, TableSchema, Value};

pub struct SqliteConnection {
    pool: SqlitePool,
}

impl SqliteConnection {
    pub async fn connect(path: &str) -> Result<Self, String> {
        let url = if path == ":memory:" {
            "sqlite::memory:".to_string()
        } else {
            format!("sqlite:{}", path)
        };

        let pool = SqlitePool::connect(&url)
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;

        Ok(Self { pool })
    }
}

fn convert_value(row: &SqliteRow, index: usize) -> Value {
    // Try to get value by type - SQLite stores types per-value
    if let Ok(v) = row.try_get::<bool, _>(index) {
        return Value::Bool(v);
    }
    if let Ok(v) = row.try_get::<i64, _>(index) {
        return Value::Int(v);
    }
    if let Ok(v) = row.try_get::<f64, _>(index) {
        return Value::Float(v);
    }
    if let Ok(v) = row.try_get::<String, _>(index) {
        return Value::Text(v);
    }
    if let Ok(v) = row.try_get::<Vec<u8>, _>(index) {
        return Value::Blob(v);
    }
    Value::Null
}

impl DatabaseConnection for SqliteConnection {
    fn execute<'a>(&'a self, query: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<QueryResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let start = Instant::now();
            let trimmed = query.trim();

            // Check if it's a SELECT-like query
            let upper = trimmed.to_uppercase();
            let is_query = upper.starts_with("SELECT")
                || upper.starts_with("PRAGMA")
                || upper.starts_with("EXPLAIN")
                || upper.starts_with("SHOW");

            if is_query {
                let rows = sqlx::query(trimmed)
                    .fetch_all(&self.pool)
                    .await
                    .map_err(|e| format!("Query error: {}", e))?;

                let columns = if let Some(first) = rows.first() {
                    (0..first.columns().len())
                        .map(|i| {
                            let col = &first.columns()[i];
                            ColumnInfo {
                                name: col.name().to_string(),
                                type_name: col.type_info().name().to_string(),
                            }
                        })
                        .collect()
                } else {
                    vec![]
                };

                let result_rows: Vec<Vec<Value>> = rows
                    .iter()
                    .map(|row| {
                        (0..row.columns().len())
                            .map(|i| convert_value(row, i))
                            .collect()
                    })
                    .collect();

                let row_count = result_rows.len();

                Ok(QueryResult {
                    columns,
                    rows: result_rows,
                    affected_rows: row_count as u64,
                    execution_time: start.elapsed(),
                    message: Some(format!("{} rows returned", row_count)),
                })
            } else {
                let result = sqlx::query(trimmed)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| format!("Execute error: {}", e))?;

                let affected = result.rows_affected();

                Ok(QueryResult {
                    columns: vec![],
                    rows: vec![],
                    affected_rows: affected,
                    execution_time: start.elapsed(),
                    message: Some(format!("{} rows affected", affected)),
                })
            }
        })
    }

    fn list_tables<'a>(&'a self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<String>, String>> + Send + 'a>> {
        Box::pin(async move {
            let rows = sqlx::query_as::<_, (String,)>(
                "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name"
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to list tables: {}", e))?;

            Ok(rows.into_iter().map(|(name,)| name).collect())
        })
    }

    fn describe_table<'a>(&'a self, table: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<TableSchema, String>> + Send + 'a>> {
        Box::pin(async move {
            let query = format!("PRAGMA table_info('{}')", table.replace('\'', "''"));
            let rows = sqlx::query(&query)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| format!("Failed to describe table: {}", e))?;

            let columns = rows
                .iter()
                .map(|row| {
                    let name: String = row.try_get("name").unwrap_or_default();
                    let data_type: String = row.try_get("type").unwrap_or_default();
                    let not_null: bool = row.try_get("notnull").unwrap_or(false);
                    let pk: bool = row.try_get("pk").unwrap_or(false);

                    ColumnSchema {
                        name,
                        data_type,
                        nullable: !not_null,
                        primary_key: pk,
                    }
                })
                .collect();

            Ok(TableSchema { columns })
        })
    }

    fn close(&self) {
        let pool = self.pool.clone();
        tokio::spawn(async move {
            pool.close().await;
        });
    }
}
