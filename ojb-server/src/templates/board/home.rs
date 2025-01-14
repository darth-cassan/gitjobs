//! This module defines some templates and types used in the home page of the
//! board site.

use rinja::Template;
use serde::{Deserialize, Serialize};

/// Home index page template.
#[derive(Debug, Clone, Template, Serialize, Deserialize)]
#[template(path = "board/home/index.html")]
pub(crate) struct Index {}
