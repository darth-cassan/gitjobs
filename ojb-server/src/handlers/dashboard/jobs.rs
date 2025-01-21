//! This module defines the HTTP handlers for the jobs page.

use std::collections::HashMap;

use anyhow::Result;
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use rinja::Template;
use tracing::instrument;
use uuid::Uuid;

use crate::{db::DynDB, handlers::error::HandlerError, templates::dashboard::jobs::Page};

/// Handler that returns the jobs page.
#[instrument(skip_all, err)]
pub(crate) async fn page(
    State(db): State<DynDB>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, HandlerError> {
    let msg = "expected employer query string parameter (uuid)"; // TODO
    let employer_id = Uuid::parse_str(query.get("employer").expect(msg)).expect(msg);
    let jobs = db.list_employer_jobs(employer_id).await?;
    let template = Page { jobs };

    Ok(Html(template.render()?))
}
