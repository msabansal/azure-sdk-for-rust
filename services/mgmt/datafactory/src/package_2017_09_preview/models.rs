#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<ActivityDependency>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityDependency {
    pub activity: String,
    #[serde(rename = "dependencyConditions")]
    pub dependency_conditions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityRun {
    #[serde(rename = "pipelineName", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    #[serde(rename = "pipelineRunId", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_run_id: Option<String>,
    #[serde(rename = "activityName", default, skip_serializing_if = "Option::is_none")]
    pub activity_name: Option<String>,
    #[serde(rename = "activityType", default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
    #[serde(rename = "activityRunId", default, skip_serializing_if = "Option::is_none")]
    pub activity_run_id: Option<String>,
    #[serde(rename = "linkedServiceName", default, skip_serializing_if = "Option::is_none")]
    pub linked_service_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "activityRunStart", default, skip_serializing_if = "Option::is_none")]
    pub activity_run_start: Option<String>,
    #[serde(rename = "activityRunEnd", default, skip_serializing_if = "Option::is_none")]
    pub activity_run_end: Option<String>,
    #[serde(rename = "durationInMs", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityRunsListResponse {
    pub value: Vec<ActivityRun>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureKeyVaultSecretReference {
    #[serde(flatten)]
    pub secret_base: SecretBase,
    pub store: LinkedServiceReference,
    #[serde(rename = "secretName")]
    pub secret_name: serde_json::Value,
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRunResponse {
    #[serde(rename = "runId")]
    pub run_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structure: Option<serde_json::Value>,
    #[serde(rename = "linkedServiceName")]
    pub linked_service_name: LinkedServiceReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitionSpecification>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasetListResponse {
    pub value: Vec<DatasetResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasetReference {
    #[serde(rename = "type")]
    pub type_: dataset_reference::Type,
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValueSpecification>,
}
pub mod dataset_reference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        DatasetReference,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasetResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    pub properties: Dataset,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Expression {
    #[serde(rename = "type")]
    pub type_: expression::Type,
    pub value: String,
}
pub mod expression {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Expression,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Factory {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<FactoryIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FactoryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactoryIdentity {
    #[serde(rename = "type")]
    pub type_: factory_identity::Type,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
pub mod factory_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactoryListResponse {
    pub value: Vec<Factory>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactoryProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "vstsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub vsts_configuration: Option<FactoryVstsConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactoryRepoUpdate {
    #[serde(rename = "factoryResourceId", default, skip_serializing_if = "Option::is_none")]
    pub factory_resource_id: Option<String>,
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "vstsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub vsts_configuration: Option<FactoryVstsConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactoryUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<FactoryIdentity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactoryVstsConfiguration {
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "projectName", default, skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "repositoryName", default, skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "collaborationBranch", default, skip_serializing_if = "Option::is_none")]
    pub collaboration_branch: Option<String>,
    #[serde(rename = "rootFolder", default, skip_serializing_if = "Option::is_none")]
    pub root_folder: Option<String>,
    #[serde(rename = "lastCommitId", default, skip_serializing_if = "Option::is_none")]
    pub last_commit_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntime {
    #[serde(rename = "type")]
    pub type_: IntegrationRuntimeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeAuthKeys {
    #[serde(rename = "authKey1", default, skip_serializing_if = "Option::is_none")]
    pub auth_key1: Option<String>,
    #[serde(rename = "authKey2", default, skip_serializing_if = "Option::is_none")]
    pub auth_key2: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IntegrationRuntimeAutoUpdate {
    On,
    Off,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeConnectionInfo {
    #[serde(rename = "serviceToken", default, skip_serializing_if = "Option::is_none")]
    pub service_token: Option<String>,
    #[serde(rename = "identityCertThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub identity_cert_thumbprint: Option<String>,
    #[serde(rename = "hostServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub host_service_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "isIdentityCertExprired", default, skip_serializing_if = "Option::is_none")]
    pub is_identity_cert_exprired: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeListResponse {
    pub value: Vec<IntegrationRuntimeResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeMonitoringData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<IntegrationRuntimeNodeMonitoringData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeNodeIpAddress {
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeNodeMonitoringData {
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(rename = "availableMemoryInMB", default, skip_serializing_if = "Option::is_none")]
    pub available_memory_in_mb: Option<i64>,
    #[serde(rename = "cpuUtilization", default, skip_serializing_if = "Option::is_none")]
    pub cpu_utilization: Option<f64>,
    #[serde(rename = "concurrentJobsLimit", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_limit: Option<i64>,
    #[serde(rename = "concurrentJobsRunning", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_running: Option<i64>,
    #[serde(rename = "maxConcurrentJobs", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_jobs: Option<i64>,
    #[serde(rename = "sentBytes", default, skip_serializing_if = "Option::is_none")]
    pub sent_bytes: Option<f64>,
    #[serde(rename = "receivedBytes", default, skip_serializing_if = "Option::is_none")]
    pub received_bytes: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeReference {
    #[serde(rename = "type")]
    pub type_: integration_runtime_reference::Type,
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValueSpecification>,
}
pub mod integration_runtime_reference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        IntegrationRuntimeReference,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeRegenerateKeyParameters {
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<integration_runtime_regenerate_key_parameters::KeyName>,
}
pub mod integration_runtime_regenerate_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyName {
        #[serde(rename = "authKey1")]
        AuthKey1,
        #[serde(rename = "authKey2")]
        AuthKey2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeRemoveNodeRequest {
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    pub properties: IntegrationRuntime,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IntegrationRuntimeState {
    Initial,
    Stopped,
    Started,
    Starting,
    Stopping,
    NeedRegistration,
    Online,
    Limited,
    Offline,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeStatus {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<IntegrationRuntimeType>,
    #[serde(rename = "dataFactoryName", default, skip_serializing_if = "Option::is_none")]
    pub data_factory_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IntegrationRuntimeState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeStatusListResponse {
    pub value: Vec<IntegrationRuntimeStatusResponse>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeStatusResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub properties: IntegrationRuntimeStatus,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IntegrationRuntimeType {
    Managed,
    SelfHosted,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedService {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "connectVia", default, skip_serializing_if = "Option::is_none")]
    pub connect_via: Option<IntegrationRuntimeReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitionSpecification>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedServiceListResponse {
    pub value: Vec<LinkedServiceResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedServiceReference {
    #[serde(rename = "type")]
    pub type_: linked_service_reference::Type,
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValueSpecification>,
}
pub mod linked_service_reference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        LinkedServiceReference,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedServiceResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    pub properties: LinkedService,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationLogSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMetricAvailability {
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationMetricSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<String>,
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<OperationMetricAvailability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationProperties {
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<OperationServiceSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationServiceSpecification {
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<OperationLogSpecification>,
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<OperationMetricSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinitionSpecification {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterSpecification {
    #[serde(rename = "type")]
    pub type_: parameter_specification::Type,
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
}
pub mod parameter_specification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Object,
        String,
        Int,
        Float,
        Bool,
        Array,
        SecureString,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterValueSpecification {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activities: Vec<Activity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitionSpecification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineListResponse {
    pub value: Vec<PipelineResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineReference {
    #[serde(rename = "type")]
    pub type_: pipeline_reference::Type,
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub mod pipeline_reference {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        PipelineReference,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    pub properties: Pipeline,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineRun {
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "pipelineName", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "invokedBy", default, skip_serializing_if = "Option::is_none")]
    pub invoked_by: Option<PipelineRunInvokedBy>,
    #[serde(rename = "lastUpdated", default, skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "runStart", default, skip_serializing_if = "Option::is_none")]
    pub run_start: Option<String>,
    #[serde(rename = "runEnd", default, skip_serializing_if = "Option::is_none")]
    pub run_end: Option<String>,
    #[serde(rename = "durationInMs", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineRunFilterParameters {
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "lastUpdatedAfter")]
    pub last_updated_after: String,
    #[serde(rename = "lastUpdatedBefore")]
    pub last_updated_before: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<PipelineRunQueryFilter>,
    #[serde(rename = "orderBy", default, skip_serializing_if = "Vec::is_empty")]
    pub order_by: Vec<PipelineRunQueryOrderBy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineRunInvokedBy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineRunQueryFilter {
    pub operand: pipeline_run_query_filter::Operand,
    pub operator: pipeline_run_query_filter::Operator,
    pub values: Vec<String>,
}
pub mod pipeline_run_query_filter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operand {
        PipelineName,
        Status,
        RunStart,
        RunEnd,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        NotEquals,
        In,
        NotIn,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineRunQueryOrderBy {
    #[serde(rename = "orderBy")]
    pub order_by: pipeline_run_query_order_by::OrderBy,
    pub order: pipeline_run_query_order_by::Order,
}
pub mod pipeline_run_query_order_by {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OrderBy {
        RunStart,
        RunEnd,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Order {
        #[serde(rename = "ASC")]
        Asc,
        #[serde(rename = "DESC")]
        Desc,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineRunQueryResponse {
    pub value: Vec<PipelineRun>,
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretBase {
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecureString {
    #[serde(flatten)]
    pub secret_base: SecretBase,
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfHostedIntegrationRuntimeNode {
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[serde(rename = "hostServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub host_service_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<self_hosted_integration_runtime_node::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[serde(rename = "versionStatus", default, skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "registerTime", default, skip_serializing_if = "Option::is_none")]
    pub register_time: Option<String>,
    #[serde(rename = "lastConnectTime", default, skip_serializing_if = "Option::is_none")]
    pub last_connect_time: Option<String>,
    #[serde(rename = "expiryTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    #[serde(rename = "lastStartTime", default, skip_serializing_if = "Option::is_none")]
    pub last_start_time: Option<String>,
    #[serde(rename = "lastStopTime", default, skip_serializing_if = "Option::is_none")]
    pub last_stop_time: Option<String>,
    #[serde(rename = "lastUpdateResult", default, skip_serializing_if = "Option::is_none")]
    pub last_update_result: Option<self_hosted_integration_runtime_node::LastUpdateResult>,
    #[serde(rename = "lastStartUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_start_update_time: Option<String>,
    #[serde(rename = "lastEndUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_end_update_time: Option<String>,
    #[serde(rename = "isActiveDispatcher", default, skip_serializing_if = "Option::is_none")]
    pub is_active_dispatcher: Option<bool>,
    #[serde(rename = "concurrentJobsLimit", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_limit: Option<i64>,
    #[serde(rename = "maxConcurrentJobs", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_jobs: Option<i64>,
}
pub mod self_hosted_integration_runtime_node {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        NeedRegistration,
        Online,
        Limited,
        Offline,
        Upgrading,
        Initializing,
        InitializeFailed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastUpdateResult {
        Succeed,
        Fail,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trigger {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "runtimeState", default, skip_serializing_if = "Option::is_none")]
    pub runtime_state: Option<TriggerRuntimeState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerListResponse {
    pub value: Vec<TriggerResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerPipelineReference {
    #[serde(rename = "pipelineReference", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_reference: Option<PipelineReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValueSpecification>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    pub properties: Trigger,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerRun {
    #[serde(rename = "triggerRunId", default, skip_serializing_if = "Option::is_none")]
    pub trigger_run_id: Option<String>,
    #[serde(rename = "triggerName", default, skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    #[serde(rename = "triggerRunTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub trigger_run_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<trigger_run::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "triggeredPipelines", default, skip_serializing_if = "Option::is_none")]
    pub triggered_pipelines: Option<serde_json::Value>,
}
pub mod trigger_run {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Succeeded,
        Failed,
        Inprogress,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerRunListResponse {
    pub value: Vec<TriggerRun>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TriggerRuntimeState {
    Started,
    Stopped,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIntegrationRuntimeNodeRequest {
    #[serde(rename = "concurrentJobsLimit", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_limit: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIntegrationRuntimeRequest {
    #[serde(rename = "autoUpdate", default, skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<IntegrationRuntimeAutoUpdate>,
    #[serde(rename = "updateDelayOffset", default, skip_serializing_if = "Option::is_none")]
    pub update_delay_offset: Option<String>,
}
