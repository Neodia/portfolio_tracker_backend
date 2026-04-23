use super::model::GetSimplePriceResponse;
use reqwest::Error;

pub trait CGClient {
    async fn get_simple_price(
        &self,
        ids: &[&str],
        vs_currencies: &[&str],
    ) -> Result<GetSimplePriceResponse, Error>;
}
