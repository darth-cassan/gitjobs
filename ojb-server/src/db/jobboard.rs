//! This module defines some database functionality for the job board site.

use anyhow::Result;
use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

use super::PgDB;

/// Trait that defines some database operations used in the job board site.
#[async_trait]
pub(crate) trait DBJobBoard {
    /// Get the job board id from the host provided.
    async fn get_job_board_id(&self, host: &str) -> Result<Option<Uuid>>;
}

#[async_trait]
impl DBJobBoard for PgDB {
    /// [DBJobBoard::get_job_board_id]
    #[instrument(skip(self), err)]
    async fn get_job_board_id(&self, host: &str) -> Result<Option<Uuid>> {
        let db = self.pool.get().await?;
        let job_board_id = db
            .query_opt(
                "select job_board_id from job_board where host = $1::text",
                &[&host],
            )
            .await?
            .map(|row| row.get("job_board_id"));

        Ok(job_board_id)
    }
}
