use super::util::vec_to_csv_format;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct GetSimplePriceRequest {
    #[serde(serialize_with = "vec_to_csv_format")]
    ids: Vec<String>,
    #[serde(serialize_with = "vec_to_csv_format")]
    vs_currencies: Vec<String>,
}

impl GetSimplePriceRequest {
    pub fn new(ids: Vec<impl Into<String>>, vs_currencies: Vec<impl Into<String>>) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
            vs_currencies: vs_currencies.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetSimplePriceResponse(pub HashMap<String, HashMap<String, f64>>);
