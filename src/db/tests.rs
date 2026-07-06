#[cfg(test)]
mod tests {
    use crate::db::sqlite::SqliteConnection;
    use crate::db::DatabaseConnection;

    #[tokio::test]
    async fn test_sqlite_connection() {
        // Test in-memory database connection
        let conn = SqliteConnection::connect(":memory:").await;
        assert!(conn.is_ok(), "Failed to connect to in-memory database");

        let conn = conn.unwrap();

        // Test creating a table
        let result = conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)").await;
        assert!(result.is_ok(), "Failed to create table");

        // Test inserting data
        let result = conn.execute("INSERT INTO test (name) VALUES ('Alice')").await;
        assert!(result.is_ok(), "Failed to insert data");

        // Test querying data
        let result = conn.execute("SELECT * FROM test").await;
        assert!(result.is_ok(), "Failed to query data");

        let result = result.unwrap();
        assert_eq!(result.rows.len(), 1, "Expected 1 row");
        assert_eq!(result.columns.len(), 2, "Expected 2 columns");

        // Test listing tables
        let tables = conn.list_tables().await;
        assert!(tables.is_ok(), "Failed to list tables");

        let tables = tables.unwrap();
        assert!(tables.contains(&"test".to_string()), "Expected 'test' table");

        // Test describing table
        let schema = conn.describe_table("test").await;
        assert!(schema.is_ok(), "Failed to describe table");

        let schema = schema.unwrap();
        assert_eq!(schema.columns.len(), 2, "Expected 2 columns in schema");

        // Clean up
        conn.close();
    }

    #[tokio::test]
    async fn test_sqlite_multiple_queries() {
        let conn = SqliteConnection::connect(":memory:").await.unwrap();

        // Create table
        conn.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)").await.unwrap();

        // Insert multiple rows
        conn.execute("INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com')").await.unwrap();
        conn.execute("INSERT INTO users (name, email) VALUES ('Bob', 'bob@example.com')").await.unwrap();
        conn.execute("INSERT INTO users (name, email) VALUES ('Charlie', 'charlie@example.com')").await.unwrap();

        // Query all rows
        let result = conn.execute("SELECT * FROM users ORDER BY id").await.unwrap();
        assert_eq!(result.rows.len(), 3, "Expected 3 rows");

        // Query with filter
        let result = conn.execute("SELECT * FROM users WHERE name = 'Bob'").await.unwrap();
        assert_eq!(result.rows.len(), 1, "Expected 1 row for Bob");

        // Count query
        let result = conn.execute("SELECT COUNT(*) as count FROM users").await.unwrap();
        assert_eq!(result.rows.len(), 1, "Expected 1 row for count");

        conn.close();
    }
}
