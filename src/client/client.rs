use super::model::GetRatesFromNetworkResponse;
use crate::client::error::ClientError;
use crate::model::Contract;
use crate::model::Network;
use std::future::Future;

pub trait CGClient: Clone {
    fn get_rates_from_network(
        &self,
        network: Network,
        contracts: Vec<Contract>,
    ) -> impl Future<Output = Result<GetRatesFromNetworkResponse, ClientError>>;
}
