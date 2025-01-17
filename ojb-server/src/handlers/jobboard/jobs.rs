//! This module defines the HTTP handlers for the jobs page.

use std::collections::HashMap;

use anyhow::Result;
use axum::{
    extract::{Query, RawQuery, State},
    http::{HeaderMap, Uri},
    response::{Html, IntoResponse},
};
use rinja::Template;
use tracing::{debug, instrument};

use crate::{
    db::DynDB,
    handlers::{error::HandlerError, extractors::JobBoardId},
    templates::jobboard::jobs::Index,
};

/// Handler that returns the index page.
#[instrument(skip_all, err)]
pub(crate) async fn index(
    State(_db): State<DynDB>,
    JobBoardId(board_id): JobBoardId,
    Query(_query): Query<HashMap<String, String>>,
    RawQuery(_raw_query): RawQuery,
    _headers: HeaderMap,
    _uri: Uri,
) -> Result<impl IntoResponse, HandlerError> {
    debug!("board_id: {}", board_id);

    Ok(Html(Index {}.render()?))
}
