#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Registry {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryCredentials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Registry>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryNameCheckRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: registry_name_check_request::Type,
}
pub mod registry_name_check_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.ContainerRegistry/registries")]
        MicrosoftContainerRegistryRegistries,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryNameStatus {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryProperties {
    #[serde(rename = "loginServer", default, skip_serializing_if = "Option::is_none")]
    pub login_server: Option<String>,
    #[serde(rename = "creationDate", default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "adminUserEnabled", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_enabled: Option<bool>,
    #[serde(rename = "storageAccount")]
    pub storage_account: StorageAccountProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryPropertiesUpdateParameters {
    #[serde(rename = "adminUserEnabled", default, skip_serializing_if = "Option::is_none")]
    pub admin_user_enabled: Option<bool>,
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<StorageAccountProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistryPropertiesUpdateParameters>,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountProperties {
    pub name: String,
    #[serde(rename = "accessKey")]
    pub access_key: String,
}
