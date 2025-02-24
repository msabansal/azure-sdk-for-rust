#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "A billing period resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPeriod {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the billing period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingPeriodProperties>,
}
impl BillingPeriod {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the billing period."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPeriodProperties {
    #[doc = "The start of the date range covered by the billing period."]
    #[serde(rename = "billingPeriodStartDate", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_start_date: Option<String>,
    #[doc = "The end of the date range covered by the billing period."]
    #[serde(rename = "billingPeriodEndDate", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_end_date: Option<String>,
    #[doc = "Array of invoice ids that associated with."]
    #[serde(rename = "invoiceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub invoice_ids: Vec<String>,
}
impl BillingPeriodProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing billing periods. It contains a list of available billing periods in reverse chronological order."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPeriodsListResult {
    #[doc = "The list of billing periods."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BillingPeriod>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl BillingPeriodsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A secure URL that can be used to download a PDF invoice until the URL expires."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadUrl {
    #[doc = "The time in UTC at which this download URL will expire."]
    #[serde(rename = "expiryTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    #[doc = "The URL to the PDF file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl DownloadUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An invoice resource can be used download a PDF version of an invoice."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Invoice {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InvoiceProperties>,
}
impl Invoice {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the invoice."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceProperties {
    #[doc = "A secure URL that can be used to download a PDF invoice until the URL expires."]
    #[serde(rename = "downloadUrl", default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<DownloadUrl>,
    #[doc = "The start of the date range covered by the invoice."]
    #[serde(rename = "invoicePeriodStartDate", default, skip_serializing_if = "Option::is_none")]
    pub invoice_period_start_date: Option<String>,
    #[doc = "The end of the date range covered by the invoice."]
    #[serde(rename = "invoicePeriodEndDate", default, skip_serializing_if = "Option::is_none")]
    pub invoice_period_end_date: Option<String>,
    #[doc = "Array of billing period ids that the invoice is attributed to."]
    #[serde(rename = "billingPeriodIds", default, skip_serializing_if = "Vec::is_empty")]
    pub billing_period_ids: Vec<String>,
}
impl InvoiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing invoices. It contains a list of available invoices in reverse chronological order."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoicesListResult {
    #[doc = "The list of invoices."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Invoice>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl InvoicesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Billing REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Billing."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Invoice, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result listing billing operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of billing operations supported by the Microsoft.Billing resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
