#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureSku {
    pub name: azure_sku::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    pub tier: azure_sku::Tier,
}
pub mod azure_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_DS13_v2+1TB_PS")]
        StandardDs13V21tbPs,
        #[serde(rename = "Standard_DS13_v2+2TB_PS")]
        StandardDs13V22tbPs,
        #[serde(rename = "Standard_DS14_v2+3TB_PS")]
        StandardDs14V23tbPs,
        #[serde(rename = "Standard_DS14_v2+4TB_PS")]
        StandardDs14V24tbPs,
        #[serde(rename = "Standard_D13_v2")]
        StandardD13V2,
        #[serde(rename = "Standard_D14_v2")]
        StandardD14V2,
        #[serde(rename = "Standard_L8s")]
        StandardL8s,
        #[serde(rename = "Standard_L16s")]
        StandardL16s,
        #[serde(rename = "Standard_L8s_v2")]
        StandardL8sV2,
        #[serde(rename = "Standard_L16s_v2")]
        StandardL16sV2,
        #[serde(rename = "Standard_D11_v2")]
        StandardD11V2,
        #[serde(rename = "Standard_D12_v2")]
        StandardD12V2,
        #[serde(rename = "Standard_L4s")]
        StandardL4s,
        #[serde(rename = "Dev(No SLA)_Standard_D11_v2")]
        DevNoSlaStandardD11V2,
        #[serde(rename = "Standard_E64i_v3")]
        StandardE64iV3,
        #[serde(rename = "Standard_E80ids_v4")]
        StandardE80idsV4,
        #[serde(rename = "Standard_E2a_v4")]
        StandardE2aV4,
        #[serde(rename = "Standard_E4a_v4")]
        StandardE4aV4,
        #[serde(rename = "Standard_E8a_v4")]
        StandardE8aV4,
        #[serde(rename = "Standard_E16a_v4")]
        StandardE16aV4,
        #[serde(rename = "Standard_E8as_v4+1TB_PS")]
        StandardE8asV41tbPs,
        #[serde(rename = "Standard_E8as_v4+2TB_PS")]
        StandardE8asV42tbPs,
        #[serde(rename = "Standard_E16as_v4+3TB_PS")]
        StandardE16asV43tbPs,
        #[serde(rename = "Standard_E16as_v4+4TB_PS")]
        StandardE16asV44tbPs,
        #[serde(rename = "Dev(No SLA)_Standard_E2a_v4")]
        DevNoSlaStandardE2aV4,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlobStorageEventType {
    #[serde(rename = "Microsoft.Storage.BlobCreated")]
    MicrosoftStorageBlobCreated,
    #[serde(rename = "Microsoft.Storage.BlobRenamed")]
    MicrosoftStorageBlobRenamed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_result::Reason>,
}
pub mod check_name_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPrincipalAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterPrincipalProperties>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPrincipalAssignmentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClusterPrincipalAssignment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPrincipalProperties {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    pub role: cluster_principal_properties::Role,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "principalType")]
    pub principal_type: cluster_principal_properties::PrincipalType,
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
    #[serde(rename = "principalName", default, skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
}
pub mod cluster_principal_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        AllDatabasesAdmin,
        AllDatabasesViewer,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrincipalType {
        App,
        Group,
        User,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Compression {
    None,
    GZip,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub kind: data_connection::Kind,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
pub mod data_connection {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        EventHub,
        EventGrid,
        IotHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataConnectionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataConnection>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub kind: database::Kind,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
pub mod database {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        ReadWrite,
        ReadOnlyFollowing,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabasePrincipalAssignment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabasePrincipalProperties>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabasePrincipalAssignmentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabasePrincipalAssignment>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabasePrincipalProperties {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    pub role: database_principal_properties::Role,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "principalType")]
    pub principal_type: database_principal_properties::PrincipalType,
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
    #[serde(rename = "principalName", default, skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
}
pub mod database_principal_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        Admin,
        Ingestor,
        Monitor,
        User,
        UnrestrictedViewer,
        Viewer,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrincipalType {
        App,
        Group,
        User,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
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
pub struct EventGridConnectionProperties {
    #[serde(rename = "storageAccountResourceId")]
    pub storage_account_resource_id: String,
    #[serde(rename = "eventHubResourceId")]
    pub event_hub_resource_id: String,
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[serde(rename = "dataFormat", default, skip_serializing_if = "Option::is_none")]
    pub data_format: Option<EventGridDataFormat>,
    #[serde(rename = "ignoreFirstRecord", default, skip_serializing_if = "Option::is_none")]
    pub ignore_first_record: Option<bool>,
    #[serde(rename = "blobStorageEventType", default, skip_serializing_if = "Option::is_none")]
    pub blob_storage_event_type: Option<BlobStorageEventType>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGridDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventGridConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventGridDataFormat {
    #[serde(rename = "MULTIJSON")]
    Multijson,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "CSV")]
    Csv,
    #[serde(rename = "TSV")]
    Tsv,
    #[serde(rename = "SCSV")]
    Scsv,
    #[serde(rename = "SOHSV")]
    Sohsv,
    #[serde(rename = "PSV")]
    Psv,
    #[serde(rename = "TXT")]
    Txt,
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "SINGLEJSON")]
    Singlejson,
    #[serde(rename = "AVRO")]
    Avro,
    #[serde(rename = "TSVE")]
    Tsve,
    #[serde(rename = "PARQUET")]
    Parquet,
    #[serde(rename = "ORC")]
    Orc,
    #[serde(rename = "APACHEAVRO")]
    Apacheavro,
    #[serde(rename = "W3CLOGFILE")]
    W3clogfile,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubConnectionProperties {
    #[serde(rename = "eventHubResourceId")]
    pub event_hub_resource_id: String,
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[serde(rename = "dataFormat", default, skip_serializing_if = "Option::is_none")]
    pub data_format: Option<EventHubDataFormat>,
    #[serde(rename = "eventSystemProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub event_system_properties: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<Compression>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventHubConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventHubDataFormat {
    #[serde(rename = "MULTIJSON")]
    Multijson,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "CSV")]
    Csv,
    #[serde(rename = "TSV")]
    Tsv,
    #[serde(rename = "SCSV")]
    Scsv,
    #[serde(rename = "SOHSV")]
    Sohsv,
    #[serde(rename = "PSV")]
    Psv,
    #[serde(rename = "TXT")]
    Txt,
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "SINGLEJSON")]
    Singlejson,
    #[serde(rename = "AVRO")]
    Avro,
    #[serde(rename = "TSVE")]
    Tsve,
    #[serde(rename = "PARQUET")]
    Parquet,
    #[serde(rename = "ORC")]
    Orc,
    #[serde(rename = "APACHEAVRO")]
    Apacheavro,
    #[serde(rename = "W3CLOGFILE")]
    W3clogfile,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubConnectionProperties {
    #[serde(rename = "iotHubResourceId")]
    pub iot_hub_resource_id: String,
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "mappingRuleName", default, skip_serializing_if = "Option::is_none")]
    pub mapping_rule_name: Option<String>,
    #[serde(rename = "dataFormat", default, skip_serializing_if = "Option::is_none")]
    pub data_format: Option<IotHubDataFormat>,
    #[serde(rename = "eventSystemProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub event_system_properties: Vec<String>,
    #[serde(rename = "sharedAccessPolicyName")]
    pub shared_access_policy_name: String,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IotHubDataConnection {
    #[serde(flatten)]
    pub data_connection: DataConnection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IotHubConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IotHubDataFormat {
    #[serde(rename = "MULTIJSON")]
    Multijson,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "CSV")]
    Csv,
    #[serde(rename = "TSV")]
    Tsv,
    #[serde(rename = "SCSV")]
    Scsv,
    #[serde(rename = "SOHSV")]
    Sohsv,
    #[serde(rename = "PSV")]
    Psv,
    #[serde(rename = "TXT")]
    Txt,
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "SINGLEJSON")]
    Singlejson,
    #[serde(rename = "AVRO")]
    Avro,
    #[serde(rename = "TSVE")]
    Tsve,
    #[serde(rename = "PARQUET")]
    Parquet,
    #[serde(rename = "ORC")]
    Orc,
    #[serde(rename = "APACHEAVRO")]
    Apacheavro,
    #[serde(rename = "W3CLOGFILE")]
    W3clogfile,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub sku: AzureSku,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KustoPoolProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoPoolCheckNameRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: kusto_pool_check_name_request::Type,
}
pub mod kusto_pool_check_name_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Synapse/workspaces/kustoPools")]
        MicrosoftSynapseWorkspacesKustoPools,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoPoolListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<KustoPool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoPoolProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<kusto_pool_properties::State>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "dataIngestionUri", default, skip_serializing_if = "Option::is_none")]
    pub data_ingestion_uri: Option<String>,
    #[serde(rename = "stateReason", default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "engineType", default, skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<kusto_pool_properties::EngineType>,
    #[serde(rename = "workspaceUid", default, skip_serializing_if = "Option::is_none")]
    pub workspace_uid: Option<String>,
}
pub mod kusto_pool_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Creating,
        Unavailable,
        Running,
        Deleting,
        Deleted,
        Stopping,
        Stopped,
        Starting,
        Updating,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EngineType {
        V2,
        V3,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KustoPoolUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AzureSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KustoPoolProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
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
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadWriteDatabase {
    #[serde(flatten)]
    pub database: Database,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReadWriteDatabaseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadWriteDatabaseProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ResourceProvisioningState>,
    #[serde(rename = "softDeletePeriod", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_period: Option<String>,
    #[serde(rename = "hotCachePeriod", default, skip_serializing_if = "Option::is_none")]
    pub hot_cache_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DatabaseStatistics>,
    #[serde(rename = "isFollowed", default, skip_serializing_if = "Option::is_none")]
    pub is_followed: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceProvisioningState {
    Running,
    Creating,
    Deleting,
    Succeeded,
    Failed,
    Moving,
    Canceled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
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
