use super::model::GetPricesFromNetworkResponse;
use crate::client::ClientError;
use crate::model::contract::Contract;
use crate::model::Network;
use std::future::Future;

pub trait CGClient {
    fn get_prices_from_network(
        &self,
        network: Network,
        contracts: Vec<Contract>,
    ) -> impl Future<Output = Result<GetPricesFromNetworkResponse, ClientError>>;
}
