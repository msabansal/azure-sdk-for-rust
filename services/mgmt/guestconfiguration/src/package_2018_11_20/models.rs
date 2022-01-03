#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentReportDetails {
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<assignment_report_details::ComplianceStatus>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<assignment_report_details::OperationType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<AssignmentReportResource>,
}
pub mod assignment_report_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComplianceStatus {
        Compliant,
        NonCompliant,
        Pending,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationType {
        Consistency,
        Initial,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentReportResource {
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<assignment_report_resource::ComplianceStatus>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<AssignmentReportResourceComplianceReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
pub mod assignment_report_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComplianceStatus {
        Compliant,
        NonCompliant,
        Pending,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentReportResourceComplianceReason {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationSetting {
    #[serde(rename = "configurationMode", default, skip_serializing_if = "Option::is_none")]
    pub configuration_mode: Option<configuration_setting::ConfigurationMode>,
    #[serde(rename = "allowModuleOverwrite", default, skip_serializing_if = "Option::is_none")]
    pub allow_module_overwrite: Option<bool>,
    #[serde(rename = "actionAfterReboot", default, skip_serializing_if = "Option::is_none")]
    pub action_after_reboot: Option<configuration_setting::ActionAfterReboot>,
    #[serde(rename = "refreshFrequencyMins", default, skip_serializing_if = "Option::is_none")]
    pub refresh_frequency_mins: Option<f64>,
    #[serde(rename = "rebootIfNeeded", default, skip_serializing_if = "Option::is_none")]
    pub reboot_if_needed: Option<bool>,
    #[serde(rename = "configurationModeFrequencyMins", default, skip_serializing_if = "Option::is_none")]
    pub configuration_mode_frequency_mins: Option<f64>,
}
pub mod configuration_setting {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConfigurationMode {
        ApplyOnly,
        ApplyAndMonitor,
        ApplyAndAutoCorrect,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionAfterReboot {
        ContinueConfiguration,
        StopConfiguration,
    }
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
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestConfigurationAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GuestConfigurationAssignmentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestConfigurationAssignmentList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GuestConfigurationAssignment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestConfigurationAssignmentProperties {
    #[serde(rename = "guestConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub guest_configuration: Option<GuestConfigurationNavigation>,
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<guest_configuration_assignment_properties::ComplianceStatus>,
    #[serde(rename = "lastComplianceStatusChecked", default, skip_serializing_if = "Option::is_none")]
    pub last_compliance_status_checked: Option<String>,
    #[serde(rename = "latestReportId", default, skip_serializing_if = "Option::is_none")]
    pub latest_report_id: Option<String>,
    #[serde(rename = "vmssVMList", default, skip_serializing_if = "Vec::is_empty")]
    pub vmss_vm_list: Vec<VmssvmInfo>,
    #[serde(rename = "parameterHash", default, skip_serializing_if = "Option::is_none")]
    pub parameter_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "assignmentHash", default, skip_serializing_if = "Option::is_none")]
    pub assignment_hash: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<guest_configuration_assignment_properties::ProvisioningState>,
}
pub mod guest_configuration_assignment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComplianceStatus {
        Compliant,
        NonCompliant,
        Pending,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Created,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestConfigurationAssignmentReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GuestConfigurationAssignmentReportProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestConfigurationAssignmentReportList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GuestConfigurationAssignmentReport>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestConfigurationAssignmentReportProperties {
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<guest_configuration_assignment_report_properties::ComplianceStatus>,
    #[serde(rename = "reportId", default, skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment: Option<AssignmentInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm: Option<VmInfo>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<AssignmentReportDetails>,
    #[serde(rename = "vmssResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vmss_resource_id: Option<String>,
}
pub mod guest_configuration_assignment_report_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComplianceStatus {
        Compliant,
        NonCompliant,
        Pending,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestConfigurationNavigation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<guest_configuration_navigation::Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "contentUri", default, skip_serializing_if = "Option::is_none")]
    pub content_uri: Option<String>,
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "contentHash", default, skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(rename = "assignmentType", default, skip_serializing_if = "Option::is_none")]
    pub assignment_type: Option<guest_configuration_navigation::AssignmentType>,
    #[serde(rename = "configurationParameter", default, skip_serializing_if = "Vec::is_empty")]
    pub configuration_parameter: Vec<ConfigurationParameter>,
    #[serde(rename = "configurationProtectedParameter", default, skip_serializing_if = "Vec::is_empty")]
    pub configuration_protected_parameter: Vec<ConfigurationParameter>,
    #[serde(rename = "configurationSetting", default, skip_serializing_if = "Option::is_none")]
    pub configuration_setting: Option<ConfigurationSetting>,
}
pub mod guest_configuration_navigation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "DSC")]
        Dsc,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AssignmentType {
        Audit,
        DeployAndAutoCorrect,
        ApplyAndAutoCorrect,
        ApplyAndMonitor,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<operation::Properties>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
        pub status_code: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmssvmInfo {
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[serde(rename = "vmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_id: Option<String>,
    #[serde(rename = "complianceStatus", default, skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<vmssvm_info::ComplianceStatus>,
    #[serde(rename = "latestReportId", default, skip_serializing_if = "Option::is_none")]
    pub latest_report_id: Option<String>,
    #[serde(rename = "lastComplianceChecked", default, skip_serializing_if = "Option::is_none")]
    pub last_compliance_checked: Option<String>,
}
pub mod vmssvm_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComplianceStatus {
        Compliant,
        NonCompliant,
        Pending,
    }
}
