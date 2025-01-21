//! This module defines some database functionality for the dashboard.

use anyhow::Result;
use async_trait::async_trait;
use tracing::instrument;

use crate::templates::dashboard::jobs::{JobBoard, JobSummary};

use super::PgDB;

/// Trait that defines some database operations used in the dashboard.
#[async_trait]
pub(crate) trait DBDashBoard {
    /// Get job board.
    async fn get_job_board(&self, job_board_id: uuid::Uuid) -> Result<JobBoard>;

    /// List employer jobs.
    async fn list_employer_jobs(&self, employer_id: uuid::Uuid) -> Result<Vec<JobSummary>>;
}

#[async_trait]
impl DBDashBoard for PgDB {
    /// [DBDashBoard::get_job_board]
    #[instrument(skip(self), err)]
    async fn get_job_board(&self, job_board_id: uuid::Uuid) -> Result<JobBoard> {
        let db = self.pool.get().await?;
        let row = db
            .query_one(
                "
                select
                    benefits,
                    skills
                from job_board
                where job_board_id = $1::uuid
                ",
                &[&job_board_id],
            )
            .await?;
        let job_board = JobBoard {
            benefits: row.get("benefits"),
            skills: row.get("skills"),
        };

        Ok(job_board)
    }

    /// [DBDashBoard::list_employer_jobs]
    #[instrument(skip(self), err)]
    async fn list_employer_jobs(&self, employer_id: uuid::Uuid) -> Result<Vec<JobSummary>> {
        let db = self.pool.get().await?;
        let jobs = db
            .query(
                "
                select
                    j.job_id,
                    j.created_at,
                    j.title,
                    j.status,
                    j.archived_at,
                    j.published_at,
                    l.city,
                    l.country
                from job j
                join location l using (location_id)
                where employer_id = $1::uuid
                order by published_at desc, created_at desc
                ",
                &[&employer_id],
            )
            .await?
            .into_iter()
            .map(|row| JobSummary {
                created_at: row.get("created_at"),
                job_id: row.get("job_id"),
                title: row.get("title"),
                status: row.get("status"),
                city: row.get("city"),
                country: row.get("country"),
                archived_at: row.get("archived_at"),
                published_at: row.get("published_at"),
            })
            .collect();

        Ok(jobs)
    }
}
