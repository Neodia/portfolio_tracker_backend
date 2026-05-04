mod cg_model;
pub mod error;
pub mod live;
mod mapper;
pub mod model;
#[cfg(test)]
mod tests;
mod util;

use crate::client::error::ClientError;
use crate::client::model::GetRatesFromNetworkResponse;
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
