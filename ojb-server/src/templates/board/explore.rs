//! This module defines some templates and types used in the explore page of
//! the board site.

use rinja::Template;
use serde::{Deserialize, Serialize};

/// Explore index page template.
#[derive(Debug, Clone, Template, Serialize, Deserialize)]
#[template(path = "board/explore/index.html")]
pub(crate) struct Index {}
