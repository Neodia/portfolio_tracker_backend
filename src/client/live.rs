use super::client::CGClient;
use super::model::GetPricesFromNetworkResponse;
use crate::client::cg_model::CGGetPricesFromNetworkResponse;
use crate::client::error::ClientError;
use crate::client::util::{WithEnrichment, join_as_csv};
use crate::model::Contract;
use crate::model::Network;
use reqwest::{Client, StatusCode};

#[derive(Clone)]
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

    async fn get<
        Enrichment,
        Intermediate: WithEnrichment<Enrichment, Response> + serde::de::DeserializeOwned,
        Response,
    >(
        &self,
        url: String,
        enrichment: Enrichment,
    ) -> Result<Response, ClientError> {
        let response = self
            .client
            .get(&url)
            .header("x-cg-demo-api-key", &self.cg_key)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let response = response.json::<Intermediate>().await?;
                Ok(response.into_domain(enrichment))
            }
            StatusCode::UNAUTHORIZED => Err(ClientError::Unauthorized),
            StatusCode::NOT_FOUND => Err(ClientError::NotFound),
            StatusCode::TOO_MANY_REQUESTS => Err(ClientError::RateLimited),
            s => Err(ClientError::Unexpected(s.as_u16())),
        }
    }
}

impl CGClient for LiveCGClient {
    async fn get_prices_from_network(
        &self,
        network: Network,
        contracts: Vec<Contract>,
    ) -> Result<GetPricesFromNetworkResponse, ClientError> {
        self.get::<_, CGGetPricesFromNetworkResponse, _>(
            format!(
                "{}/onchain/networks/{}/tokens/multi/{}",
                self.base_url,
                network.to_id(),
                join_as_csv(&contracts)
            ),
            network,
        )
        .await
    }
}
