pub mod types;

pub use types::*;

use serde::de::DeserializeOwned;

use crate::{Result, client::FlowfullClient, request::RequestOptions};

#[derive(Clone)]
pub struct PaymentsClient {
    client: FlowfullClient,
}

impl PaymentsClient {
    pub(crate) fn new(client: FlowfullClient) -> Self {
        Self { client }
    }

    pub async fn create_intent(&self, data: CreatePaymentIntent) -> Result<PaymentIntent> {
        self.client
            .post("/bridge-payment/payments/intents", &data)
            .await
    }

    pub async fn get_intent(&self, id: impl AsRef<str>) -> Result<PaymentIntent> {
        self.client
            .get(&format!("/bridge-payment/payments/intents/{}", id.as_ref()))
            .await
    }

    pub async fn list_payments(&self, params: PageParams) -> Result<Vec<Payment>> {
        self.get_list("/bridge-payment/payments", params).await
    }

    pub async fn get_payment(&self, id: impl AsRef<str>) -> Result<Payment> {
        self.client
            .get(&format!("/bridge-payment/payments/{}", id.as_ref()))
            .await
    }

    pub async fn list_methods(&self, params: PageParams) -> Result<Vec<PaymentMethod>> {
        self.get_list("/bridge-payment/payment-methods", params)
            .await
    }

    pub async fn get_method(&self, id: impl AsRef<str>) -> Result<PaymentMethod> {
        self.client
            .get(&format!("/bridge-payment/payment-methods/{}", id.as_ref()))
            .await
    }

    pub async fn update_method(
        &self,
        id: impl AsRef<str>,
        data: UpdatePaymentMethod,
    ) -> Result<PaymentMethod> {
        self.client
            .put(
                &format!("/bridge-payment/payment-methods/{}", id.as_ref()),
                &data,
            )
            .await
    }

