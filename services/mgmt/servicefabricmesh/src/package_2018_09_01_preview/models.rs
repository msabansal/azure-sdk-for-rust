#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddRemoveReplicaScalingMechanism {
    #[serde(flatten)]
    pub auto_scaling_mechanism: AutoScalingMechanism,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
    #[serde(rename = "minCount")]
    pub min_count: i64,
    #[serde(rename = "maxCount")]
    pub max_count: i64,
    #[serde(rename = "scaleIncrement")]
    pub scale_increment: i64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<ServiceResourceDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsDescription>,
    #[serde(rename = "debugParams", default, skip_serializing_if = "Option::is_none")]
    pub debug_params: Option<String>,
    #[serde(rename = "serviceNames", default, skip_serializing_if = "Vec::is_empty")]
    pub service_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[serde(rename = "unhealthyEvaluation", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluation: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: ApplicationResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationResourceDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApplicationResourceDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub application_properties: ApplicationProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationScopedVolume {
    #[serde(flatten)]
    pub volume_reference: VolumeReference,
    #[serde(rename = "creationParameters")]
    pub creation_parameters: ApplicationScopedVolumeCreationParameters,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationScopedVolumeCreationParameters {
    pub kind: ApplicationScopedVolumeKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationScopedVolumeCreationParametersServiceFabricVolumeDisk {
    #[serde(flatten)]
    pub application_scoped_volume_creation_parameters: ApplicationScopedVolumeCreationParameters,
    #[serde(rename = "sizeDisk")]
    pub size_disk: application_scoped_volume_creation_parameters_service_fabric_volume_disk::SizeDisk,
}
pub mod application_scoped_volume_creation_parameters_service_fabric_volume_disk {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SizeDisk {
        Small,
        Medium,
        Large,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApplicationScopedVolumeKind {
    ServiceFabricVolumeDisk,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScalingMechanism {
    pub kind: AutoScalingMechanismKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AutoScalingMechanismKind {
    AddRemoveReplica,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScalingMetric {
    pub kind: AutoScalingMetricKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AutoScalingMetricKind {
    Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScalingPolicy {
    pub name: String,
    pub trigger: AutoScalingTrigger,
    pub mechanism: AutoScalingMechanism,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScalingResourceMetric {
    #[serde(flatten)]
    pub auto_scaling_metric: AutoScalingMetric,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
    pub name: AutoScalingResourceMetricName,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AutoScalingResourceMetricName {
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "memoryInGB")]
    MemoryInGb,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoScalingTrigger {
    pub kind: AutoScalingTriggerKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AutoScalingTriggerKind {
    AverageLoad,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableOperationDisplay {
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
pub struct AverageLoadScalingTrigger {
    #[serde(flatten)]
    pub auto_scaling_trigger: AutoScalingTrigger,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
    pub metric: AutoScalingMetric,
    #[serde(rename = "lowerLoadThreshold")]
    pub lower_load_threshold: f64,
    #[serde(rename = "upperLoadThreshold")]
    pub upper_load_threshold: f64,
    #[serde(rename = "scaleIntervalInSeconds")]
    pub scale_interval_in_seconds: i64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureInternalMonitoringPipelineSinkDescription {
    #[serde(flatten)]
    pub diagnostics_sink_properties: DiagnosticsSinkProperties,
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "maConfigUrl", default, skip_serializing_if = "Option::is_none")]
    pub ma_config_url: Option<String>,
    #[serde(rename = "fluentdConfigUrl", default, skip_serializing_if = "Option::is_none")]
    pub fluentd_config_url: Option<serde_json::Value>,
    #[serde(rename = "autoKeyConfigUrl", default, skip_serializing_if = "Option::is_none")]
    pub auto_key_config_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerCodePackageProperties {
    pub name: String,
    pub image: String,
    #[serde(rename = "imageRegistryCredential", default, skip_serializing_if = "Option::is_none")]
    pub image_registry_credential: Option<ImageRegistryCredential>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<String>,
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentVariable>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub settings: Vec<Setting>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<ContainerLabel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<EndpointProperties>,
    pub resources: ResourceRequirements,
    #[serde(rename = "volumeRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_refs: Vec<VolumeReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub volumes: Vec<ApplicationScopedVolume>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsRef>,
    #[serde(rename = "reliableCollectionsRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub reliable_collections_refs: Vec<ReliableCollectionsRef>,
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<ContainerInstanceView>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "firstTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
    #[serde(rename = "lastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerInstanceView {
    #[serde(rename = "restartCount", default, skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i64>,
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<ContainerState>,
    #[serde(rename = "previousState", default, skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<ContainerState>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<ContainerEvent>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerLabel {
    pub name: String,
    pub value: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerLogs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<String>,
    #[serde(rename = "finishTime", default, skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    #[serde(rename = "detailStatus", default, skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsDescription {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sinks: Vec<DiagnosticsSinkProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "defaultSinkRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub default_sink_refs: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "sinkRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub sink_refs: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiagnosticsSinkKind {
    Invalid,
    AzureInternalMonitoringPipeline,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsSinkProperties {
    pub kind: DiagnosticsSinkKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointProperties {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetailsModel {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorErrorModel {
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetailsModel>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorModel {
    pub error: ErrorErrorModel,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayDestination {
    #[serde(rename = "applicationName")]
    pub application_name: String,
    #[serde(rename = "serviceName")]
    pub service_name: String,
    #[serde(rename = "endpointName")]
    pub endpoint_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "sourceNetwork")]
    pub source_network: NetworkRef,
    #[serde(rename = "destinationNetwork")]
    pub destination_network: NetworkRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tcp: Vec<TcpConfig>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub http: Vec<HttpConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: GatewayResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayResourceDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GatewayResourceDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub gateway_properties: GatewayProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HealthState {
    Invalid,
    Ok,
    Warning,
    Error,
    Unknown,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpConfig {
    pub name: String,
    pub port: i64,
    pub hosts: Vec<HttpHostConfig>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpHostConfig {
    pub name: String,
    pub routes: Vec<HttpRouteConfig>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRouteConfig {
    pub name: String,
    #[serde(rename = "match")]
    pub match_: HttpRouteMatchRule,
    pub destination: GatewayDestination,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRouteMatchHeader {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<http_route_match_header::Type>,
}
pub mod http_route_match_header {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "exact")]
        Exact,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRouteMatchPath {
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<String>,
    #[serde(rename = "type")]
    pub type_: http_route_match_path::Type,
}
pub mod http_route_match_path {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "prefix")]
        Prefix,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRouteMatchRule {
    pub path: HttpRouteMatchPath,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<HttpRouteMatchHeader>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageRegistryCredential {
    pub server: String,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlinedValueSecretResourceProperties {
    #[serde(flatten)]
    pub secret_resource_properties: SecretResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalNetworkResourceProperties {
    #[serde(flatten)]
    pub network_resource_properties: NetworkResourceProperties,
    #[serde(rename = "networkAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub network_address_prefix: Option<NetworkAddressPrefix>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedProxyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
pub type NetworkAddressPrefix = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NetworkKind {
    Local,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "endpointRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_refs: Vec<EndpointRef>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: NetworkResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkResourceDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkResourceDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkResourceProperties {
    #[serde(flatten)]
    pub network_resource_properties_base: NetworkResourcePropertiesBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkResourcePropertiesBase {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    pub kind: NetworkKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperatingSystemType {
    Linux,
    Windows,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResult>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<AvailableOperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisionedResourceProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReliableCollectionsRef {
    pub name: String,
    #[serde(rename = "doNotPersistState", default, skip_serializing_if = "Option::is_none")]
    pub do_not_persist_state: Option<bool>,
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
pub struct ResourceLimits {
    #[serde(rename = "memoryInGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_gb: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequests {
    #[serde(rename = "memoryInGB")]
    pub memory_in_gb: f64,
    pub cpu: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub requests: ResourceRequests,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceLimits>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceStatus {
    Unknown,
    Ready,
    Upgrading,
    Creating,
    Deleting,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SecretKind {
    #[serde(rename = "inlinedValue")]
    InlinedValue,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: SecretResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretResourceDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SecretResourceDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretResourceProperties {
    #[serde(flatten)]
    pub secret_resource_properties_base: SecretResourcePropertiesBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretResourcePropertiesBase {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    pub kind: SecretKind,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretValueProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretValueResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: SecretValueResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretValueResourceDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SecretValueResourceDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretValueResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub secret_value_properties: SecretValueProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "replicaCount", default, skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i64>,
    #[serde(rename = "autoScalingPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub auto_scaling_policies: Vec<AutoScalingPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<HealthState>,
    #[serde(rename = "unhealthyEvaluation", default, skip_serializing_if = "Option::is_none")]
    pub unhealthy_evaluation: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceReplicaDescription {
    #[serde(flatten)]
    pub service_replica_properties: ServiceReplicaProperties,
    #[serde(rename = "replicaName")]
    pub replica_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceReplicaDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceReplicaDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceReplicaProperties {
    #[serde(rename = "osType")]
    pub os_type: OperatingSystemType,
    #[serde(rename = "codePackages")]
    pub code_packages: Vec<ContainerCodePackageProperties>,
    #[serde(rename = "networkRefs", default, skip_serializing_if = "Vec::is_empty")]
    pub network_refs: Vec<NetworkRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsRef>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceDescription {
    #[serde(flatten)]
    pub managed_proxy_resource: ManagedProxyResource,
    pub properties: ServiceResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceResourceDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub service_replica_properties: ServiceReplicaProperties,
    #[serde(flatten)]
    pub service_properties: ServiceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Setting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TcpConfig {
    pub name: String,
    pub port: i64,
    pub destination: GatewayDestination,
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
pub struct VolumeProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceStatus>,
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    pub provider: VolumeProvider,
    #[serde(rename = "azureFileParameters", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_parameters: Option<VolumeProviderParametersAzureFile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VolumeProvider {
    #[serde(rename = "SFAzureFile")]
    SfAzureFile,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProviderParametersAzureFile {
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "accountKey", default, skip_serializing_if = "Option::is_none")]
    pub account_key: Option<String>,
    #[serde(rename = "shareName")]
    pub share_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeReference {
    pub name: String,
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "destinationPath")]
    pub destination_path: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeResourceDescription {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: VolumeResourceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeResourceDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VolumeResourceDescription>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeResourceProperties {
    #[serde(flatten)]
    pub provisioned_resource_properties: ProvisionedResourceProperties,
    #[serde(flatten)]
    pub volume_properties: VolumeProperties,
}
