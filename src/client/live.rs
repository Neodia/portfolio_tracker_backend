use super::client::CGClient;
use super::model::{GetSimplePriceRequest, GetSimplePriceResponse};
use reqwest::{Client, Error};

pub struct LiveCGClient {
    base_url: String,
    cg_key: String,
    client: Client,
}

impl LiveCGClient {
    pub fn new(base_url: String, cg_key: String, client: Client) -> LiveCGClient {
        Self {
            base_url,
            cg_key,
            client,
        }
    }
}

impl CGClient for LiveCGClient {
    async fn get_simple_price(
        &self,
        ids: &[&str],
        vs_currencies: &[&str],
    ) -> Result<GetSimplePriceResponse, Error> {
        self.client
            .get(format!("{0}/simple/price", self.base_url))
            .query(&GetSimplePriceRequest::new(
                ids.to_vec(),
                vs_currencies.to_vec(),
            ))
            .header("x-cg-demo-api-key", self.cg_key.as_str())
            .send()
            .await?
            .json::<GetSimplePriceResponse>()
            .await
    }
}
