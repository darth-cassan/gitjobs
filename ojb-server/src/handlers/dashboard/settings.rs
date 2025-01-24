//! This module defines the HTTP handlers for the settings page.

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use axum_extra::extract::Form;
use rinja::Template;
use tracing::instrument;

use crate::{
    db::DynDB,
    handlers::{error::HandlerError, extractors::EmployerId},
    templates::dashboard::settings,
};

// Pages handlers.

/// Handler that returns the page to update the employer details.
#[instrument(skip_all, err)]
pub(crate) async fn update_employer_page(
    State(db): State<DynDB>,
    EmployerId(employer_id): EmployerId,
) -> Result<impl IntoResponse, HandlerError> {
    let employer_details = db.get_employer_details(&employer_id).await?;
    let template = settings::UpdateEmployerPage { employer_details };

    Ok(Html(template.render()?))
}

// Actions handlers.

/// Handler that updates the employer details.
#[instrument(skip_all, err)]
pub(crate) async fn update_employer(
    State(db): State<DynDB>,
    EmployerId(employer_id): EmployerId,
    Form(employer_details): Form<settings::EmployerDetails>,
) -> Result<impl IntoResponse, HandlerError> {
    db.update_employer(&employer_id, &employer_details).await?;

    Ok(StatusCode::NO_CONTENT)
}
