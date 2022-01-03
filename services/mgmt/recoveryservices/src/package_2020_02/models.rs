#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RawCertificateData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientDiscoveryDisplay {
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
pub struct ClientDiscoveryForLogSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientDiscoveryForProperties {
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ClientDiscoveryForServiceSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientDiscoveryForServiceSpecification {
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<ClientDiscoveryForLogSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientDiscoveryResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClientDiscoveryValueForSingleApi>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientDiscoveryValueForSingleApi {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ClientDiscoveryDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClientDiscoveryForProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CmkKekIdentity {
    #[serde(rename = "useSystemAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub use_system_assigned_identity: Option<bool>,
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CmkKeyVaultProperties {
    #[serde(rename = "keyUri", default, skip_serializing_if = "Option::is_none")]
    pub key_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Error>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityData {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: identity_data::Type,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod identity_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        None,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobsSummary {
    #[serde(rename = "failedJobs", default, skip_serializing_if = "Option::is_none")]
    pub failed_jobs: Option<i64>,
    #[serde(rename = "suspendedJobs", default, skip_serializing_if = "Option::is_none")]
    pub suspended_jobs: Option<i64>,
    #[serde(rename = "inProgressJobs", default, skip_serializing_if = "Option::is_none")]
    pub in_progress_jobs: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringSummary {
    #[serde(rename = "unHealthyVmCount", default, skip_serializing_if = "Option::is_none")]
    pub un_healthy_vm_count: Option<i64>,
    #[serde(rename = "unHealthyProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub un_healthy_provider_count: Option<i64>,
    #[serde(rename = "eventsCount", default, skip_serializing_if = "Option::is_none")]
    pub events_count: Option<i64>,
    #[serde(rename = "deprecatedProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub deprecated_provider_count: Option<i64>,
    #[serde(rename = "supportedProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub supported_provider_count: Option<i64>,
    #[serde(rename = "unsupportedProviderCount", default, skip_serializing_if = "Option::is_none")]
    pub unsupported_provider_count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResource {
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchTrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchVault {
    #[serde(flatten)]
    pub patch_tracked_resource: PatchTrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection::ProvisioningState>,
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
}
pub mod private_endpoint_connection {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
        Deleting,
        Failed,
        Pending,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionVaultProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnection>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResources {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<private_link_service_connection_state::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
pub mod private_link_service_connection_state {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        Approved,
        Rejected,
        Disconnected,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawCertificateData {
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<raw_certificate_data::AuthType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}
pub mod raw_certificate_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AuthType {
        Invalid,
        #[serde(rename = "ACS")]
        Acs,
        #[serde(rename = "AAD")]
        Aad,
        AccessControlService,
        AzureActiveDirectory,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationUsage {
    #[serde(rename = "monitoringSummary", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_summary: Option<MonitoringSummary>,
    #[serde(rename = "jobsSummary", default, skip_serializing_if = "Option::is_none")]
    pub jobs_summary: Option<JobsSummary>,
    #[serde(rename = "protectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_count: Option<i64>,
    #[serde(rename = "recoveryPlanCount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_plan_count: Option<i64>,
    #[serde(rename = "registeredServersCount", default, skip_serializing_if = "Option::is_none")]
    pub registered_servers_count: Option<i64>,
    #[serde(rename = "recoveryServicesProviderAuthType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_services_provider_auth_type: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationUsageList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReplicationUsage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCertificateAndAadDetails {
    #[serde(flatten)]
    pub resource_certificate_details: ResourceCertificateDetails,
    #[serde(rename = "aadAuthority")]
    pub aad_authority: String,
    #[serde(rename = "aadTenantId")]
    pub aad_tenant_id: String,
    #[serde(rename = "servicePrincipalClientId")]
    pub service_principal_client_id: String,
    #[serde(rename = "servicePrincipalObjectId")]
    pub service_principal_object_id: String,
    #[serde(rename = "azureManagementEndpointAudience")]
    pub azure_management_endpoint_audience: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCertificateAndAcsDetails {
    #[serde(flatten)]
    pub resource_certificate_details: ResourceCertificateDetails,
    #[serde(rename = "globalAcsNamespace")]
    pub global_acs_namespace: String,
    #[serde(rename = "globalAcsHostName")]
    pub global_acs_host_name: String,
    #[serde(rename = "globalAcsRPRealm")]
    pub global_acs_rp_realm: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceCertificateDetails {
    #[serde(rename = "authType")]
    pub auth_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[serde(rename = "validFrom", default, skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo", default, skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Standard,
        #[serde(rename = "RS0")]
        Rs0,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpgradeDetails {
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "startTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub start_time_utc: Option<String>,
    #[serde(rename = "lastUpdatedTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time_utc: Option<String>,
    #[serde(rename = "endTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub end_time_utc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<upgrade_details::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<upgrade_details::TriggerType>,
    #[serde(rename = "upgradedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub upgraded_resource_id: Option<String>,
    #[serde(rename = "previousResourceId", default, skip_serializing_if = "Option::is_none")]
    pub previous_resource_id: Option<String>,
}
pub mod upgrade_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Unknown,
        InProgress,
        Upgraded,
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TriggerType {
        UserTriggered,
        ForcedUpgrade,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserIdentity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultCertificateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceCertificateDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultExtendedInfo {
    #[serde(rename = "integrityKey", default, skip_serializing_if = "Option::is_none")]
    pub integrity_key: Option<String>,
    #[serde(rename = "encryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(rename = "encryptionKeyThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key_thumbprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultExtendedInfoResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultExtendedInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vault>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "upgradeDetails", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_details: Option<UpgradeDetails>,
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnectionVaultProperties>,
    #[serde(rename = "privateEndpointStateForBackup", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_state_for_backup: Option<vault_properties::PrivateEndpointStateForBackup>,
    #[serde(rename = "privateEndpointStateForSiteRecovery", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_state_for_site_recovery: Option<vault_properties::PrivateEndpointStateForSiteRecovery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<vault_properties::Encryption>,
}
pub mod vault_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrivateEndpointStateForBackup {
        None,
        Enabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrivateEndpointStateForSiteRecovery {
        None,
        Enabled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Encryption {
        #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_properties: Option<CmkKeyVaultProperties>,
        #[serde(rename = "kekIdentity", default, skip_serializing_if = "Option::is_none")]
        pub kek_identity: Option<CmkKekIdentity>,
        #[serde(rename = "infrastructureEncryption", default, skip_serializing_if = "Option::is_none")]
        pub infrastructure_encryption: Option<encryption::InfrastructureEncryption>,
    }
    pub mod encryption {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum InfrastructureEncryption {
            Enabled,
            Disabled,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<vault_usage::Unit>,
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[serde(rename = "nextResetTime", default, skip_serializing_if = "Option::is_none")]
    pub next_reset_time: Option<String>,
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<NameInfo>,
}
pub mod vault_usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountPerSecond,
        BytesPerSecond,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultUsageList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VaultUsage>,
}
