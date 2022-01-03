#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttestedData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Compute {
    #[serde(rename = "azEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub az_environment: Option<String>,
    #[serde(rename = "customData", default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[serde(rename = "placementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub placement_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanProperties>,
    #[serde(rename = "publicKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<PublicKeysProperties>,
    #[serde(rename = "platformFaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain: Option<String>,
    #[serde(rename = "platformUpdateDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_update_domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[serde(rename = "vmScaleSetName", default, skip_serializing_if = "Option::is_none")]
    pub vm_scale_set_name: Option<String>,
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<identity_error_response::Error>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}
pub mod identity_error_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Error {
        #[serde(rename = "invalid_request")]
        InvalidRequest,
        #[serde(rename = "unauthorized_client")]
        UnauthorizedClient,
        #[serde(rename = "access_denied")]
        AccessDenied,
        #[serde(rename = "unsupported_response_type")]
        UnsupportedResponseType,
        #[serde(rename = "invalid_scope")]
        InvalidScope,
        #[serde(rename = "server_error")]
        ServerError,
        #[serde(rename = "service_unavailable")]
        ServiceUnavailable,
        #[serde(rename = "bad_request")]
        BadRequest,
        #[serde(rename = "forbidden")]
        Forbidden,
        #[serde(rename = "not_found")]
        NotFound,
        #[serde(rename = "method_not_allowed")]
        MethodNotAllowed,
        #[serde(rename = "too_many_requests")]
        TooManyRequests,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityInfoResponse {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityTokenResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext_expires_in: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msi_res_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Instance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute: Option<Compute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ipv4Properties {
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ipv6Properties {
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Network {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interface: Vec<NetworkInterface>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterface {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<network_interface::Ipv4>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<network_interface::Ipv6>,
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}
pub mod network_interface {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Ipv4 {
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_address: Vec<Ipv4Properties>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub subnet: Vec<SubnetProperties>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Ipv6 {
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_address: Vec<Ipv6Properties>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKeysProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "keyData", default, skip_serializing_if = "Option::is_none")]
    pub key_data: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubnetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}
