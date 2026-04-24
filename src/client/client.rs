use super::model::{GetPricesFromNetworkResponse, GetSimplePriceResponse};
use crate::client::error::ClientError;
use crate::model::contract::Contract;
use crate::model::Network;

pub trait CGClient {
    async fn get_simple_price(
        &self,
        ids: &[&str],
        vs_currencies: &[&str],
    ) -> Result<GetSimplePriceResponse, ClientError>;

    async fn get_prices_from_network(&self, network: Network, contracts: &[Contract]) -> Result<GetPricesFromNetworkResponse, ClientError>;
}
