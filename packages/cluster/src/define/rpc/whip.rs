use poem_openapi::Object;
use proc_macro::{IntoVecU8, TryFromSliceU8};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Object, PartialEq, Eq, IntoVecU8, TryFromSliceU8, Clone)]
pub struct WhipConnectRequest {
    pub session_uuid: u64,
    pub ip_addr: String,
    pub user_agent: String,
    pub token: String,
    pub sdp: Option<String>,
    pub compressed_sdp: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize, Object, PartialEq, Eq, IntoVecU8, TryFromSliceU8)]
pub struct WhipConnectResponse {
    pub conn_id: String,
    pub sdp: Option<String>,
    pub compressed_sdp: Option<Vec<u8>>,
}