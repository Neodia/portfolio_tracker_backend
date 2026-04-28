use super::model::GetPricesFromNetworkResponse;
use crate::client::error::ClientError;
use crate::model::Contract;
use crate::model::Network;
use std::future::Future;

pub trait CGClient: Clone {
    fn get_prices_from_network(
        &self,
        network: Network,
        contracts: Vec<Contract>,
    ) -> impl Future<Output = Result<GetPricesFromNetworkResponse, ClientError>>;
}
