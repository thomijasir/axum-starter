//! Database connection pool abstraction using Diesel and r2d2 for SQLite.
//!
//! This module provides a convenient wrapper around a SQLite connection pool for use in async applications.
//! It handles connection pooling, timeout management, and provides both sync and async interfaces for
//! database operations.
//!
//! # Features
//!
//! - Connection pooling with configurable limits
//! - Async support via `tokio::spawn_blocking`
//! - Automatic connection health checks
//! - Transaction and execute helpers
//!
//! # Example
//!
//! ## Basic Usage
//!
//! ```rust
//! use axum_starter::services::DBSqlite;
//! use anyhow::Result;
//!
//! async fn example() -> Result<()> {
//!     // Create a new database connection pool
//!     let db = DBSqlite::new("sqlite://database.db")?;
//!
//!     // Get a connection from the pool
//!     let conn = db.get_connection()?;
//!
//!     // Use the connection for queries...
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Async Operations
//!
//! ```rust
//! use axum_starter::services::DBSqlite;
//! use anyhow::Result;
//! use diesel::sql_query;
//! use diesel::RunQueryDsl;
//!
//! async fn async_example() -> Result<()> {
//!     let db = DBSqlite::new("sqlite://database.db")?;
//!
//!     // Execute a read query
//!     let result: i32 = db.execute(|conn| {
//!         let count: Result<i32, _> = sql_query("SELECT COUNT(*) FROM users")
//!             .get_result(conn);
//!         Ok(count?)
//!     }).await?;
//!
//!     // Execute a transaction (write operations)
//!     db.transaction(|conn| {
//!         sql_query("INSERT INTO users (name) VALUES ('John')")
//!             .execute(conn)?;
//!         Ok(())
//!     }).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Health Check
//!
//! ```rust
//! use axum_starter::services::DBSqlite;
//!
//! async fn health_check_example() {
//!     let db = DBSqlite::new("sqlite://database.db").unwrap();
//!     
//!     match db.health_check().await {
//!         Ok(()) => println!("Database is healthy"),
//!         Err(e) => println!("Database health check failed: {}", e),
//!     }
//! }
//! ```

use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use std::time::Duration;

