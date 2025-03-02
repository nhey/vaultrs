use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [ListMountsRequest][crate::api::sys::requests::ListMountsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct MountResponse {
    pub accessor: String,
    pub config: MountConfigResponse,
    pub description: String,
    pub external_entropy_access: bool,
    pub local: bool,
    pub options: Option<HashMap<String, String>>,
    pub seal_wrap: bool,
    #[serde(rename = "type")]
    pub mount_type: String,
    pub uuid: String,
}

/// Response from executing
/// [ListMountsRequest][crate::api::sys::requests::ListMountsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct MountConfigResponse {
    pub default_lease_ttl: u64,
    pub force_no_cache: bool,
    pub max_lease_ttl: u64,
}

/// Response from executing
/// [ListAuthsRequest][crate::api::sys::requests::ListAuthsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct AuthResponse {
    pub accessor: String,
    pub config: AuthConfigResponse,
    pub description: String,
    pub external_entropy_access: bool,
    pub local: bool,
    pub options: Option<HashMap<String, String>>,
    pub seal_wrap: bool,
    #[serde(rename = "type")]
    pub mount_type: String,
    pub uuid: String,
}

/// Response from executing
/// [ListAuthsRequest][crate::api::sys::requests::ListAuthsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct AuthConfigResponse {
    pub default_lease_ttl: u64,
    pub force_no_cache: bool,
    pub max_lease_ttl: u64,
    pub token_type: String,
}

/// Response from executing
/// [WrappingLookupRequest][crate::api::sys::requests::WrappingLookupRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct WrappingLookupResponse {
    pub creation_path: String,
    pub creation_time: String,
    pub creation_ttl: u64,
}

/// Response from executing
/// [ReadHealthRequest][crate::api::sys::requests::ReadHealthRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadHealthResponse {
    pub cluster_id: String,
    pub cluster_name: String,
    pub initialized: bool,
    pub performance_standby: bool,
    pub replication_dr_mode: Option<String>,
    pub replication_perf_mode: Option<String>,
    pub sealed: bool,
    pub server_time_utc: u64,
    pub standby: bool,
    pub version: String,
}
