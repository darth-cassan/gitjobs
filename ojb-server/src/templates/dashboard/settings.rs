//! This module defines some templates and types used in the settings page.

use rinja::Template;
use serde::{Deserialize, Serialize};

/// Settings page template.
#[derive(Debug, Clone, Template, Serialize, Deserialize)]
#[template(path = "dashboard/settings/page.html")]
pub(crate) struct Page {}
