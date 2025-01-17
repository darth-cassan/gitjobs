//! This module defines the HTTP handlers for the settings page.

use anyhow::Result;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use rinja::Template;
use tracing::{debug, instrument};

use crate::{
    db::DynDB,
    handlers::{error::HandlerError, extractors::JobBoardId},
    templates::dashboard::settings::Page,
};

/// Handler that returns the settings page.
#[instrument(skip_all, err)]
pub(crate) async fn page(
    State(_db): State<DynDB>,
    JobBoardId(job_board_id): JobBoardId,
) -> Result<impl IntoResponse, HandlerError> {
    debug!("job_board_id: {}", job_board_id);

    Ok(Html(Page {}.render()?))
}
