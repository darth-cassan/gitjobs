//! Custom extractors for handlers.

use std::collections::HashMap;

use anyhow::Result;
use axum::{
    extract::{FromRequestParts, Query},
    http::{header::HOST, request::Parts, StatusCode},
};
use cached::proc_macro::cached;
use tracing::{error, instrument};
use uuid::Uuid;

use crate::{db::DynDB, router};

/// Custom extractor to get the job board id from the request's host header.
pub(crate) struct JobBoardId(pub Uuid);

impl FromRequestParts<router::State> for JobBoardId {
    type Rejection = (StatusCode, &'static str);

    #[instrument(skip_all, err(Debug))]
    async fn from_request_parts(parts: &mut Parts, state: &router::State) -> Result<Self, Self::Rejection> {
        // Extract host from the request headers
        let Some(host_header) = parts.headers.get(HOST) else {
            return Err((StatusCode::BAD_REQUEST, "missing host header"));
        };
        let host = host_header
            .to_str()
            .unwrap_or_default()
            .split(':')
            .next()
            .unwrap_or_default();

        // Lookup the job board id in the database
        let Some(job_board_id) = lookup_job_board_id(state.db.clone(), host).await.map_err(|err| {
            error!(?err, "error looking up job board id");
            (StatusCode::INTERNAL_SERVER_ERROR, "")
        })?
        else {
            return Err((StatusCode::BAD_REQUEST, "job board host not found"));
        };

        Ok(JobBoardId(job_board_id))
    }
}

/// Lookup the job board id in the database using the host provided.
#[cached(
    time = 86400,
    key = "String",
    convert = r#"{ String::from(host) }"#,
    sync_writes = true,
    result = true
)]
#[instrument(skip(db), err)]
async fn lookup_job_board_id(db: DynDB, host: &str) -> Result<Option<Uuid>> {
    if host.is_empty() {
        return Ok(None);
    }
    db.get_job_board_id(host).await
}

/// Custom extractor to get the employer id from the request's query string.
pub(crate) struct EmployerId(pub Uuid);

impl FromRequestParts<router::State> for EmployerId {
    type Rejection = (StatusCode, &'static str);

    #[instrument(skip_all, err(Debug))]
    async fn from_request_parts(parts: &mut Parts, state: &router::State) -> Result<Self, Self::Rejection> {
        let Ok(query) = Query::<HashMap<String, String>>::from_request_parts(parts, state).await else {
            return Err((StatusCode::BAD_REQUEST, "error parsing query string"));
        };
        let Some(employer_id) = query.get("employer_id") else {
            return Err((StatusCode::BAD_REQUEST, "missing employer_id parameter"));
        };
        let Ok(employer_id) = Uuid::parse_str(employer_id) else {
            return Err((StatusCode::BAD_REQUEST, "employer_id should be a valid uuid"));
        };
        Ok(EmployerId(employer_id))
    }
}
