//! This module defines some database functionality for the dashboard.

use super::PgDB;
use async_trait::async_trait;

/// Trait that defines some database operations used in the dashboard.
#[async_trait]
pub(crate) trait DBDashBoard {}

#[async_trait]
impl DBDashBoard for PgDB {}
