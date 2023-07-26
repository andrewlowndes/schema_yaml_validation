use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Example {
    pub prop: String,
    #[validate(range(min=2))]
    pub another: Option<u32>,
}

