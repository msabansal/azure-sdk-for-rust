#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkBatchJob {
    #[serde(rename = "livyInfo", default, skip_serializing_if = "Option::is_none")]
    pub livy_info: Option<SparkBatchJobState>,
    #[doc = "The batch name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The workspace name."]
    #[serde(rename = "workspaceName", default, skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
    #[doc = "The Spark pool name."]
    #[serde(rename = "sparkPoolName", default, skip_serializing_if = "Option::is_none")]
    pub spark_pool_name: Option<String>,
    #[doc = "The submitter name."]
    #[serde(rename = "submitterName", default, skip_serializing_if = "Option::is_none")]
    pub submitter_name: Option<String>,
    #[doc = "The submitter identifier."]
    #[serde(rename = "submitterId", default, skip_serializing_if = "Option::is_none")]
    pub submitter_id: Option<String>,
    #[doc = "The artifact identifier."]
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[doc = "The job type."]
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<spark_batch_job::JobType>,
    #[doc = "The Spark batch job result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<spark_batch_job::Result>,
    #[serde(rename = "schedulerInfo", default, skip_serializing_if = "Option::is_none")]
    pub scheduler_info: Option<SparkScheduler>,
    #[serde(rename = "pluginInfo", default, skip_serializing_if = "Option::is_none")]
    pub plugin_info: Option<SparkServicePlugin>,
    #[doc = "The error information."]
    #[serde(rename = "errorInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub error_info: Vec<SparkServiceError>,
    #[doc = "The tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The session Id."]
    pub id: i32,
    #[doc = "The application id of this session"]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "The detailed application info."]
    #[serde(rename = "appInfo", default, skip_serializing_if = "Option::is_none")]
    pub app_info: Option<serde_json::Value>,
    #[doc = "The batch state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The log lines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub log: Vec<String>,
}
impl SparkBatchJob {
    pub fn new(id: i32) -> Self {
        Self {
            livy_info: None,
            name: None,
            workspace_name: None,
            spark_pool_name: None,
            submitter_name: None,
            submitter_id: None,
            artifact_id: None,
            job_type: None,
            result: None,
            scheduler_info: None,
            plugin_info: None,
            error_info: Vec::new(),
            tags: None,
            id,
            app_id: None,
            app_info: None,
            state: None,
            log: Vec::new(),
        }
    }
}
pub mod spark_batch_job {
    use super::*;
    #[doc = "The job type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobType {
        SparkBatch,
        SparkSession,
    }
    #[doc = "The Spark batch job result."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        Uncertain,
        Succeeded,
        Failed,
        Cancelled,
    }
}
#[doc = "Response for batch list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkBatchJobCollection {
    #[doc = "The start index of fetched sessions."]
    pub from: i32,
    #[doc = "Number of sessions fetched."]
    pub total: i32,
    #[doc = "Batch list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sessions: Vec<SparkBatchJob>,
}
impl SparkBatchJobCollection {
    pub fn new(from: i32, total: i32) -> Self {
        Self {
            from,
            total,
            sessions: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkBatchJobOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    pub name: String,
    pub file: String,
    #[serde(rename = "className", default, skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jars: Vec<String>,
    #[serde(rename = "pyFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub py_files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub archives: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conf: Option<serde_json::Value>,
    #[serde(rename = "driverMemory", default, skip_serializing_if = "Option::is_none")]
    pub driver_memory: Option<String>,
    #[serde(rename = "driverCores", default, skip_serializing_if = "Option::is_none")]
    pub driver_cores: Option<i32>,
    #[serde(rename = "executorMemory", default, skip_serializing_if = "Option::is_none")]
    pub executor_memory: Option<String>,
    #[serde(rename = "executorCores", default, skip_serializing_if = "Option::is_none")]
    pub executor_cores: Option<i32>,
    #[serde(rename = "numExecutors", default, skip_serializing_if = "Option::is_none")]
    pub num_executors: Option<i32>,
}
impl SparkBatchJobOptions {
    pub fn new(name: String, file: String) -> Self {
        Self {
            tags: None,
            artifact_id: None,
            name,
            file,
            class_name: None,
            args: Vec::new(),
            jars: Vec::new(),
            py_files: Vec::new(),
            files: Vec::new(),
            archives: Vec::new(),
            conf: None,
            driver_memory: None,
            driver_cores: None,
            executor_memory: None,
            executor_cores: None,
            num_executors: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkBatchJobState {
    #[doc = "the time that at which \"not_started\" livy state was first seen."]
    #[serde(rename = "notStartedAt", default, skip_serializing_if = "Option::is_none")]
    pub not_started_at: Option<String>,
    #[doc = "the time that at which \"starting\" livy state was first seen."]
    #[serde(rename = "startingAt", default, skip_serializing_if = "Option::is_none")]
    pub starting_at: Option<String>,
    #[doc = "the time that at which \"running\" livy state was first seen."]
    #[serde(rename = "runningAt", default, skip_serializing_if = "Option::is_none")]
    pub running_at: Option<String>,
    #[doc = "time that at which \"dead\" livy state was first seen."]
    #[serde(rename = "deadAt", default, skip_serializing_if = "Option::is_none")]
    pub dead_at: Option<String>,
    #[doc = "the time that at which \"success\" livy state was first seen."]
    #[serde(rename = "successAt", default, skip_serializing_if = "Option::is_none")]
    pub success_at: Option<String>,
    #[doc = "the time that at which \"killed\" livy state was first seen."]
    #[serde(rename = "killedAt", default, skip_serializing_if = "Option::is_none")]
    pub killed_at: Option<String>,
    #[doc = "the time that at which \"recovering\" livy state was first seen."]
    #[serde(rename = "recoveringAt", default, skip_serializing_if = "Option::is_none")]
    pub recovering_at: Option<String>,
    #[doc = "the Spark job state."]
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "jobCreationRequest", default, skip_serializing_if = "Option::is_none")]
    pub job_creation_request: Option<SparkRequest>,
}
impl SparkBatchJobState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "className", default, skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jars: Vec<String>,
    #[serde(rename = "pyFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub py_files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub archives: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conf: Option<serde_json::Value>,
    #[serde(rename = "driverMemory", default, skip_serializing_if = "Option::is_none")]
    pub driver_memory: Option<String>,
    #[serde(rename = "driverCores", default, skip_serializing_if = "Option::is_none")]
    pub driver_cores: Option<i32>,
    #[serde(rename = "executorMemory", default, skip_serializing_if = "Option::is_none")]
    pub executor_memory: Option<String>,
    #[serde(rename = "executorCores", default, skip_serializing_if = "Option::is_none")]
    pub executor_cores: Option<i32>,
    #[serde(rename = "numExecutors", default, skip_serializing_if = "Option::is_none")]
    pub num_executors: Option<i32>,
}
impl SparkRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkScheduler {
    #[serde(rename = "submittedAt", default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
    #[serde(rename = "scheduledAt", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(rename = "endedAt", default, skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(rename = "cancellationRequestedAt", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_requested_at: Option<String>,
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<spark_scheduler::CurrentState>,
}
impl SparkScheduler {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spark_scheduler {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CurrentState {
        Queued,
        Scheduled,
        Ended,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkServiceError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<spark_service_error::Source>,
}
impl SparkServiceError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spark_service_error {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        System,
        User,
        Unknown,
        Dependency,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkServicePlugin {
    #[serde(rename = "preparationStartedAt", default, skip_serializing_if = "Option::is_none")]
    pub preparation_started_at: Option<String>,
    #[serde(rename = "resourceAcquisitionStartedAt", default, skip_serializing_if = "Option::is_none")]
    pub resource_acquisition_started_at: Option<String>,
    #[serde(rename = "submissionStartedAt", default, skip_serializing_if = "Option::is_none")]
    pub submission_started_at: Option<String>,
    #[serde(rename = "monitoringStartedAt", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_started_at: Option<String>,
    #[serde(rename = "cleanupStartedAt", default, skip_serializing_if = "Option::is_none")]
    pub cleanup_started_at: Option<String>,
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<spark_service_plugin::CurrentState>,
}
impl SparkServicePlugin {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spark_service_plugin {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CurrentState {
        Preparation,
        ResourceAcquisition,
        Queued,
        Submission,
        Monitoring,
        Cleanup,
        Ended,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkSession {
    #[serde(rename = "livyInfo", default, skip_serializing_if = "Option::is_none")]
    pub livy_info: Option<SparkSessionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "workspaceName", default, skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
    #[serde(rename = "sparkPoolName", default, skip_serializing_if = "Option::is_none")]
    pub spark_pool_name: Option<String>,
    #[serde(rename = "submitterName", default, skip_serializing_if = "Option::is_none")]
    pub submitter_name: Option<String>,
    #[serde(rename = "submitterId", default, skip_serializing_if = "Option::is_none")]
    pub submitter_id: Option<String>,
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<spark_session::JobType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<spark_session::Result>,
    #[serde(rename = "schedulerInfo", default, skip_serializing_if = "Option::is_none")]
    pub scheduler_info: Option<SparkScheduler>,
    #[serde(rename = "pluginInfo", default, skip_serializing_if = "Option::is_none")]
    pub plugin_info: Option<SparkServicePlugin>,
    #[serde(rename = "errorInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub error_info: Vec<SparkServiceError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub id: i32,
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "appInfo", default, skip_serializing_if = "Option::is_none")]
    pub app_info: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub log: Vec<String>,
}
impl SparkSession {
    pub fn new(id: i32) -> Self {
        Self {
            livy_info: None,
            name: None,
            workspace_name: None,
            spark_pool_name: None,
            submitter_name: None,
            submitter_id: None,
            artifact_id: None,
            job_type: None,
            result: None,
            scheduler_info: None,
            plugin_info: None,
            error_info: Vec::new(),
            tags: None,
            id,
            app_id: None,
            app_info: None,
            state: None,
            log: Vec::new(),
        }
    }
}
pub mod spark_session {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobType {
        SparkBatch,
        SparkSession,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        Uncertain,
        Succeeded,
        Failed,
        Cancelled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkSessionCollection {
    pub from: i32,
    pub total: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sessions: Vec<SparkSession>,
}
impl SparkSessionCollection {
    pub fn new(from: i32, total: i32) -> Self {
        Self {
            from,
            total,
            sessions: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkSessionOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "className", default, skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jars: Vec<String>,
    #[serde(rename = "pyFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub py_files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub archives: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conf: Option<serde_json::Value>,
    #[serde(rename = "driverMemory", default, skip_serializing_if = "Option::is_none")]
    pub driver_memory: Option<String>,
    #[serde(rename = "driverCores", default, skip_serializing_if = "Option::is_none")]
    pub driver_cores: Option<i32>,
    #[serde(rename = "executorMemory", default, skip_serializing_if = "Option::is_none")]
    pub executor_memory: Option<String>,
    #[serde(rename = "executorCores", default, skip_serializing_if = "Option::is_none")]
    pub executor_cores: Option<i32>,
    #[serde(rename = "numExecutors", default, skip_serializing_if = "Option::is_none")]
    pub num_executors: Option<i32>,
}
impl SparkSessionOptions {
    pub fn new(name: String) -> Self {
        Self {
            tags: None,
            artifact_id: None,
            name,
            file: None,
            class_name: None,
            args: Vec::new(),
            jars: Vec::new(),
            py_files: Vec::new(),
            files: Vec::new(),
            archives: Vec::new(),
            conf: None,
            driver_memory: None,
            driver_cores: None,
            executor_memory: None,
            executor_cores: None,
            num_executors: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkSessionState {
    #[serde(rename = "notStartedAt", default, skip_serializing_if = "Option::is_none")]
    pub not_started_at: Option<String>,
    #[serde(rename = "startingAt", default, skip_serializing_if = "Option::is_none")]
    pub starting_at: Option<String>,
    #[serde(rename = "idleAt", default, skip_serializing_if = "Option::is_none")]
    pub idle_at: Option<String>,
    #[serde(rename = "deadAt", default, skip_serializing_if = "Option::is_none")]
    pub dead_at: Option<String>,
    #[serde(rename = "shuttingDownAt", default, skip_serializing_if = "Option::is_none")]
    pub shutting_down_at: Option<String>,
    #[serde(rename = "killedAt", default, skip_serializing_if = "Option::is_none")]
    pub killed_at: Option<String>,
    #[serde(rename = "recoveringAt", default, skip_serializing_if = "Option::is_none")]
    pub recovering_at: Option<String>,
    #[serde(rename = "busyAt", default, skip_serializing_if = "Option::is_none")]
    pub busy_at: Option<String>,
    #[serde(rename = "errorAt", default, skip_serializing_if = "Option::is_none")]
    pub error_at: Option<String>,
    #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "jobCreationRequest", default, skip_serializing_if = "Option::is_none")]
    pub job_creation_request: Option<SparkRequest>,
}
impl SparkSessionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkStatement {
    pub id: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<SparkStatementOutput>,
}
impl SparkStatement {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            code: None,
            state: None,
            output: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkStatementCancellationResult {
    #[doc = "The msg property from the Livy API. The value is always \"canceled\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}
impl SparkStatementCancellationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkStatementCollection {
    pub total_statements: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statements: Vec<SparkStatement>,
}
impl SparkStatementCollection {
    pub fn new(total_statements: i32) -> Self {
        Self {
            total_statements,
            statements: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkStatementOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<spark_statement_options::Kind>,
}
impl SparkStatementOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spark_statement_options {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "spark")]
        Spark,
        #[serde(rename = "pyspark")]
        Pyspark,
        #[serde(rename = "dotnetspark")]
        Dotnetspark,
        #[serde(rename = "sql")]
        Sql,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SparkStatementOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub execution_count: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evalue: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub traceback: Vec<String>,
}
impl SparkStatementOutput {
    pub fn new(execution_count: i32) -> Self {
        Self {
            status: None,
            execution_count,
            data: None,
            ename: None,
            evalue: None,
            traceback: Vec::new(),
        }
    }
}
