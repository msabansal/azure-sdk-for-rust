#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
}
pub mod error_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<ErrorDetails>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthMonitor {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HealthMonitorProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthMonitorList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HealthMonitor>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthMonitorProperties {
    #[serde(rename = "monitorName", default, skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<String>,
    #[serde(rename = "monitorType", default, skip_serializing_if = "Option::is_none")]
    pub monitor_type: Option<String>,
    #[serde(rename = "monitoredObject", default, skip_serializing_if = "Option::is_none")]
    pub monitored_object: Option<String>,
    #[serde(rename = "parentMonitorName", default, skip_serializing_if = "Option::is_none")]
    pub parent_monitor_name: Option<String>,
    #[serde(rename = "previousMonitorState", default, skip_serializing_if = "Option::is_none")]
    pub previous_monitor_state: Option<HealthState>,
    #[serde(rename = "currentMonitorState", default, skip_serializing_if = "Option::is_none")]
    pub current_monitor_state: Option<HealthState>,
    #[serde(rename = "evaluationTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_timestamp: Option<String>,
    #[serde(rename = "currentStateFirstObservedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub current_state_first_observed_timestamp: Option<String>,
    #[serde(rename = "lastReportedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_reported_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<serde_json::Value>,
    #[serde(rename = "monitorConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub monitor_configuration: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthMonitorStateChange {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HealthMonitorStateChangeProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthMonitorStateChangeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HealthMonitorStateChange>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthMonitorStateChangeProperties {
    #[serde(rename = "monitorName", default, skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<String>,
    #[serde(rename = "monitorType", default, skip_serializing_if = "Option::is_none")]
    pub monitor_type: Option<String>,
    #[serde(rename = "monitoredObject", default, skip_serializing_if = "Option::is_none")]
    pub monitored_object: Option<String>,
    #[serde(rename = "evaluationTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_timestamp: Option<String>,
    #[serde(rename = "currentStateFirstObservedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub current_state_first_observed_timestamp: Option<String>,
    #[serde(rename = "previousMonitorState", default, skip_serializing_if = "Option::is_none")]
    pub previous_monitor_state: Option<HealthState>,
    #[serde(rename = "currentMonitorState", default, skip_serializing_if = "Option::is_none")]
    pub current_monitor_state: Option<HealthState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<serde_json::Value>,
    #[serde(rename = "monitorConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub monitor_configuration: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HealthState {
    Healthy,
    Critical,
    Warning,
    Unknown,
    Disabled,
    None,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    pub name: String,
    pub display: operation::Display,
    pub origin: String,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        pub provider: String,
        pub resource: String,
        pub operation: String,
        pub description: String,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