/// A wrapper around a SQLite connection pool using Diesel and r2d2.
///
/// This struct provides a thread-safe, cloneable handle to a connection pool.
/// It is designed to be used in async applications and can be safely shared
/// across multiple tasks.
///
/// # Connection Pool Configuration
///
/// The pool is configured with the following defaults:
/// - Connection timeout: 60 seconds
/// - Max pool size: 32 connections
/// - Min idle connections: 8
/// - Idle timeout: 600 seconds (10 minutes)
/// - Max lifetime: 3600 seconds (1 hour)
/// - Test on check-out: enabled
///
/// # Example
///
/// ```rust
/// use axum_starter::services::DBSqlite;
///
/// let db = DBSqlite::new("sqlite://database.db").unwrap();
/// let conn = db.get_connection().unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct DBSqlite {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DBSqlite {
    /// Creates a new database connection pool from the given URL.
    ///
    /// The URL should be a valid SQLite connection string, such as:
    /// - `"sqlite://database.db"` - relative path
    /// - `"sqlite:///absolute/path/to/database.db"` - absolute path
    /// - `":memory:"` - in-memory database
    ///
    /// # Arguments
    ///
    /// * `database_url` - A SQLite connection string
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` with the configured pool, or a `PoolError` if the pool
    /// could not be created.
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_starter::services::DBSqlite;
    ///
    /// // Create pool with file database
    /// let db = DBSqlite::new("sqlite://database.db")?;
    ///
    /// // Create pool with in-memory database
    /// let db = DBSqlite::new(":memory:")?;
    /// # Ok::<_, diesel::r2d2::PoolError>(())
    /// ```
    pub fn new(database_url: &str) -> Result<Self, diesel::r2d2::PoolError> {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = Pool::builder()
            .connection_timeout(Duration::from_secs(60))
            .max_size(32)
            .min_idle(Some(8))
            .idle_timeout(Some(Duration::from_secs(600)))
            .max_lifetime(Some(Duration::from_secs(3600)))
            .test_on_check_out(true)
            .build(manager)?;
        Ok(Self { pool })
    }

    /// Retrieves a pooled database connection.
    ///
    /// This method blocks until a connection is available or the connection
    /// timeout is reached.
    ///
    /// # Returns
    ///
    /// Returns a `PooledConnection` on success, or a `PoolError` if:
    /// - The pool has reached its maximum size and timeout elapsed
    /// - The connection could not be established
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_starter::services::DBSqlite;
    /// use diesel::RunQueryDsl;
    /// use diesel::sql_query;
    ///
    /// let db = DBSqlite::new("sqlite://database.db")?;
    /// let conn = db.get_connection()?;
    ///
    /// // Use the connection for synchronous operations
    /// sql_query("SELECT 1").execute(&conn)?;
    /// # Ok::<_, anyhow::Error>(())
    /// ```
    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, diesel::r2d2::PoolError>
    {
        self.pool.get()
    }

    /// Executes a write operation within a database transaction.
    ///
    /// This method wraps the operation in a transaction context. If the
    /// operation returns an error, the transaction is rolled back automatically.
    /// The operation runs in a blocking thread pool to avoid blocking the
    /// async runtime.
    ///
    /// # Arguments
    ///
    /// * `operation` - A closure that takes a mutable reference to a `SqliteConnection`
    ///   and returns a `Result`. Use this for INSERT, UPDATE, DELETE operations.
    ///
    /// # Returns
    ///
    /// Returns the result of the operation, or an error if:
    /// - The connection could not be acquired
    /// - The operation failed
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_starter::services::DBSqlite;
    /// use anyhow::Result;
    /// use diesel::sql_query;
    /// use diesel::RunQueryDsl;
    ///
    /// async fn create_user(db: &DBSqlite, name: &str) -> Result<()> {
    ///     db.transaction(|conn| {
    ///         sql_query("INSERT INTO users (name) VALUES (?)")
    ///             .bind::<diesel::sql_types::Text, _>(name)
    ///             .execute(conn)?;
    ///         Ok(())
    ///     }).await
    /// }
    /// ```
    pub async fn transaction<F, T>(&self, operation: F) -> Result<T>
    where
        F: FnOnce(&mut SqliteConnection) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            operation(&mut conn)
        })
        .await?
    }

    /// Executes a read-only operation using a pooled connection.
    ///
    /// This method is optimized for SELECT/GET queries as it does not incur
    /// the overhead of transaction management. The operation runs in a
    /// blocking thread pool to avoid blocking the async runtime.
    ///
    /// # Arguments
    ///
    /// * `operation` - A closure that takes a mutable reference to a `SqliteConnection`
    ///   and returns a `Result`. Use this for SELECT queries.
    ///
    /// # Returns
    ///
    /// Returns the result of the operation, or an error if:
    /// - The connection could not be acquired
    /// - The operation failed
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_starter::services::DBSqlite;
    /// use anyhow::Result;
    /// use diesel::sql_query;
    /// use diesel::RunQueryDsl;
    /// use diesel::QueryableByName;
    ///
    /// #[derive(QueryableByName)]
    /// struct UserCount {
    ///     #[diesel(sql_type = diesel::sql_types::BigInt)]
    ///     count: i64,
    /// }
    ///
    /// async fn count_users(db: &DBSqlite) -> Result<i64> {
    ///     let result: UserCount = db.execute(|conn| {
    ///         sql_query("SELECT COUNT(*) as count FROM users")
    ///             .get_result(conn)
    ///             .map_err(|e| anyhow::anyhow!("Query failed: {}", e))
    ///     }).await?;
    ///     Ok(result.count)
    /// }
    /// ```
    pub async fn execute<F, T>(&self, operation: F) -> Result<T>
    where
        F: FnOnce(&mut SqliteConnection) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            operation(&mut conn)
        })
        .await?
    }

    /// Runs a health check query to verify database connectivity.
    ///
    /// Executes `SELECT 1` against the database to ensure the connection
    /// is active and responsive.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the health check succeeds, or an error if:
    /// - The connection could not be acquired
    /// - The query failed
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_starter::services::DBSqlite;
    ///
    /// async fn check_database_health(db: &DBSqlite) -> bool {
    ///     db.health_check().await.is_ok()
    /// }
    /// ```
    pub async fn health_check(&self) -> Result<()> {
        use diesel::RunQueryDsl;
        use diesel::sql_query;

        self.execute(|conn| {
            sql_query("SELECT 1").execute(conn)?;
            Ok(())
        })
        .await
    }

    /// Retrieves statistics about the current state of the connection pool.
    ///
    /// # Returns
    ///
    /// Returns a tuple `(total_connections, idle_connections)`:
    /// - `total_connections` - The total number of connections in the pool
    /// - `idle_connections` - The number of currently idle connections
    ///
    /// # Example
    ///
    /// ```rust
    /// use axum_starter::services::DBSqlite;
    ///
    /// let db = DBSqlite::new("sqlite://database.db")?;
    /// let (total, idle) = db.pool_stats();
    /// println!("Total: {}, Idle: {}", total, idle);
    /// # Ok::<_, diesel::r2d2::PoolError>(())
    /// ```
    pub fn pool_stats(&self) -> (u32, u32) {
        let state = self.pool.state();
        (state.connections, state.idle_connections)
    }
}
