//! This module defines the HTTP handlers for the home page of the board site.

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
    handlers::{error::HandlerError, extractors::BoardId},
    templates::board::home::Index,
};

/// Handler that returns the home index page.
#[instrument(skip_all, err)]
pub(crate) async fn index(
    State(_db): State<DynDB>,
    BoardId(board_id): BoardId,
    _uri: Uri,
) -> Result<impl IntoResponse, HandlerError> {
    debug!("board_id: {}", board_id);

    Ok(Html(Index {}.render()?))
}
