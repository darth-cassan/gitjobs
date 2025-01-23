//! This module defines some templates and types used in the settings page.

use rinja::Template;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

/// Update employer page template.
#[derive(Debug, Clone, Template, Serialize, Deserialize)]
#[template(path = "dashboard/settings/update_employer.html")]
pub(crate) struct UpdateEmployerPage {
    pub employer_details: EmployerDetails,
}

/// Employer details.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EmployerDetails {
    pub company: String,
    pub description: String,
    pub public: bool,

    pub city: Option<String>,
    pub country: Option<String>,
    pub location_id: Option<Uuid>,
    pub logo_url: Option<String>,
    pub state: Option<String>,
    pub website_url: Option<String>,
}
