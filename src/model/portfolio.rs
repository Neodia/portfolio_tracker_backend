use serde::Serialize;
use crate::model::{AssetAllocation, PortfolioHoldings, PortfolioValueAt};

#[derive(Serialize, PartialEq, Debug)]
pub struct PortfolioResponse {
    pub expected_asset_allocations: Vec<AssetAllocation>,
    pub holdings: PortfolioHoldings,
    pub historical_portfolio_value: Vec<PortfolioValueAt>,
}
impl PortfolioResponse {
    pub fn new(
        expected_asset_allocations: Vec<AssetAllocation>,
        holdings: PortfolioHoldings,
        historical_portfolio_value: Vec<PortfolioValueAt>,
    ) -> Self {
        Self {
            expected_asset_allocations,
            holdings,
            historical_portfolio_value,
        }
    }
}
