#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alias {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<AliasPath>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<alias::Type>,
    #[serde(rename = "defaultPath", default, skip_serializing_if = "Option::is_none")]
    pub default_path: Option<String>,
    #[serde(rename = "defaultPattern", default, skip_serializing_if = "Option::is_none")]
    pub default_pattern: Option<AliasPattern>,
    #[serde(rename = "defaultMetadata", default, skip_serializing_if = "Option::is_none")]
    pub default_metadata: Option<AliasPathMetadata>,
}
pub mod alias {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        PlainText,
        Mask,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasPath {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<AliasPattern>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AliasPathMetadata>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasPathMetadata {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<alias_path_metadata::Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<alias_path_metadata::Attributes>,
}
pub mod alias_path_metadata {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        Any,
        String,
        Object,
        Array,
        Integer,
        Number,
        Boolean,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Attributes {
        None,
        Modifiable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasPattern {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<alias_pattern::Type>,
}
pub mod alias_pattern {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        Extract,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataEffect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "detailsSchema", default, skip_serializing_if = "Option::is_none")]
    pub details_schema: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataManifestCustomResourceFunctionDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "fullyQualifiedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_resource_type: Option<String>,
    #[serde(rename = "defaultProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub default_properties: Vec<String>,
    #[serde(rename = "allowCustomProperties", default, skip_serializing_if = "Option::is_none")]
    pub allow_custom_properties: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataManifestResourceFunctionsDefinition {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub standard: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom: Vec<DataManifestCustomResourceFunctionDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataPolicyManifest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataPolicyManifestProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataPolicyManifestListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataPolicyManifest>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataPolicyManifestProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespaces: Vec<String>,
    #[serde(rename = "policyMode", default, skip_serializing_if = "Option::is_none")]
    pub policy_mode: Option<String>,
    #[serde(rename = "isBuiltInOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_built_in_only: Option<bool>,
    #[serde(rename = "resourceTypeAliases", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_type_aliases: Vec<ResourceTypeAliases>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<DataEffect>,
    #[serde(rename = "fieldValues", default, skip_serializing_if = "Vec::is_empty")]
    pub field_values: Vec<String>,
    #[serde(rename = "resourceFunctions", default, skip_serializing_if = "Option::is_none")]
    pub resource_functions: Option<DataManifestResourceFunctionsDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NonComplianceMessage {
    pub message: String,
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinitions {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinitionsValue {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<parameter_definitions_value::Type>,
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<serde_json::Value>,
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<parameter_definitions_value::Metadata>,
}
pub mod parameter_definitions_value {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        String,
        Array,
        Object,
        Boolean,
        Integer,
        Float,
        DateTime,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Metadata {
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(rename = "strongType", default, skip_serializing_if = "Option::is_none")]
        pub strong_type: Option<String>,
        #[serde(rename = "assignPermissions", default, skip_serializing_if = "Option::is_none")]
        pub assign_permissions: Option<bool>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterValues {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterValuesValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyAssignmentProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyAssignment>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentProperties {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "notScopes", default, skip_serializing_if = "Vec::is_empty")]
    pub not_scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValues>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "enforcementMode", default, skip_serializing_if = "Option::is_none")]
    pub enforcement_mode: Option<policy_assignment_properties::EnforcementMode>,
    #[serde(rename = "nonComplianceMessages", default, skip_serializing_if = "Vec::is_empty")]
    pub non_compliance_messages: Vec<NonComplianceMessage>,
}
pub mod policy_assignment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EnforcementMode {
        Default,
        DoNotEnforce,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyDefinitionProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyDefinitionGroup {
    pub name: String,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "additionalMetadataId", default, skip_serializing_if = "Option::is_none")]
    pub additional_metadata_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyDefinitionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyDefinition>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyDefinitionProperties {
    #[serde(rename = "policyType", default, skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<policy_definition_properties::PolicyType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "policyRule", default, skip_serializing_if = "Option::is_none")]
    pub policy_rule: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitions>,
}
pub mod policy_definition_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PolicyType {
        NotSpecified,
        BuiltIn,
        Custom,
        Static,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyDefinitionReference {
    #[serde(rename = "policyDefinitionId")]
    pub policy_definition_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValues>,
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[serde(rename = "groupNames", default, skip_serializing_if = "Vec::is_empty")]
    pub group_names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyExemption {
    pub properties: PolicyExemptionProperties,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyExemptionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyExemption>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyExemptionProperties {
    #[serde(rename = "policyAssignmentId")]
    pub policy_assignment_id: String,
    #[serde(rename = "policyDefinitionReferenceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_definition_reference_ids: Vec<String>,
    #[serde(rename = "exemptionCategory")]
    pub exemption_category: policy_exemption_properties::ExemptionCategory,
    #[serde(rename = "expiresOn", default, skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
pub mod policy_exemption_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExemptionCategory {
        Waiver,
        Mitigated,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicySetDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicySetDefinitionProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicySetDefinitionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicySetDefinition>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicySetDefinitionProperties {
    #[serde(rename = "policyType", default, skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<policy_set_definition_properties::PolicyType>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitions>,
    #[serde(rename = "policyDefinitions")]
    pub policy_definitions: Vec<PolicyDefinitionReference>,
    #[serde(rename = "policyDefinitionGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_definition_groups: Vec<PolicyDefinitionGroup>,
}
pub mod policy_set_definition_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PolicyType {
        NotSpecified,
        BuiltIn,
        Custom,
        Static,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceTypeAliases {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<Alias>,
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
