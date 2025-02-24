#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientTokenResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl ClientTokenResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
