//! This module defines the HTTP handlers for the jobs page.

use std::collections::HashMap;

use anyhow::Result;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use rinja::Template;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    db::DynDB,
    handlers::{error::HandlerError, extractors::JobBoardId},
    templates::dashboard::jobs,
};

/// Handler that returns the jobs page.
#[instrument(skip_all, err)]
pub(crate) async fn page(
    State(db): State<DynDB>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, HandlerError> {
    let Some(employer_id) = query.get("employer_id") else {
        return Ok((
            StatusCode::BAD_REQUEST,
            Html("missing employer_id parameter".to_string()),
        ));
    };
    let employer_id = Uuid::parse_str(employer_id).expect("employer_id to be valid UUID");
    let jobs = db.list_employer_jobs(employer_id).await?;
    let template = jobs::Page { jobs };

    Ok((StatusCode::OK, Html(template.render()?)))
}

/// Handler that returns the form to add a new job.
#[instrument(skip_all, err)]
pub(crate) async fn add_form(
    State(db): State<DynDB>,
    JobBoardId(job_board_id): JobBoardId,
) -> Result<impl IntoResponse, HandlerError> {
    let job_board = db.get_job_board(job_board_id).await?;
    let template = jobs::AddForm {
        benefits: job_board.benefits,
        skills: job_board.skills,
    };

    Ok(Html(template.render()?))
}
