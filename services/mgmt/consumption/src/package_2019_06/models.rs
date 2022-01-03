#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Balance {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BalanceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "beginningBalance", default, skip_serializing_if = "Option::is_none")]
    pub beginning_balance: Option<f64>,
    #[serde(rename = "endingBalance", default, skip_serializing_if = "Option::is_none")]
    pub ending_balance: Option<f64>,
    #[serde(rename = "newPurchases", default, skip_serializing_if = "Option::is_none")]
    pub new_purchases: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utilized: Option<f64>,
    #[serde(rename = "serviceOverage", default, skip_serializing_if = "Option::is_none")]
    pub service_overage: Option<f64>,
    #[serde(rename = "chargesBilledSeparately", default, skip_serializing_if = "Option::is_none")]
    pub charges_billed_separately: Option<f64>,
    #[serde(rename = "totalOverage", default, skip_serializing_if = "Option::is_none")]
    pub total_overage: Option<f64>,
    #[serde(rename = "totalUsage", default, skip_serializing_if = "Option::is_none")]
    pub total_usage: Option<f64>,
    #[serde(rename = "azureMarketplaceServiceCharges", default, skip_serializing_if = "Option::is_none")]
    pub azure_marketplace_service_charges: Option<f64>,
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<balance_properties::BillingFrequency>,
    #[serde(rename = "priceHidden", default, skip_serializing_if = "Option::is_none")]
    pub price_hidden: Option<bool>,
    #[serde(rename = "newPurchasesDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub new_purchases_details: Vec<serde_json::Value>,
    #[serde(rename = "adjustmentDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub adjustment_details: Vec<serde_json::Value>,
}
pub mod balance_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BillingFrequency {
        Month,
        Quarter,
        Year,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Budget {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BudgetProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetProperties {
    pub category: budget_properties::Category,
    pub amount: f64,
    #[serde(rename = "timeGrain")]
    pub time_grain: budget_properties::TimeGrain,
    #[serde(rename = "timePeriod")]
    pub time_period: BudgetTimePeriod,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    #[serde(rename = "currentSpend", default, skip_serializing_if = "Option::is_none")]
    pub current_spend: Option<CurrentSpend>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<serde_json::Value>,
}
pub mod budget_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Category {
        Cost,
        Usage,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeGrain {
        Monthly,
        Quarterly,
        Annually,
        BillingMonth,
        BillingQuarter,
        BillingAnnual,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetTimePeriod {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Budget>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargeSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ChargeSummaryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargeSummaryProperties {
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[serde(rename = "azureCharges", default, skip_serializing_if = "Option::is_none")]
    pub azure_charges: Option<f64>,
    #[serde(rename = "chargesBilledSeparately", default, skip_serializing_if = "Option::is_none")]
    pub charges_billed_separately: Option<f64>,
    #[serde(rename = "marketplaceCharges", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_charges: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargesListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ChargeSummary>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentSpend {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
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
pub struct Filters {
    #[serde(rename = "resourceGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_groups: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub meters: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Forecast {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ForecastProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastProperties {
    #[serde(rename = "usageDate", default, skip_serializing_if = "Option::is_none")]
    pub usage_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grain: Option<forecast_properties::Grain>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "chargeType", default, skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<forecast_properties::ChargeType>,
    #[serde(rename = "confidenceLevels", default, skip_serializing_if = "Vec::is_empty")]
    pub confidence_levels: Vec<serde_json::Value>,
}
pub mod forecast_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Grain {
        Daily,
        Monthly,
        Yearly,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChargeType {
        Actual,
        Forecast,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Forecast>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementGroupAggregatedCostProperties {
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[serde(rename = "azureCharges", default, skip_serializing_if = "Option::is_none")]
    pub azure_charges: Option<f64>,
    #[serde(rename = "marketplaceCharges", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_charges: Option<f64>,
    #[serde(rename = "chargesBilledSeparately", default, skip_serializing_if = "Option::is_none")]
    pub charges_billed_separately: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<ManagementGroupAggregatedCostResult>,
    #[serde(rename = "includedSubscriptions", default, skip_serializing_if = "Vec::is_empty")]
    pub included_subscriptions: Vec<String>,
    #[serde(rename = "excludedSubscriptions", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_subscriptions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementGroupAggregatedCostResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupAggregatedCostProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Marketplace {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MarketplaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceProperties {
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[serde(rename = "resourceRate", default, skip_serializing_if = "Option::is_none")]
    pub resource_rate: Option<f64>,
    #[serde(rename = "offerName", default, skip_serializing_if = "Option::is_none")]
    pub offer_name: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "orderNumber", default, skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "consumedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub consumed_quantity: Option<f64>,
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    #[serde(rename = "pretaxCost", default, skip_serializing_if = "Option::is_none")]
    pub pretax_cost: Option<f64>,
    #[serde(rename = "isEstimated", default, skip_serializing_if = "Option::is_none")]
    pub is_estimated: Option<bool>,
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "subscriptionGuid", default, skip_serializing_if = "Option::is_none")]
    pub subscription_guid: Option<String>,
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "departmentName", default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[serde(rename = "consumedService", default, skip_serializing_if = "Option::is_none")]
    pub consumed_service: Option<String>,
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<String>,
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[serde(rename = "planName", default, skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "isRecurringCharge", default, skip_serializing_if = "Option::is_none")]
    pub is_recurring_charge: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplacesListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Marketplace>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeterDetails {
    #[serde(rename = "meterName", default, skip_serializing_if = "Option::is_none")]
    pub meter_name: Option<String>,
    #[serde(rename = "meterCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_category: Option<String>,
    #[serde(rename = "meterSubCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_sub_category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "meterLocation", default, skip_serializing_if = "Option::is_none")]
    pub meter_location: Option<String>,
    #[serde(rename = "totalIncludedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub total_included_quantity: Option<f64>,
    #[serde(rename = "pretaxStandardRate", default, skip_serializing_if = "Option::is_none")]
    pub pretax_standard_rate: Option<f64>,
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "serviceTier", default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeterDetailsResponse {
    #[serde(rename = "meterName", default, skip_serializing_if = "Option::is_none")]
    pub meter_name: Option<String>,
    #[serde(rename = "meterCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_category: Option<String>,
    #[serde(rename = "meterSubCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_sub_category: Option<String>,
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    #[serde(rename = "serviceFamily", default, skip_serializing_if = "Option::is_none")]
    pub service_family: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub enabled: bool,
    pub operator: notification::Operator,
    pub threshold: f64,
    #[serde(rename = "contactEmails")]
    pub contact_emails: Vec<String>,
    #[serde(rename = "contactRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub contact_roles: Vec<String>,
    #[serde(rename = "contactGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub contact_groups: Vec<String>,
}
pub mod notification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        EqualTo,
        GreaterThan,
        GreaterThanOrEqualTo,
    }
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
pub struct PriceSheetModel {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pricesheets: Vec<PriceSheetProperties>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceSheetProperties {
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "meterDetails", default, skip_serializing_if = "Option::is_none")]
    pub meter_details: Option<MeterDetails>,
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    #[serde(rename = "includedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub included_quantity: Option<f64>,
    #[serde(rename = "partNumber", default, skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceSheetResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PriceSheetModel>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationDetail {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationDetailProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationDetailProperties {
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[serde(rename = "instanceFlexibilityRatio", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility_ratio: Option<String>,
    #[serde(rename = "instanceFlexibilityGroup", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility_group: Option<String>,
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[serde(rename = "reservedHours", default, skip_serializing_if = "Option::is_none")]
    pub reserved_hours: Option<f64>,
    #[serde(rename = "usageDate", default, skip_serializing_if = "Option::is_none")]
    pub usage_date: Option<String>,
    #[serde(rename = "usedHours", default, skip_serializing_if = "Option::is_none")]
    pub used_hours: Option<f64>,
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "totalReservedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub total_reserved_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationDetailsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationDetail>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationRecommendation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub resource_attributes: ResourceAttributes,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationRecommendationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationRecommendationProperties {
    #[serde(rename = "lookBackPeriod", default, skip_serializing_if = "Option::is_none")]
    pub look_back_period: Option<String>,
    #[serde(rename = "instanceFlexibilityRatio", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility_ratio: Option<i64>,
    #[serde(rename = "instanceFlexibilityGroup", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility_group: Option<String>,
    #[serde(rename = "normalizedSize", default, skip_serializing_if = "Option::is_none")]
    pub normalized_size: Option<String>,
    #[serde(rename = "recommendedQuantityNormalized", default, skip_serializing_if = "Option::is_none")]
    pub recommended_quantity_normalized: Option<f64>,
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[serde(rename = "costWithNoReservedInstances", default, skip_serializing_if = "Option::is_none")]
    pub cost_with_no_reserved_instances: Option<f64>,
    #[serde(rename = "recommendedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub recommended_quantity: Option<f64>,
    #[serde(rename = "totalCostWithReservedInstances", default, skip_serializing_if = "Option::is_none")]
    pub total_cost_with_reserved_instances: Option<f64>,
    #[serde(rename = "netSavings", default, skip_serializing_if = "Option::is_none")]
    pub net_savings: Option<f64>,
    #[serde(rename = "firstUsageDate", default, skip_serializing_if = "Option::is_none")]
    pub first_usage_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "skuProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub sku_properties: Vec<SkuProperty>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationRecommendationsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationRecommendation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationSummariesListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationSummary>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationSummaryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationSummaryProperties {
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[serde(rename = "reservedHours", default, skip_serializing_if = "Option::is_none")]
    pub reserved_hours: Option<f64>,
    #[serde(rename = "usageDate", default, skip_serializing_if = "Option::is_none")]
    pub usage_date: Option<String>,
    #[serde(rename = "usedHours", default, skip_serializing_if = "Option::is_none")]
    pub used_hours: Option<f64>,
    #[serde(rename = "minUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub min_utilization_percentage: Option<f64>,
    #[serde(rename = "avgUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub avg_utilization_percentage: Option<f64>,
    #[serde(rename = "maxUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub max_utilization_percentage: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "purchasedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub purchased_quantity: Option<f64>,
    #[serde(rename = "remainingQuantity", default, skip_serializing_if = "Option::is_none")]
    pub remaining_quantity: Option<f64>,
    #[serde(rename = "totalReservedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub total_reserved_quantity: Option<f64>,
    #[serde(rename = "usedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub used_quantity: Option<f64>,
    #[serde(rename = "utilizedPercentage", default, skip_serializing_if = "Option::is_none")]
    pub utilized_percentage: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationTransaction {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationTransactionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationTransactionProperties {
    #[serde(rename = "eventDate", default, skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "reservationOrderName", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_name: Option<String>,
    #[serde(rename = "purchasingEnrollment", default, skip_serializing_if = "Option::is_none")]
    pub purchasing_enrollment: Option<String>,
    #[serde(rename = "purchasingSubscriptionGuid", default, skip_serializing_if = "Option::is_none")]
    pub purchasing_subscription_guid: Option<String>,
    #[serde(rename = "purchasingSubscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub purchasing_subscription_name: Option<String>,
    #[serde(rename = "armSkuName", default, skip_serializing_if = "Option::is_none")]
    pub arm_sku_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "accountOwnerEmail", default, skip_serializing_if = "Option::is_none")]
    pub account_owner_email: Option<String>,
    #[serde(rename = "departmentName", default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(rename = "currentEnrollment", default, skip_serializing_if = "Option::is_none")]
    pub current_enrollment: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationTransactionsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationTransaction>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceAttributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsResult {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TagProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageDetail {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UsageDetailProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageDetailProperties {
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[serde(rename = "billingAccountName", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_name: Option<String>,
    #[serde(rename = "billingPeriodStartDate", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_start_date: Option<String>,
    #[serde(rename = "billingPeriodEndDate", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_end_date: Option<String>,
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[serde(rename = "billingProfileName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_name: Option<String>,
    #[serde(rename = "accountOwnerId", default, skip_serializing_if = "Option::is_none")]
    pub account_owner_id: Option<String>,
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "partNumber", default, skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "meterDetails", default, skip_serializing_if = "Option::is_none")]
    pub meter_details: Option<MeterDetailsResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(rename = "effectivePrice", default, skip_serializing_if = "Option::is_none")]
    pub effective_price: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
    #[serde(rename = "billingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "consumedService", default, skip_serializing_if = "Option::is_none")]
    pub consumed_service: Option<String>,
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "serviceInfo1", default, skip_serializing_if = "Option::is_none")]
    pub service_info1: Option<String>,
    #[serde(rename = "serviceInfo2", default, skip_serializing_if = "Option::is_none")]
    pub service_info2: Option<String>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[serde(rename = "invoiceSection", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section: Option<String>,
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "reservationName", default, skip_serializing_if = "Option::is_none")]
    pub reservation_name: Option<String>,
    #[serde(rename = "productOrderId", default, skip_serializing_if = "Option::is_none")]
    pub product_order_id: Option<String>,
    #[serde(rename = "productOrderName", default, skip_serializing_if = "Option::is_none")]
    pub product_order_name: Option<String>,
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(rename = "isAzureCreditEligible", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_credit_eligible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[serde(rename = "publisherType", default, skip_serializing_if = "Option::is_none")]
    pub publisher_type: Option<String>,
    #[serde(rename = "planName", default, skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "chargeType", default, skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageDetailsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageDetail>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
