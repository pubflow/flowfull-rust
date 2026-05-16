use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Metadata = serde_json::Map<String, Value>;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageParams {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuestData {
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePaymentIntent {
    pub total_cents: Option<i64>,
    pub subtotal_cents: Option<i64>,
    pub tax_cents: Option<i64>,
    pub discount_cents: Option<i64>,
    pub shipping_cents: Option<i64>,
    pub currency: String,
    pub provider_id: String,
    pub description: Option<String>,
    pub concept: Option<String>,
    pub reference_code: Option<String>,
    pub category: Option<String>,
    pub payment_method_id: Option<String>,
    pub save_payment_method: Option<bool>,
    pub guest_data: Option<GuestData>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentIntent {
    pub id: String,
    pub client_secret: Option<String>,
    pub status: Option<String>,
    pub total_cents: Option<i64>,
    pub currency: Option<String>,
    pub provider_id: Option<String>,
    pub user_id: Option<String>,
    pub guest_email: Option<String>,
    pub guest_token: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Payment {
    pub id: String,
    pub user_id: Option<String>,
    pub guest_email: Option<String>,
    pub provider_id: Option<String>,
    pub provider_payment_id: Option<String>,
    pub total_cents: Option<i64>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<Metadata>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentMethod {
    pub id: String,
    pub user_id: Option<String>,
    pub guest_email: Option<String>,
    pub organization_id: Option<String>,
    pub provider_id: Option<String>,
    pub provider_payment_method_id: Option<String>,
    #[serde(rename = "type")]
    pub method_type: Option<String>,
    pub last4: Option<String>,
    pub brand: Option<String>,
    pub exp_month: Option<u8>,
    pub exp_year: Option<u16>,
    pub alias: Option<String>,
    pub is_default: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdatePaymentMethod {
    pub alias: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateAddress {
    pub address_type: String,
    pub name: String,
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
    pub is_default: Option<bool>,
}

pub type UpdateAddress = CreateAddress;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub id: String,
    pub user_id: Option<String>,
    pub guest_email: Option<String>,
    pub organization_id: Option<String>,
    pub address_type: Option<String>,
    pub name: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub is_default: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCustomer {
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
    pub provider_id: Option<String>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateCustomer {
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Customer {
    pub id: String,
    pub user_id: Option<String>,
    pub guest_email: Option<String>,
    pub organization_id: Option<String>,
    pub provider_id: Option<String>,
    pub provider_customer_id: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub metadata: Option<Metadata>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSubscription {
    pub customer_id: String,
    pub payment_method_id: String,
    pub product_id: Option<String>,
    pub provider_id: Option<String>,
    pub trial_days: Option<u64>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Subscription {
    pub id: String,
    pub user_id: Option<String>,
    pub organization_id: Option<String>,
    pub customer_id: Option<String>,
    pub product_id: Option<String>,
    pub provider_id: Option<String>,
    pub provider_subscription_id: Option<String>,
    pub status: Option<String>,
    pub current_period_start: Option<DateTime<Utc>>,
    pub current_period_end: Option<DateTime<Utc>>,
    pub cancel_at_period_end: Option<bool>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub metadata: Option<Metadata>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CancelSubscription {
    pub cancel_at_period_end: Option<bool>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOrganization {
    pub name: String,
    pub business_email: Option<String>,
    pub business_phone: Option<String>,
    pub tax_id: Option<String>,
    pub address: Option<String>,
    pub metadata: Option<Metadata>,
}

pub type UpdateOrganization = CreateOrganization;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Organization {
    pub id: String,
    pub name: Option<String>,
    pub business_email: Option<String>,
    pub business_phone: Option<String>,
    pub tax_id: Option<String>,
    pub address: Option<String>,
    pub metadata: Option<Metadata>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrganizationMember {
    pub id: String,
    pub organization_id: Option<String>,
    pub user_id: Option<String>,
    pub role: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddOrganizationMember {
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateOrganizationMemberRole {
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConvertGuest {
    pub guest_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuestConversionResult {
    #[serde(default)]
    pub success: bool,
    pub message: Option<String>,
    pub payments_count: Option<u64>,
    pub payment_methods_count: Option<u64>,
    pub addresses_count: Option<u64>,
}
