//! Server utilities.

use serde::Serialize;

/// Response from the status endpoint.
#[derive(Clone, Debug, Serialize)]
pub struct ServerStatus<'a> {
    pub application: &'a str,
    pub version: &'a str,
    pub uptime_seconds: u64,
}
