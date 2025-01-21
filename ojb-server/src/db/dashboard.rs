//! This module defines some database functionality for the dashboard.

use anyhow::Result;
use async_trait::async_trait;
use tracing::instrument;

use crate::templates::dashboard::jobs::{JobBoard, JobStatus, JobSummary, NewJob};

use super::PgDB;

/// Trait that defines some database operations used in the dashboard.
#[async_trait]
pub(crate) trait DBDashBoard {
    /// Add job to the job board.
    async fn add_job(&self, employer_id: uuid::Uuid, job: NewJob) -> Result<()>;

    /// Get job board.
    async fn get_job_board(&self, job_board_id: uuid::Uuid) -> Result<JobBoard>;

    /// List employer jobs.
    async fn list_employer_jobs(&self, employer_id: uuid::Uuid) -> Result<Vec<JobSummary>>;
}

#[async_trait]
impl DBDashBoard for PgDB {
    /// [DBDashBoard::add_job]
    #[instrument(skip(self), err)]
    async fn add_job(&self, employer_id: uuid::Uuid, job: NewJob) -> Result<()> {
        let db = self.pool.get().await?;
        db.execute(
            "
            insert into job (
                employer_id,
                type,
                status,
                location_id,
                workplace,
                title,
                description,
                apply_instructions,
                apply_url,
                benefits,
                open_source,
                salary,
                salary_currency,
                salary_min,
                salary_max,
                salary_period,
                skills,
                upstream_commitment
            ) values (
                $1::uuid,
                $2::text,
                $3::text,
                $4::uuid,
                $5::text,
                $6::text,
                $7::text,
                $8::text,
                $9::text,
                $10::text[],
                $11::int,
                $12::bigint,
                $13::text,
                $14::bigint,
                $15::bigint,
                $16::text,
                $17::text[],
                $18::int
            )
            ",
            &[
                &employer_id,
                &job.type_.to_string(),
                &JobStatus::Draft.to_string(),
                &job.location_id,
                &job.workplace.to_string(),
                &job.title,
                &job.description,
                &job.apply_instructions,
                &job.apply_url,
                &job.benefits,
                &job.open_source,
                &job.salary,
                &job.salary_currency,
                &job.salary_min,
                &job.salary_max,
                &job.salary_period,
                &job.skills,
                &job.upstream_commitment,
            ],
        )
        .await?;

        Ok(())
    }

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
                left join location l using (location_id)
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
                status: row.get::<_, String>("status").parse().expect("valid job status"),
                city: row.get("city"),
                country: row.get("country"),
                archived_at: row.get("archived_at"),
                published_at: row.get("published_at"),
            })
            .collect();

        Ok(jobs)
    }
}
