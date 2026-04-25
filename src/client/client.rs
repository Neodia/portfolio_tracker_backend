use super::model::GetPricesFromNetworkResponse;
use crate::model::Network;
use crate::model::contract::Contract;
use crate::model::error::AppError;

pub trait CGClient {
    async fn get_prices_from_network(
        &self,
        network: Network,
        contracts: Vec<Contract>,
    ) -> Result<GetPricesFromNetworkResponse, AppError>;
}
