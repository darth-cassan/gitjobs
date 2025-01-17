//! This module defines some templates and types used in the jobs page.

use rinja::Template;
use serde::{Deserialize, Serialize};

/// Index page template.
#[derive(Debug, Clone, Template, Serialize, Deserialize)]
#[template(path = "jobboard/jobs/index.html")]
pub(crate) struct Index {}
