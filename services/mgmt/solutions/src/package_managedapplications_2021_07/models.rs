#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    pub properties: ApplicationProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationArtifact {
    pub name: ApplicationArtifactName,
    pub uri: String,
    #[serde(rename = "type")]
    pub type_: ApplicationArtifactType,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationArtifactName {
    NotSpecified,
    ViewDefinition,
    Authorizations,
    CustomRoleDefinition,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationArtifactType {
    NotSpecified,
    Template,
    Custom,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationAuthorization {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationBillingDetailsDefinition {
    #[serde(rename = "resourceUsageId", default, skip_serializing_if = "Option::is_none")]
    pub resource_usage_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationClientDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub puid: Option<String>,
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinition {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    pub properties: ApplicationDefinitionProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionArtifact {
    pub name: ApplicationDefinitionArtifactName,
    pub uri: String,
    #[serde(rename = "type")]
    pub type_: ApplicationArtifactType,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationDefinitionArtifactName {
    NotSpecified,
    ApplicationResourceTemplate,
    CreateUiDefinition,
    MainTemplateParameters,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationDefinition>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionPatchable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDefinitionProperties {
    #[serde(rename = "lockLevel")]
    pub lock_level: ApplicationLockLevel,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorizations: Vec<ApplicationAuthorization>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ApplicationDefinitionArtifact>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "packageFileUri", default, skip_serializing_if = "Option::is_none")]
    pub package_file_uri: Option<String>,
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[serde(rename = "mainTemplate", default, skip_serializing_if = "Option::is_none")]
    pub main_template: Option<serde_json::Value>,
    #[serde(rename = "createUiDefinition", default, skip_serializing_if = "Option::is_none")]
    pub create_ui_definition: Option<serde_json::Value>,
    #[serde(rename = "notificationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub notification_policy: Option<ApplicationNotificationPolicy>,
    #[serde(rename = "lockingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub locking_policy: Option<ApplicationPackageLockingPolicyDefinition>,
    #[serde(rename = "deploymentPolicy", default, skip_serializing_if = "Option::is_none")]
    pub deployment_policy: Option<ApplicationDeploymentPolicy>,
    #[serde(rename = "managementPolicy", default, skip_serializing_if = "Option::is_none")]
    pub management_policy: Option<ApplicationManagementPolicy>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<ApplicationPolicy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationDeploymentPolicy {
    #[serde(rename = "deploymentMode")]
    pub deployment_mode: DeploymentMode,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationJitAccessPolicy {
    #[serde(rename = "jitAccessEnabled")]
    pub jit_access_enabled: bool,
    #[serde(rename = "jitApprovalMode", default, skip_serializing_if = "Option::is_none")]
    pub jit_approval_mode: Option<JitApprovalMode>,
    #[serde(rename = "jitApprovers", default, skip_serializing_if = "Vec::is_empty")]
    pub jit_approvers: Vec<JitApproverDefinition>,
    #[serde(rename = "maximumJitAccessDuration", default, skip_serializing_if = "Option::is_none")]
    pub maximum_jit_access_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationLockLevel {
    CanNotDelete,
    ReadOnly,
    None,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationManagementMode {
    NotSpecified,
    Unmanaged,
    Managed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationManagementPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ApplicationManagementMode>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationNotificationEndpoint {
    pub uri: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationNotificationPolicy {
    #[serde(rename = "notificationEndpoints")]
    pub notification_endpoints: Vec<ApplicationNotificationEndpoint>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPackageContact {
    #[serde(rename = "contactName", default, skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    pub email: String,
    pub phone: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPackageLockingPolicyDefinition {
    #[serde(rename = "allowedActions", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_actions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPackageSupportUrls {
    #[serde(rename = "publicAzure", default, skip_serializing_if = "Option::is_none")]
    pub public_azure: Option<String>,
    #[serde(rename = "governmentCloud", default, skip_serializing_if = "Option::is_none")]
    pub government_cloud: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPatchable {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanPatchable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(rename = "managedResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_id: Option<String>,
    #[serde(rename = "applicationDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub application_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "billingDetails", default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<ApplicationBillingDetailsDefinition>,
    #[serde(rename = "jitAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub jit_access_policy: Option<ApplicationJitAccessPolicy>,
    #[serde(rename = "publisherTenantId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorizations: Vec<ApplicationAuthorization>,
    #[serde(rename = "managementMode", default, skip_serializing_if = "Option::is_none")]
    pub management_mode: Option<ApplicationManagementMode>,
    #[serde(rename = "customerSupport", default, skip_serializing_if = "Option::is_none")]
    pub customer_support: Option<ApplicationPackageContact>,
    #[serde(rename = "supportUrls", default, skip_serializing_if = "Option::is_none")]
    pub support_urls: Option<ApplicationPackageSupportUrls>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ApplicationArtifact>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ApplicationClientDetails>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<ApplicationClientDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPropertiesPatchable {
    #[serde(rename = "managedResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_id: Option<String>,
    #[serde(rename = "applicationDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub application_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeploymentMode {
    NotSpecified,
    Incremental,
    Complete,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JitApprovalMode {
    NotSpecified,
    AutoApprove,
    ManualApprove,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitApproverDefinition {
    pub id: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<jit_approver_definition::Type>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
pub mod jit_approver_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "group")]
        Group,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitAuthorizationPolicies {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitRequestDefinition {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JitRequestProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitRequestDefinitionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JitRequestDefinition>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitRequestPatchable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitRequestProperties {
    #[serde(rename = "applicationResourceId")]
    pub application_resource_id: String,
    #[serde(rename = "publisherTenantId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_tenant_id: Option<String>,
    #[serde(rename = "jitAuthorizationPolicies")]
    pub jit_authorization_policies: Vec<JitAuthorizationPolicies>,
    #[serde(rename = "jitSchedulingPolicy")]
    pub jit_scheduling_policy: JitSchedulingPolicy,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "jitRequestState", default, skip_serializing_if = "Option::is_none")]
    pub jit_request_state: Option<JitRequestState>,
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ApplicationClientDetails>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<ApplicationClientDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JitRequestState {
    NotSpecified,
    Pending,
    Approved,
    Denied,
    Failed,
    Canceled,
    Expired,
    Timeout,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JitSchedulingPolicy {
    #[serde(rename = "type")]
    pub type_: JitSchedulingType,
    pub duration: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JitSchedulingType {
    NotSpecified,
    Once,
    Recurring,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned,UserAssigned")]
    SystemAssignedUserAssigned,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
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
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        Internal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    pub publisher: String,
    pub product: String,
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    pub version: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanPatchable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    NotSpecified,
    Accepted,
    Running,
    Deleting,
    Deleted,
    Canceled,
    Failed,
    Succeeded,
    Updating,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedIdentities {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedIdentity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
