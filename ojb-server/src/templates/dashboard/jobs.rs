//! This module defines some templates and types used in the jobs page.

use chrono::{DateTime, Utc};
use postgres_types::{FromSql, ToSql};
use rinja::Template;
use serde::{Deserialize, Serialize};

use crate::templates::{filters, helpers::DATE_FORMAT};

/// Jobs page template.
#[derive(Debug, Clone, Template, Serialize, Deserialize)]
#[template(path = "dashboard/jobs/page.html")]
pub(crate) struct Page {
    pub jobs: Vec<JobSummary>,
}

/// Job summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct JobSummary {
    pub created_at: DateTime<Utc>,
    pub job_id: uuid::Uuid,
    pub title: String,
    pub status: JobStatus,

    pub archived_at: Option<DateTime<Utc>>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Job status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromSql, ToSql)]
pub(crate) enum JobStatus {
    Archived,
    Draft,
    Published,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Archived => write!(f, "Archived"),
            JobStatus::Draft => write!(f, "Draft"),
            JobStatus::Published => write!(f, "Published"),
        }
    }
}

/// Add job form.
#[derive(Debug, Clone, Template, Serialize, Deserialize)]
#[template(path = "dashboard/jobs/add_form.html")]
pub(crate) struct AddForm {
    pub benefits: Vec<String>,
    pub skills: Vec<String>,
}

/// Job board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct JobBoard {
    pub benefits: Vec<String>,
    pub skills: Vec<String>,
}
