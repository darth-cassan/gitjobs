//! This module defines the HTTP handlers for the about page.

use anyhow::Result;
use axum::{
    extract::State,
    http::Uri,
    response::{Html, IntoResponse},
};
use rinja::Template;
use tracing::{debug, instrument};

use crate::{
    db::DynDB,
    handlers::{error::HandlerError, extractors::JobBoardId},
    templates::jobboard::about::Index,
};

/// Handler that returns the index page.
#[instrument(skip_all, err)]
pub(crate) async fn index(
    State(_db): State<DynDB>,
    JobBoardId(board_id): JobBoardId,
    _uri: Uri,
) -> Result<impl IntoResponse, HandlerError> {
    debug!("board_id: {}", board_id);

    Ok(Html(Index {}.render()?))
}