    pub async fn delete_method(&self, id: impl AsRef<str>) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .delete(&format!("/bridge-payment/payment-methods/{}", id.as_ref()))
            .await?;
        Ok(())
    }

    pub async fn create_address(&self, data: CreateAddress) -> Result<Address> {
        self.client.post("/bridge-payment/addresses", &data).await
    }

    pub async fn list_addresses(&self, params: PageParams) -> Result<Vec<Address>> {
        self.get_list("/bridge-payment/addresses", params).await
    }

    pub async fn get_address(&self, id: impl AsRef<str>) -> Result<Address> {
        self.client
            .get(&format!("/bridge-payment/addresses/{}", id.as_ref()))
            .await
    }

    pub async fn update_address(
        &self,
        id: impl AsRef<str>,
        data: UpdateAddress,
    ) -> Result<Address> {
        self.client
            .put(&format!("/bridge-payment/addresses/{}", id.as_ref()), &data)
            .await
    }

    pub async fn delete_address(&self, id: impl AsRef<str>) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .delete(&format!("/bridge-payment/addresses/{}", id.as_ref()))
            .await?;
        Ok(())
    }

    pub async fn create_customer(&self, data: CreateCustomer) -> Result<Customer> {
        self.client.post("/bridge-payment/customers", &data).await
    }

    pub async fn list_customers(&self, params: PageParams) -> Result<Vec<Customer>> {
        self.get_list("/bridge-payment/customers", params).await
    }

    pub async fn get_customer(&self, id: impl AsRef<str>) -> Result<Customer> {
        self.client
            .get(&format!("/bridge-payment/customers/{}", id.as_ref()))
            .await
    }

    pub async fn update_customer(
        &self,
        id: impl AsRef<str>,
        data: UpdateCustomer,
    ) -> Result<Customer> {
        self.client
            .put(&format!("/bridge-payment/customers/{}", id.as_ref()), &data)
            .await
    }

    pub async fn delete_customer(&self, id: impl AsRef<str>) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .delete(&format!("/bridge-payment/customers/{}", id.as_ref()))
            .await?;
        Ok(())
    }

    pub async fn create_subscription(&self, data: CreateSubscription) -> Result<Subscription> {
        self.client
            .post("/bridge-payment/subscriptions", &data)
            .await
    }

    pub async fn list_subscriptions(&self, params: PageParams) -> Result<Vec<Subscription>> {
        self.get_list("/bridge-payment/subscriptions", params).await
    }

    pub async fn get_subscription(&self, id: impl AsRef<str>) -> Result<Subscription> {
        self.client
            .get(&format!("/bridge-payment/subscriptions/{}", id.as_ref()))
            .await
    }

    pub async fn cancel_subscription(
        &self,
        id: impl AsRef<str>,
        data: CancelSubscription,
    ) -> Result<Subscription> {
        self.client
            .post(
                &format!("/bridge-payment/subscriptions/{}/cancel", id.as_ref()),
                &data,
            )
            .await
    }

    pub async fn create_organization(&self, data: CreateOrganization) -> Result<Organization> {
        self.client
            .post("/bridge-payment/organizations", &data)
            .await
    }

    pub async fn list_organizations(&self, params: PageParams) -> Result<Vec<Organization>> {
        self.get_list("/bridge-payment/organizations", params).await
    }

    pub async fn get_organization(&self, id: impl AsRef<str>) -> Result<Organization> {
        self.client
            .get(&format!("/bridge-payment/organizations/{}", id.as_ref()))
            .await
    }

    pub async fn update_organization(
        &self,
        id: impl AsRef<str>,
        data: UpdateOrganization,
    ) -> Result<Organization> {
        self.client
            .put(
                &format!("/bridge-payment/organizations/{}", id.as_ref()),
                &data,
            )
            .await
    }

    pub async fn delete_organization(&self, id: impl AsRef<str>) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .delete(&format!("/bridge-payment/organizations/{}", id.as_ref()))
            .await?;
        Ok(())
    }

    pub async fn list_org_members(
        &self,
        organization_id: impl AsRef<str>,
        params: PageParams,
    ) -> Result<Vec<OrganizationMember>> {
        self.get_list(
            &format!(
                "/bridge-payment/organizations/{}/members",
                organization_id.as_ref()
            ),
            params,
        )
        .await
    }

    pub async fn add_org_member(
        &self,
        organization_id: impl AsRef<str>,
        data: AddOrganizationMember,
    ) -> Result<OrganizationMember> {
        self.client
            .post(
                &format!(
                    "/bridge-payment/organizations/{}/members",
                    organization_id.as_ref()
                ),
                &data,
            )
            .await
    }

    pub async fn update_org_member_role(
        &self,
        organization_id: impl AsRef<str>,
        member_id: impl AsRef<str>,
        data: UpdateOrganizationMemberRole,
    ) -> Result<OrganizationMember> {
        self.client
            .put(
                &format!(
                    "/bridge-payment/organizations/{}/members/{}",
                    organization_id.as_ref(),
                    member_id.as_ref()
                ),
                &data,
            )
            .await
    }

    pub async fn remove_org_member(
        &self,
        organization_id: impl AsRef<str>,
        member_id: impl AsRef<str>,
    ) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .delete(&format!(
                "/bridge-payment/organizations/{}/members/{}",
                organization_id.as_ref(),
                member_id.as_ref()
            ))
            .await?;
        Ok(())
    }

    pub async fn leave_organization(&self, organization_id: impl AsRef<str>) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post(
                &format!(
                    "/bridge-payment/organizations/{}/leave",
                    organization_id.as_ref()
                ),
                &serde_json::json!({}),
            )
            .await?;
        Ok(())
    }

    pub async fn convert_guest(&self, data: ConvertGuest) -> Result<GuestConversionResult> {
        self.client
            .post("/bridge-payment/guest/convert", &data)
            .await
    }

    pub fn guest_options(guest_token: &str) -> Result<RequestOptions> {
        RequestOptions::new().header("X-Guest-Token", guest_token)
    }

    async fn get_list<T>(&self, endpoint: &str, params: PageParams) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let mut options = RequestOptions::default();
        if let Some(page) = params.page {
            options.query.push(("page".to_string(), page.to_string()));
        }
        if let Some(limit) = params.limit {
            options.query.push(("limit".to_string(), limit.to_string()));
        }
        self.client.get_with_options(endpoint, options).await
    }
}
