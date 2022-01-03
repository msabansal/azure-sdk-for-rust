#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdmCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdmCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdmCredentialProperties {
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "authTokenUrl", default, skip_serializing_if = "Option::is_none")]
    pub auth_token_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApnsCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApnsCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApnsCredentialProperties {
    #[serde(rename = "apnsCertificate", default, skip_serializing_if = "Option::is_none")]
    pub apns_certificate: Option<String>,
    #[serde(rename = "certificateKey", default, skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaiduCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BaiduCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaiduCredentialProperties {
    #[serde(rename = "baiduApiKey", default, skip_serializing_if = "Option::is_none")]
    pub baidu_api_key: Option<String>,
    #[serde(rename = "baiduEndPoint", default, skip_serializing_if = "Option::is_none")]
    pub baidu_end_point: Option<String>,
    #[serde(rename = "baiduSecretKey", default, skip_serializing_if = "Option::is_none")]
    pub baidu_secret_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAvailabilityParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(rename = "isAvailiable", default, skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAvailabilityResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "isAvailiable", default, skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcmCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GcmCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcmCredentialProperties {
    #[serde(rename = "gcmEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub gcm_endpoint: Option<String>,
    #[serde(rename = "googleApiKey", default, skip_serializing_if = "Option::is_none")]
    pub google_api_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpnsCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MpnsCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MpnsCredentialProperties {
    #[serde(rename = "mpnsCertificate", default, skip_serializing_if = "Option::is_none")]
    pub mpns_certificate: Option<String>,
    #[serde(rename = "certificateKey", default, skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NamespaceResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespacePatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "serviceBusEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_endpoint: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "scaleUnit", default, skip_serializing_if = "Option::is_none")]
    pub scale_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    #[serde(rename = "namespaceType", default, skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<namespace_properties::NamespaceType>,
}
pub mod namespace_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NamespaceType {
        Messaging,
        NotificationHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubCreateOrUpdateParameters {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: NotificationHubProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NotificationHubResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "registrationTtl", default, skip_serializing_if = "Option::is_none")]
    pub registration_ttl: Option<String>,
    #[serde(rename = "authorizationRules", default, skip_serializing_if = "Vec::is_empty")]
    pub authorization_rules: Vec<SharedAccessAuthorizationRuleProperties>,
    #[serde(rename = "apnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub apns_credential: Option<ApnsCredential>,
    #[serde(rename = "wnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub wns_credential: Option<WnsCredential>,
    #[serde(rename = "gcmCredential", default, skip_serializing_if = "Option::is_none")]
    pub gcm_credential: Option<GcmCredential>,
    #[serde(rename = "mpnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub mpns_credential: Option<MpnsCredential>,
    #[serde(rename = "admCredential", default, skip_serializing_if = "Option::is_none")]
    pub adm_credential: Option<AdmCredential>,
    #[serde(rename = "baiduCredential", default, skip_serializing_if = "Option::is_none")]
    pub baidu_credential: Option<BaiduCredential>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NotificationHubProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PnsCredentialsProperties {
    #[serde(rename = "apnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub apns_credential: Option<ApnsCredential>,
    #[serde(rename = "wnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub wns_credential: Option<WnsCredential>,
    #[serde(rename = "gcmCredential", default, skip_serializing_if = "Option::is_none")]
    pub gcm_credential: Option<GcmCredential>,
    #[serde(rename = "mpnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub mpns_credential: Option<MpnsCredential>,
    #[serde(rename = "admCredential", default, skip_serializing_if = "Option::is_none")]
    pub adm_credential: Option<AdmCredential>,
    #[serde(rename = "baiduCredential", default, skip_serializing_if = "Option::is_none")]
    pub baidu_credential: Option<BaiduCredential>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PnsCredentialsResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PnsCredentialsProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicykeyResource {
    #[serde(rename = "policyKey", default, skip_serializing_if = "Option::is_none")]
    pub policy_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceListKeys {
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleCreateOrUpdateParameters {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: SharedAccessAuthorizationRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedAccessAuthorizationRuleResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rights: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedAccessAuthorizationRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Free,
        Basic,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WnsCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WnsCredentialProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WnsCredentialProperties {
    #[serde(rename = "packageSid", default, skip_serializing_if = "Option::is_none")]
    pub package_sid: Option<String>,
    #[serde(rename = "secretKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[serde(rename = "windowsLiveEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub windows_live_endpoint: Option<String>,
}
