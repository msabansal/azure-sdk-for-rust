#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AliasPathType {
    #[doc = "The path of an alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The api versions."]
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
}
impl AliasPathType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AliasType {
    #[doc = "The alias name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The paths for an alias."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<AliasPathType>,
}
impl AliasType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment dependency information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BasicDependency {
    #[doc = "The ID of the dependency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The dependency resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The dependency resource name."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
impl BasicDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response for a resource management request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebugSetting {
    #[doc = "The debug detail level."]
    #[serde(rename = "detailLevel", default, skip_serializing_if = "Option::is_none")]
    pub detail_level: Option<String>,
}
impl DebugSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment dependency information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dependency {
    #[doc = "The list of dependencies."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<BasicDependency>,
    #[doc = "The ID of the dependency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The dependency resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The dependency resource name."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
impl Dependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment operation parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Deployment {
    #[doc = "Deployment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentProperties>,
}
impl Deployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentExportResult {
    #[doc = "The template content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
}
impl DeploymentExportResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentExtended {
    #[doc = "The ID of the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the deployment."]
    pub name: String,
    #[doc = "Deployment properties with additional details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentPropertiesExtended>,
}
impl DeploymentExtended {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            properties: None,
        }
    }
}
#[doc = "Deployment filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentExtendedFilter {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl DeploymentExtendedFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of deployments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentListResult {
    #[doc = "The list of deployments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeploymentExtended>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DeploymentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment operation information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentOperation {
    #[doc = "Full deployment operation id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Deployment operation id."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "Deployment operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentOperationProperties>,
}
impl DeploymentOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment operation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentOperationProperties {
    #[doc = "The state of the provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The date and time of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[doc = "Deployment operation service request id."]
    #[serde(rename = "serviceRequestId", default, skip_serializing_if = "Option::is_none")]
    pub service_request_id: Option<String>,
    #[doc = "Operation status code."]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[doc = "Operation status message."]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<serde_json::Value>,
    #[doc = "Target resource."]
    #[serde(rename = "targetResource", default, skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<TargetResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<HttpMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<HttpMessage>,
}
impl DeploymentOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of deployment operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentOperationsListResult {
    #[doc = "The list of deployments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeploymentOperation>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DeploymentOperationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentProperties {
    #[doc = "The template content. It can be a JObject or a well formed JSON string. Use only one of Template or TemplateLink."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[doc = "Entity representing the reference to the template."]
    #[serde(rename = "templateLink", default, skip_serializing_if = "Option::is_none")]
    pub template_link: Option<TemplateLink>,
    #[doc = "Deployment parameters. It can be a JObject or a well formed JSON string. Use only one of Parameters or ParametersLink."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Entity representing the reference to the deployment parameters."]
    #[serde(rename = "parametersLink", default, skip_serializing_if = "Option::is_none")]
    pub parameters_link: Option<ParametersLink>,
    #[doc = "The deployment mode."]
    pub mode: deployment_properties::Mode,
    #[serde(rename = "debugSetting", default, skip_serializing_if = "Option::is_none")]
    pub debug_setting: Option<DebugSetting>,
}
impl DeploymentProperties {
    pub fn new(mode: deployment_properties::Mode) -> Self {
        Self {
            template: None,
            template_link: None,
            parameters: None,
            parameters_link: None,
            mode,
            debug_setting: None,
        }
    }
}
pub mod deployment_properties {
    use super::*;
    #[doc = "The deployment mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Incremental,
        Complete,
    }
}
#[doc = "Deployment properties with additional details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentPropertiesExtended {
    #[doc = "The state of the provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The correlation ID of the deployment."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The timestamp of the template deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[doc = "Key/value pairs that represent deployment output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[doc = "The list of resource providers needed for the deployment."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub providers: Vec<Provider>,
    #[doc = "The list of deployment dependencies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,
    #[doc = "The template content. Use only one of Template or TemplateLink."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[doc = "Entity representing the reference to the template."]
    #[serde(rename = "templateLink", default, skip_serializing_if = "Option::is_none")]
    pub template_link: Option<TemplateLink>,
    #[doc = "Deployment parameters. Use only one of Parameters or ParametersLink."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Entity representing the reference to the deployment parameters."]
    #[serde(rename = "parametersLink", default, skip_serializing_if = "Option::is_none")]
    pub parameters_link: Option<ParametersLink>,
    #[doc = "The deployment mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<deployment_properties_extended::Mode>,
    #[serde(rename = "debugSetting", default, skip_serializing_if = "Option::is_none")]
    pub debug_setting: Option<DebugSetting>,
}
impl DeploymentPropertiesExtended {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment_properties_extended {
    use super::*;
    #[doc = "The deployment mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Incremental,
        Complete,
    }
}
#[doc = "Information from validate template deployment response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentValidateResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ResourceManagementErrorWithDetails>,
    #[doc = "Deployment properties with additional details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentPropertiesExtended>,
}
impl DeploymentValidateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
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
    pub details: Vec<ErrorResponse>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Export resource group template request parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportTemplateRequest {
    #[doc = "The IDs of the resources to filter the export by. To export all resources, supply an array with single entry '*'."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[doc = "The export template options. A CSV-formatted list containing zero or more of the following: 'IncludeParameterDefaultValue', 'IncludeComments', 'SkipResourceNameParameterization', 'SkipAllParameterization'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}
impl ExportTemplateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Plan for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "The resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The kind of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Id of the resource that manages this resource."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "Sku for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl GenericResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericResourceExpanded {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[doc = "The created time of the resource. This is only present if requested via the $expand query parameter."]
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[doc = "The changed time of the resource. This is only present if requested via the $expand query parameter."]
    #[serde(rename = "changedTime", default, skip_serializing_if = "Option::is_none")]
    pub changed_time: Option<String>,
    #[doc = "The provisioning state of the resource. This is only present if requested via the $expand query parameter."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl GenericResourceExpanded {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericResourceFilter {
    #[doc = "The resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The tag name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagname: Option<String>,
    #[doc = "The tag value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagvalue: Option<String>,
}
impl GenericResourceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpMessage {
    #[doc = "HTTP message content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}
impl HttpMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The principal id of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[doc = "Entity representing the reference to the deployment parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParametersLink {
    #[doc = "URI referencing the template."]
    pub uri: String,
    #[doc = "If included it must match the ContentVersion in the template."]
    #[serde(rename = "contentVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
}
impl ParametersLink {
    pub fn new(uri: String) -> Self {
        Self {
            uri,
            content_version: None,
        }
    }
}
#[doc = "Plan for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Plan {
    #[doc = "The plan ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The publisher ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The offer ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "The promotion code."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}
impl Plan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource provider information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Provider {
    #[doc = "The provider id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The namespace of the provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The registration state of the provider."]
    #[serde(rename = "registrationState", default, skip_serializing_if = "Option::is_none")]
    pub registration_state: Option<String>,
    #[doc = "The collection of provider resource types."]
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<ProviderResourceType>,
}
impl Provider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of resource providers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderListResult {
    #[doc = "The list of resource providers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Provider>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ProviderListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource type managed by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderResourceType {
    #[doc = "The resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The collection of locations where this resource type can be created in."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "The aliases that are supported by this resource type."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<AliasType>,
    #[doc = "The api version."]
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[doc = "The properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ProviderResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource group information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroup {
    #[doc = "The ID of the resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The Name of the resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceGroupProperties>,
    #[doc = "The location of the resource group. It cannot be changed after the resource group has been created. Has to be one of the supported Azure Locations, such as West US, East US, West Europe, East Asia, etc."]
    pub location: String,
    #[doc = "The tags attached to the resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceGroup {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            properties: None,
            location,
            tags: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupExportResult {
    #[doc = "The template content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ResourceManagementErrorWithDetails>,
}
impl ResourceGroupExportResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource group filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupFilter {
    #[doc = "The tag name."]
    #[serde(rename = "tagName", default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[doc = "The tag value."]
    #[serde(rename = "tagValue", default, skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}
impl ResourceGroupFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of resource groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupListResult {
    #[doc = "The list of resource groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceGroup>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl ResourceGroupListResult {
    pub fn new(next_link: String) -> Self {
        Self {
            value: Vec::new(),
            next_link,
        }
    }
}
#[doc = "The resource group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupProperties {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ResourceGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of resource groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceListResult {
    #[doc = "The list of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GenericResourceExpanded>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl ResourceListResult {
    pub fn new(next_link: String) -> Self {
        Self {
            value: Vec::new(),
            next_link,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceManagementErrorWithDetails {
    #[doc = "The error code returned from the server."]
    pub code: String,
    #[doc = "The error message returned from the server."]
    pub message: String,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Validation error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ResourceManagementErrorWithDetails>,
}
impl ResourceManagementErrorWithDetails {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[doc = "Resource provider operation's display properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationDisplayProperties {
    #[doc = "Operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Operation provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Operation resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ResourceProviderOperationDisplayProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters of move resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcesMoveInfo {
    #[doc = "The ids of the resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[doc = "The target resource group."]
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
}
impl ResourcesMoveInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sku for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "The sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The sku tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The sku size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The sku family."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The sku model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "The sku capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag count."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagCount {
    #[doc = "Type of count."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Value of count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TagCount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagDetails {
    #[doc = "The tag ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The tag name."]
    #[serde(rename = "tagName", default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[doc = "Tag count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<TagCount>,
    #[doc = "The list of tag values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<TagValue>,
}
impl TagDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagValue {
    #[doc = "The tag ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The tag value."]
    #[serde(rename = "tagValue", default, skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    #[doc = "Tag count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<TagCount>,
}
impl TagValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of subscription tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsListResult {
    #[doc = "The list of tags."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TagDetails>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl TagsListResult {
    pub fn new(next_link: String) -> Self {
        Self {
            value: Vec::new(),
            next_link,
        }
    }
}
#[doc = "Target resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetResource {
    #[doc = "The ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
impl TargetResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to calculate template hash. It contains a string of minified template and its hash."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TemplateHashResult {
    #[doc = "The minified template string."]
    #[serde(rename = "minifiedTemplate", default, skip_serializing_if = "Option::is_none")]
    pub minified_template: Option<String>,
    #[doc = "The template hash."]
    #[serde(rename = "templateHash", default, skip_serializing_if = "Option::is_none")]
    pub template_hash: Option<String>,
}
impl TemplateHashResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Entity representing the reference to the template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateLink {
    #[doc = "URI referencing the template."]
    pub uri: String,
    #[doc = "If included it must match the ContentVersion in the template."]
    #[serde(rename = "contentVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
}
impl TemplateLink {
    pub fn new(uri: String) -> Self {
        Self {
            uri,
            content_version: None,
        }
    }
}
