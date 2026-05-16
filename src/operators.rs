use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct FilterOperator {
    pub operator: String,
    pub value: Value,
}

impl FilterOperator {
    pub fn new(operator: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            operator: operator.into(),
            value: value.into(),
        }
    }
}

pub fn eq(value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("eq", value)
}

pub fn ne(value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("ne", value)
}

pub fn gt(value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("gt", value)
}

pub fn gte(value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("gte", value)
}

pub fn lt(value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("lt", value)
}

pub fn lte(value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("lte", value)
}

pub fn like(value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("like", value)
}

pub fn ilike(value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("ilike", value)
}

pub fn contains(value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("contains", value)
}

pub fn starts_with(value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("startsWith", value)
}

pub fn ends_with(value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("endsWith", value)
}

pub fn in_op<I, V>(values: I) -> FilterOperator
where
    I: IntoIterator<Item = V>,
    V: Into<Value>,
{
    FilterOperator::new(
        "in",
        Value::Array(values.into_iter().map(Into::into).collect()),
    )
}

pub fn not_in<I, V>(values: I) -> FilterOperator
where
    I: IntoIterator<Item = V>,
    V: Into<Value>,
{
    FilterOperator::new(
        "nin",
        Value::Array(values.into_iter().map(Into::into).collect()),
    )
}

pub fn is_null() -> FilterOperator {
    FilterOperator::new("null", true)
}

pub fn not_null() -> FilterOperator {
    FilterOperator::new("notNull", true)
}

pub fn between(min: impl Into<Value>, max: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("between", Value::Array(vec![min.into(), max.into()]))
}

pub fn not_between(min: impl Into<Value>, max: impl Into<Value>) -> FilterOperator {
    FilterOperator::new("notBetween", Value::Array(vec![min.into(), max.into()]))
}

pub fn op(operator: impl Into<String>, value: impl Into<Value>) -> FilterOperator {
    FilterOperator::new(operator, value)
}

pub fn format_value(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => value.clone(),
        Value::Array(values) => values
            .iter()
            .map(format_value)
            .collect::<Vec<_>>()
            .join(","),
        Value::Object(_) => value.to_string(),
    }
}
