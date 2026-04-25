use super::client::CGClient;
use super::model::GetPricesFromNetworkResponse;
use crate::client::cg_model::CGGetPricesFromNetworkResponse;
use crate::client::util::join_as_csv;
use crate::model::Network;
use crate::model::contract::Contract;
use crate::model::error::AppError;
use reqwest::Client;

pub struct LiveCGClient {
    base_url: String,
    cg_key: String,
    client: Client,
}

impl LiveCGClient {
    pub fn new(base_url: String, cg_key: String) -> LiveCGClient {
        Self {
            base_url,
            cg_key,
            client: Client::new(),
        }
    }
    fn get(&self, url: String) -> reqwest::RequestBuilder {
        self.client
            .get(url)
            .header("x-cg-demo-api-key", &self.cg_key)
    }
}

impl CGClient for LiveCGClient {
    async fn get_prices_from_network(
        &self,
        network: Network,
        contracts: Vec<Contract>,
    ) -> Result<GetPricesFromNetworkResponse, AppError> {
        let response = self
            .get(format!(
                "{}/onchain/networks/{}/tokens/multi/{}",
                self.base_url,
                network,
                join_as_csv(&contracts)
            ))
            .send()
            .await?
            .json::<CGGetPricesFromNetworkResponse>()
            .await?;

        Ok(response.into_domain(network))
    }
}
