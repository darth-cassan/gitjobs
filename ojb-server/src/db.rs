//! This module defines an abstraction layer over the database.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use tracing::instrument;
use uuid::Uuid;

/// Abstraction layer over the database. Trait that defines some operations a
/// DB implementation must support.
#[async_trait]
pub(crate) trait DB {
    /// Get the board id from the host provided.
    async fn get_board_id(&self, host: &str) -> Result<Option<Uuid>>;
}

/// Type alias to represent a DB trait object.
pub(crate) type DynDB = Arc<dyn DB + Send + Sync>;

/// DB implementation backed by `PostgreSQL`.
pub(crate) struct PgDB {
    pool: Pool,
}

impl PgDB {
    /// Create a new `PgDB` instance.
    pub(crate) fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DB for PgDB {
    /// [DB::get_board_id]
    #[instrument(skip(self), err)]
    async fn get_board_id(&self, host: &str) -> Result<Option<Uuid>> {
        let db = self.pool.get().await?;
        let board_id = db
            .query_opt("select board_id from board where host = $1::text", &[&host])
            .await?
            .map(|row| row.get("board_id"));

        Ok(board_id)
    }
}
