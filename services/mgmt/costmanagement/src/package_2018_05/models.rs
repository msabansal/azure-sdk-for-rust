#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DimensionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DimensionProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DimensionsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Dimension>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
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
pub struct Query {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryColumn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryProperties {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<QueryColumn>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Vec<serde_json::Value>>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Query>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfig {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReportConfigProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigAggregation {
    pub name: String,
    pub function: report_config_aggregation::Function,
}
pub mod report_config_aggregation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Function {
        Sum,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReportConfigColumnType {
    Tag,
    Dimension,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigComparisonExpression {
    pub name: String,
    pub operator: report_config_comparison_expression::Operator,
    pub values: Vec<String>,
}
pub mod report_config_comparison_expression {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        In,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDataset {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<report_config_dataset::Granularity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ReportConfigDatasetConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grouping: Vec<ReportConfigGrouping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReportConfigFilter>,
}
pub mod report_config_dataset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Granularity {
        Daily,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDatasetConfiguration {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDefinition {
    #[serde(rename = "type")]
    pub type_: report_config_definition::Type,
    pub timeframe: report_config_definition::Timeframe,
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<ReportConfigTimePeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataset: Option<ReportConfigDataset>,
}
pub mod report_config_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        WeekToDate,
        MonthToDate,
        YearToDate,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDeliveryDestination {
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub container: String,
    #[serde(rename = "rootFolderPath", default, skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDeliveryInfo {
    pub destination: ReportConfigDeliveryDestination,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigFilter {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub and: Vec<ReportConfigFilter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub or: Vec<ReportConfigFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Box<Option<ReportConfigFilter>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<ReportConfigComparisonExpression>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<ReportConfigComparisonExpression>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigGrouping {
    #[serde(rename = "columnType")]
    pub column_type: ReportConfigColumnType,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReportConfig>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ReportConfigSchedule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<report_config_properties::Format>,
    #[serde(rename = "deliveryInfo")]
    pub delivery_info: ReportConfigDeliveryInfo,
    pub definition: ReportConfigDefinition,
}
pub mod report_config_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        Csv,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigRecurrencePeriod {
    pub from: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigSchedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<report_config_schedule::Status>,
    pub recurrence: report_config_schedule::Recurrence,
    #[serde(rename = "recurrencePeriod")]
    pub recurrence_period: ReportConfigRecurrencePeriod,
}
pub mod report_config_schedule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Active,
        Inactive,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Recurrence {
        Daily,
        Weekly,
        Monthly,
        Annually,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigTimePeriod {
    pub from: String,
    pub to: String,
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
    pub tags: Option<serde_json::Value>,
}
