//! This module defines some common database functionality.

use anyhow::Result;
use async_trait::async_trait;
use tracing::instrument;

use crate::templates::common::Location;

use super::PgDB;

/// Trait that defines some common database operations.
#[async_trait]
pub(crate) trait DBCommon {
    /// Search locations.
    async fn search_locations(&self, ts_query: &str) -> Result<Vec<Location>>;
}

#[async_trait]
impl DBCommon for PgDB {
    /// [DBCommon::search_locations]
    #[instrument(skip(self), err)]
    async fn search_locations(&self, ts_query: &str) -> Result<Vec<Location>> {
        let db = self.pool.get().await?;
        let locations = db
            .query("select * from search_locations($1::text)", &[&ts_query])
            .await?
            .into_iter()
            .map(|row| Location {
                location_id: row.get("location_id"),
                city: row.get("city"),
                country: row.get("country"),
                state: row.get("state"),
            })
            .collect();

        Ok(locations)
    }
}
