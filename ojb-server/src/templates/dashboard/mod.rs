//! This module defines the templates for the dashboard pages.

use rinja::Template;
use serde::{Deserialize, Serialize};

pub(crate) mod jobs;
pub(crate) mod settings;

/// Dashboard page template.
#[derive(Debug, Clone, Template, Serialize, Deserialize)]
#[template(path = "dashboard/page.html")]
pub(crate) struct Page {}
