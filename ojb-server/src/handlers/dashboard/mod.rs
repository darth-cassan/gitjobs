//! This module defines the HTTP handlers for the dashboard.

use anyhow::Result;
use axum::response::{Html, IntoResponse};
use rinja::Template;
use tracing::instrument;

use crate::{handlers::error::HandlerError, templates::dashboard};

pub(crate) mod jobs;
pub(crate) mod settings;

/// Handler that returns the dashboard page.
#[instrument(skip_all, err)]
pub(crate) async fn page() -> Result<impl IntoResponse, HandlerError> {
    let template = dashboard::Page {};

    Ok(Html(template.render()?))
}
