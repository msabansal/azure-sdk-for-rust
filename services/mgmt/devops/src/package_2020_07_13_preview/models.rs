#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(rename = "authorizationType")]
    pub authorization_type: authorization::AuthorizationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
pub mod authorization {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AuthorizationType {
        #[serde(rename = "personalAccessToken")]
        PersonalAccessToken,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BootstrapConfiguration {
    #[serde(rename = "sourceRepository", default, skip_serializing_if = "Option::is_none")]
    pub source_repository: Option<CodeRepository>,
    pub template: PipelineTemplate,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeRepository {
    #[serde(rename = "repositoryType")]
    pub repository_type: code_repository::RepositoryType,
    pub id: String,
    #[serde(rename = "defaultBranch")]
    pub default_branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
pub mod code_repository {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RepositoryType {
        #[serde(rename = "gitHub")]
        GitHub,
        #[serde(rename = "vstsGit")]
        VstsGit,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputDescriptor {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: input_descriptor::Type,
    #[serde(rename = "possibleValues", default, skip_serializing_if = "Vec::is_empty")]
    pub possible_values: Vec<InputValue>,
}
pub mod input_descriptor {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        String,
        SecureString,
        Int,
        Bool,
        Authorization,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "displayValue", default, skip_serializing_if = "Option::is_none")]
    pub display_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: PipelineProperties,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Pipeline>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineProperties {
    #[serde(rename = "pipelineId", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<i64>,
    #[serde(rename = "pipelineType")]
    pub pipeline_type: pipeline_properties::PipelineType,
    #[serde(rename = "bootstrapConfiguration")]
    pub bootstrap_configuration: BootstrapConfiguration,
}
pub mod pipeline_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PipelineType {
        #[serde(rename = "githubWorkflow")]
        GithubWorkflow,
        #[serde(rename = "azurePipeline")]
        AzurePipeline,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTemplate {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTemplateDefinition {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<InputDescriptor>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTemplateDefinitionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PipelineTemplateDefinition>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzurePipelineProperties {
    #[serde(flatten)]
    pub pipeline_properties: PipelineProperties,
    pub organization: OrganizationReference,
    pub project: ProjectReference,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GithubWorkflowProperties {
    #[serde(flatten)]
    pub pipeline_properties: PipelineProperties,
    pub repository: CodeRepository,
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
