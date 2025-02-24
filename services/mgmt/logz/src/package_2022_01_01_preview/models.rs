#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of a filtering tag. Filtering tags are used for capturing resources and include/exclude them from being monitored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FilteringTag {
    #[doc = "The name (also known as the key) of the tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Valid actions for a filtering tag. Exclusion takes priority over inclusion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<TagAction>,
}
impl FilteringTag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProperties {
    #[doc = "The identity ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagedIdentityTypes>,
}
impl IdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LiftrResourceCategories {
    Unknown,
    MonitorLogs,
}
#[doc = "Set of rules for sending logs for the Monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogRules {
    #[doc = "Flag specifying if AAD logs should be sent for the Monitor resource."]
    #[serde(rename = "sendAadLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_aad_logs: Option<bool>,
    #[doc = "Flag specifying if subscription logs should be sent for the Monitor resource."]
    #[serde(rename = "sendSubscriptionLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_subscription_logs: Option<bool>,
    #[doc = "Flag specifying if activity logs from Azure resources should be sent for the Monitor resource."]
    #[serde(rename = "sendActivityLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_activity_logs: Option<bool>,
    #[doc = "List of filtering tags to be used for capturing logs. This only takes effect if SendActivityLogs flag is enabled. If empty, all resources will be captured. If only Exclude action is specified, the rules will apply to the list of all available resources. If Include actions are specified, the rules will only include resources with the associated tags."]
    #[serde(rename = "filteringTags", default, skip_serializing_if = "Vec::is_empty")]
    pub filtering_tags: Vec<FilteringTag>,
}
impl LogRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogzMetricsResource {
    #[doc = "ARM id of the metrics resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Name of the metrics resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the metrics resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties specific to the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
impl LogzMetricsResource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            system_data: None,
            name: None,
            type_: None,
            properties: None,
            identity: None,
            tags: None,
            location,
        }
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogzMetricsResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LogzMetricsResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl LogzMetricsResourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for a PATCH request to a metrics resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogzMetricsResourceUpdateParameters {
    #[doc = "The set of properties that can be update in a PATCH request to a monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorUpdateProperties>,
    #[doc = "The new tags of the metrics resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl LogzMetricsResourceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogzMonitorResource {
    #[doc = "ARM id of the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Name of the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the monitor resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties specific to the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
impl LogzMonitorResource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            system_data: None,
            name: None,
            type_: None,
            properties: None,
            identity: None,
            tags: None,
            location,
        }
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogzMonitorResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LogzMonitorResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl LogzMonitorResourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for a PATCH request to a monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogzMonitorResourceUpdateParameters {
    #[doc = "The set of properties that can be update in a PATCH request to a monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorUpdateProperties>,
    #[doc = "The new tags of the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl LogzMonitorResourceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogzOrganizationProperties {
    #[doc = "Name of the Logz organization."]
    #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[doc = "Id of the Logz organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The Id of the Enterprise App used for Single sign on."]
    #[serde(rename = "enterpriseAppId", default, skip_serializing_if = "Option::is_none")]
    pub enterprise_app_id: Option<String>,
    #[doc = "The login URL specific to this Logz Organization."]
    #[serde(rename = "singleSignOnUrl", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_url: Option<String>,
}
impl LogzOrganizationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogzSingleSignOnProperties {
    #[doc = "Various states of the SSO resource"]
    #[serde(rename = "singleSignOnState", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_state: Option<SingleSignOnStates>,
    #[doc = "The Id of the Enterprise App used for Single sign-on."]
    #[serde(rename = "enterpriseAppId", default, skip_serializing_if = "Option::is_none")]
    pub enterprise_app_id: Option<String>,
    #[doc = "The login URL specific to this Logz Organization."]
    #[serde(rename = "singleSignOnUrl", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_url: Option<String>,
    #[doc = "Flag specifying if the resource provisioning state as tracked by ARM."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl LogzSingleSignOnProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogzSingleSignOnResource {
    #[doc = "ARM id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogzSingleSignOnProperties>,
}
impl LogzSingleSignOnResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogzSingleSignOnResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LogzSingleSignOnResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl LogzSingleSignOnResourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagedIdentityTypes {
    SystemAssigned,
    UserAssigned,
}
#[doc = "Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MarketplaceSubscriptionStatus {
    Active,
    Suspended,
}
impl Default for MarketplaceSubscriptionStatus {
    fn default() -> Self {
        Self::Active
    }
}
#[doc = "Set of rules for sending metrics for the Monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricRules {
    #[doc = "Subscription Id for which filtering tags are applicable"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "List of filtering tags to be used for capturing metrics. If empty, all resources will be captured. If only Exclude action is specified, the rules will apply to the list of all available resources. If Include actions are specified, the rules will only include resources with the associated tags."]
    #[serde(rename = "filteringTags", default, skip_serializing_if = "Vec::is_empty")]
    pub filtering_tags: Vec<FilteringTag>,
}
impl MetricRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Capture metrics of Azure resources based on ARM tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsTagRules {
    #[doc = "Name of the rule set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The id of the rule set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the rule set."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Definition of the properties for a TagRules resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricsTagRulesProperties>,
}
impl MetricsTagRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsTagRulesListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MetricsTagRules>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl MetricsTagRulesListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the properties for a TagRules resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsTagRulesProperties {
    #[doc = "Flag specifying if the resource provisioning state as tracked by ARM."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Flag specifying if metrics from Azure resources should be sent for the Monitor resource."]
    #[serde(rename = "sendMetrics", default, skip_serializing_if = "Option::is_none")]
    pub send_metrics: Option<bool>,
    #[serde(rename = "metricRules", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_rules: Vec<MetricRules>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl MetricsTagRulesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties specific to the monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorProperties {
    #[doc = "Flag specifying if the resource provisioning state as tracked by ARM."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Flag specifying if the resource monitoring is enabled or disabled."]
    #[serde(rename = "monitoringStatus", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_status: Option<MonitoringStatus>,
    #[doc = "Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state."]
    #[serde(rename = "marketplaceSubscriptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_subscription_status: Option<MarketplaceSubscriptionStatus>,
    #[serde(rename = "logzOrganizationProperties", default, skip_serializing_if = "Option::is_none")]
    pub logz_organization_properties: Option<LogzOrganizationProperties>,
    #[serde(rename = "userInfo", default, skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
    #[serde(rename = "planData", default, skip_serializing_if = "Option::is_none")]
    pub plan_data: Option<PlanData>,
    #[serde(rename = "liftrResourceCategory", default, skip_serializing_if = "Option::is_none")]
    pub liftr_resource_category: Option<LiftrResourceCategories>,
    #[doc = "The priority of the resource."]
    #[serde(rename = "liftrResourcePreference", default, skip_serializing_if = "Option::is_none")]
    pub liftr_resource_preference: Option<i32>,
}
impl MonitorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The set of properties that can be update in a PATCH request to a monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorUpdateProperties {
    #[doc = "Flag specifying if the resource monitoring is enabled or disabled."]
    #[serde(rename = "monitoringStatus", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_status: Option<MonitoringStatus>,
}
impl MonitorUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a resource currently being monitored by the Logz monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoredResource {
    #[doc = "The ARM id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Flag indicating if resource is sending metrics to Logz."]
    #[serde(rename = "sendingMetrics", default, skip_serializing_if = "Option::is_none")]
    pub sending_metrics: Option<bool>,
    #[doc = "Reason for why the resource is sending metrics (or why it is not sending)."]
    #[serde(rename = "reasonForMetricsStatus", default, skip_serializing_if = "Option::is_none")]
    pub reason_for_metrics_status: Option<String>,
    #[doc = "Flag indicating if resource is sending logs to Logz."]
    #[serde(rename = "sendingLogs", default, skip_serializing_if = "Option::is_none")]
    pub sending_logs: Option<bool>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Reason for why the resource is sending logs (or why it is not sending)."]
    #[serde(rename = "reasonForLogsStatus", default, skip_serializing_if = "Option::is_none")]
    pub reason_for_logs_status: Option<String>,
}
impl MonitoredResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoredResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MonitoredResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl MonitoredResourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Flag specifying if the resource monitoring is enabled or disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MonitoringStatus {
    Enabled,
    Disabled,
}
impl Default for MonitoringStatus {
    fn default() -> Self {
        Self::Enabled
    }
}
#[doc = "Capture logs and metrics of Azure resources based on ARM tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringTagRules {
    #[doc = "Name of the rule set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The id of the rule set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the rule set."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Definition of the properties for a TagRules resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitoringTagRulesProperties>,
}
impl MonitoringTagRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringTagRulesListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MonitoringTagRules>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl MonitoringTagRulesListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the properties for a TagRules resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringTagRulesProperties {
    #[doc = "Flag specifying if the resource provisioning state as tracked by ARM."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Set of rules for sending logs for the Monitor resource."]
    #[serde(rename = "logRules", default, skip_serializing_if = "Option::is_none")]
    pub log_rules: Option<LogRules>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl MonitoringTagRulesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider, i.e., Microsoft.Logz."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Type on which the operation is performed, e.g., 'monitors'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation type, e.g., read, write, delete, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation, e.g., 'Write monitors'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of GET request to list the Microsoft.Logz operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Microsoft.Logz provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResult>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Microsoft.Logz REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "Operation name, i.e., {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanData {
    #[doc = "different usage type like PAYG/COMMITTED. this could be enum"]
    #[serde(rename = "usageType", default, skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
    #[doc = "different billing cycles like MONTHLY/WEEKLY. this could be enum"]
    #[serde(rename = "billingCycle", default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[doc = "plan id as published by Logz"]
    #[serde(rename = "planDetails", default, skip_serializing_if = "Option::is_none")]
    pub plan_details: Option<String>,
    #[doc = "date when plan was applied"]
    #[serde(rename = "effectiveDate", default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
}
impl PlanData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Flag specifying if the resource provisioning state as tracked by ARM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
    Deleted,
    NotSpecified,
}
#[doc = "Various states of the SSO resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SingleSignOnStates {
    Initial,
    Enable,
    Disable,
    Existing,
}
#[doc = "Valid actions for a filtering tag. Exclusion takes priority over inclusion."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TagAction {
    Include,
    Exclude,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserInfo {
    #[doc = "First Name of the user"]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last Name of the user"]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Email of the user used by Logz for contacting them if needed"]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[doc = "Phone number of the user used by Logz for contacting them if needed"]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl UserInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User roles on configured in Logz.io account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    None,
    User,
    Admin,
}
#[doc = "Response for list of user's role for Logz.io account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserRoleListResponse {
    #[doc = "List of user roles for Logz.io account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UserRoleResponse>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl UserRoleListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request for checking user's role for Logz.io account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserRoleRequest {
    #[doc = "Email of the user used by Logz for contacting them if needed"]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}
impl UserRoleRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for checking user's role for Logz.io account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserRoleResponse {
    #[doc = "User roles on configured in Logz.io account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<UserRole>,
}
impl UserRoleResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of payload to be passed while installing VM agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmExtensionPayload {
    #[doc = "API Key corresponding to the resource."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[doc = "Logz.io region where the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}
impl VmExtensionPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request of a list VM Host Update Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmHostUpdateRequest {
    #[doc = "Request of a list vm host update operation."]
    #[serde(rename = "vmResourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_resource_ids: Vec<VmResources>,
    #[doc = "Various states of the updating vm extension on resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<VmHostUpdateState>,
}
impl VmHostUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Various states of the updating vm extension on resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VmHostUpdateState {
    Install,
    Delete,
}
#[doc = "VM Resource Ids"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmResources {
    #[doc = "Request of a list vm host update operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Version of the Logz agent installed on the VM."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
}
impl VmResources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list VM Host Update Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmResourcesListResponse {
    #[doc = "Response of a list vm host update operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VmResources>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl VmResourcesListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
