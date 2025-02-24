#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Error response indicates Azure Resource Manager is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Http status code."]
    #[serde(rename = "httpStatus", default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<String>,
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The principal ID of the resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the resource identity."]
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
        None,
    }
}
#[doc = "The policy assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignment {
    #[doc = "The policy assignment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyAssignmentProperties>,
    #[doc = "The ID of the policy assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the policy assignment."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the policy assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The policy sku. This property is optional, obsolete, and will be ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PolicySku>,
    #[doc = "The location of the policy assignment. Only required when utilizing managed identity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl PolicyAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of policy assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentListResult {
    #[doc = "An array of policy assignments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyAssignment>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PolicyAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy assignment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentProperties {
    #[doc = "The display name of the policy assignment."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The ID of the policy definition or policy set definition being assigned."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "The scope for the policy assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The policy's excluded scopes."]
    #[serde(rename = "notScopes", default, skip_serializing_if = "Vec::is_empty")]
    pub not_scopes: Vec<String>,
    #[doc = "Required if a parameter is used in policy rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "This message will be part of response in case of policy violation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The policy assignment metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl PolicyAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinition {
    #[doc = "The policy definition properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyDefinitionProperties>,
    #[doc = "The ID of the policy definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the policy definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource (Microsoft.Authorization/policyDefinitions)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PolicyDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of policy definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinitionListResult {
    #[doc = "An array of policy definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PolicyDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinitionProperties {
    #[doc = "The type of policy definition. Possible values are NotSpecified, BuiltIn, and Custom."]
    #[serde(rename = "policyType", default, skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<policy_definition_properties::PolicyType>,
    #[doc = "The policy definition mode. Some examples are All, Indexed, Microsoft.KeyVault.Data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[doc = "The display name of the policy definition."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The policy definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The policy rule."]
    #[serde(rename = "policyRule", default, skip_serializing_if = "Option::is_none")]
    pub policy_rule: Option<serde_json::Value>,
    #[doc = "The policy definition metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "Required if a parameter is used in policy rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl PolicyDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_definition_properties {
    use super::*;
    #[doc = "The type of policy definition. Possible values are NotSpecified, BuiltIn, and Custom."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PolicyType {
        NotSpecified,
        BuiltIn,
        Custom,
    }
}
#[doc = "The policy definition reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinitionReference {
    #[doc = "The ID of the policy definition or policy set definition."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "Required if a parameter is used in policy rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl PolicyDefinitionReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy set definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicySetDefinition {
    #[doc = "The policy set definition properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicySetDefinitionProperties>,
    #[doc = "The ID of the policy set definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the policy set definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource (Microsoft.Authorization/policySetDefinitions)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PolicySetDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of policy set definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicySetDefinitionListResult {
    #[doc = "An array of policy set definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicySetDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PolicySetDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy set definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicySetDefinitionProperties {
    #[doc = "The type of policy definition. Possible values are NotSpecified, BuiltIn, and Custom."]
    #[serde(rename = "policyType", default, skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<policy_set_definition_properties::PolicyType>,
    #[doc = "The display name of the policy set definition."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The policy set definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The policy set definition metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The policy set definition parameters that can be used in policy definition references."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "An array of policy definition references."]
    #[serde(rename = "policyDefinitions")]
    pub policy_definitions: Vec<PolicyDefinitionReference>,
}
impl PolicySetDefinitionProperties {
    pub fn new(policy_definitions: Vec<PolicyDefinitionReference>) -> Self {
        Self {
            policy_type: None,
            display_name: None,
            description: None,
            metadata: None,
            parameters: None,
            policy_definitions,
        }
    }
}
pub mod policy_set_definition_properties {
    use super::*;
    #[doc = "The type of policy definition. Possible values are NotSpecified, BuiltIn, and Custom."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PolicyType {
        NotSpecified,
        BuiltIn,
        Custom,
    }
}
#[doc = "The policy sku. This property is optional, obsolete, and will be ignored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicySku {
    #[doc = "The name of the policy sku. Possible values are A0 and A1."]
    pub name: String,
    #[doc = "The policy sku tier. Possible values are Free and Standard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl PolicySku {
    pub fn new(name: String) -> Self {
        Self { name, tier: None }
    }
}
