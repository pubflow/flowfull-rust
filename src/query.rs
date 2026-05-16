use reqwest::Method;
use serde::de::DeserializeOwned;

use crate::{
    Result,
    client::FlowfullClient,
    operators::{FilterOperator, format_value},
    request::RequestOptions,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    fn as_str(self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

#[derive(Clone)]
pub struct QueryBuilder {
    client: FlowfullClient,
    endpoint: String,
    filters: Vec<(String, FilterOperator)>,
    params: Vec<(String, String)>,
    sorts: Vec<(String, SortDirection)>,
    page: Option<u64>,
    limit: Option<u64>,
}

impl QueryBuilder {
    pub(crate) fn new(client: FlowfullClient, endpoint: String) -> Self {
        Self {
            client,
            endpoint,
            filters: Vec::new(),
            params: Vec::new(),
            sorts: Vec::new(),
            page: None,
            limit: None,
        }
    }

    pub fn where_(mut self, field: impl Into<String>, operator: FilterOperator) -> Self {
        self.filters.push((field.into(), operator));
        self
    }

    pub fn param(mut self, key: impl Into<String>, value: impl ToString) -> Self {
        self.params.push((key.into(), value.to_string()));
        self
    }

    pub fn sort(mut self, field: impl Into<String>, direction: SortDirection) -> Self {
        self.sorts.push((field.into(), direction));
        self
    }

    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn select<I, S>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let fields = fields
            .into_iter()
            .map(|field| field.as_ref().to_string())
            .collect::<Vec<_>>()
            .join(",");
        self.params.push(("fields".to_string(), fields));
        self
    }

    pub async fn get<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_options(RequestOptions::default()).await
    }

    pub async fn get_with_options<T>(self, mut options: RequestOptions) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.apply_query(&mut options);
        self.client
            .request_json(Method::GET, &self.endpoint, Option::<&()>::None, options)
            .await
    }

    fn apply_query(&self, options: &mut RequestOptions) {
        for (field, operator) in &self.filters {
            let key = if operator.operator.is_empty() || operator.operator == "eq" {
                field.clone()
            } else {
                format!("{field}[{}]", operator.operator)
            };
            options.query.push((key, format_value(&operator.value)));
        }

        for (key, value) in &self.params {
            options.query.push((key.clone(), value.clone()));
        }

        for (field, direction) in &self.sorts {
            options.query.push((
                "sort".to_string(),
                format!("{field}:{}", direction.as_str()),
            ));
        }

        if let Some(page) = self.page {
            options.query.push(("page".to_string(), page.to_string()));
        }

        if let Some(limit) = self.limit {
            options.query.push(("limit".to_string(), limit.to_string()));
        }
    }
}
