use crate::model::{Asset, AssetAllocation};
use crate::model::{AssetHoldings, AssetRate, Holding, Network};
use crate::repository::error::DBError;
use crate::repository::model::{AssetAllocationDTO, BlockchainAssetDTO, HoldingDTO};

impl TryFrom<BlockchainAssetDTO> for Asset {
    type Error = DBError;

    fn try_from(value: BlockchainAssetDTO) -> Result<Self, Self::Error> {
        let network = Network::from_id(value.network.as_str())
            .ok_or(DBError::NetworkDeserializeError(value.network))?;
        Ok(Asset::new(
            value.id,
            value.symbol,
            value.name,
            network,
            value.contract_address,
        ))
    }
}

impl TryFrom<AssetAllocationDTO> for AssetAllocation {
    type Error = DBError;

    fn try_from(asset: AssetAllocationDTO) -> Result<Self, Self::Error> {
        let network = Network::from_id(asset.network.as_str())
            .ok_or(DBError::NetworkDeserializeError(asset.network))?;
        Ok(AssetAllocation::new(
            asset.id,
            asset.symbol,
            asset.name,
            network,
            asset.contract_address,
            asset.allocation,
        ))
    }
}

impl TryFrom<Vec<HoldingDTO>> for AssetHoldings {
    type Error = DBError;

    fn try_from(holdings: Vec<HoldingDTO>) -> Result<Self, Self::Error> {
        let first = &holdings[0]; // Safe to do because this always comes from a `group_map_by`
        let network = Network::from_id(first.network.as_str())
            .ok_or(DBError::NetworkDeserializeError(first.network.clone()))?;
        let asset = Asset::new(
            first.asset_id,
            first.symbol.clone(),
            first.name.clone(),
            network,
            first.contract_address.clone(),
        );
        let asset_rate = AssetRate::new(asset, first.rate_usd);
        let total_value_usd = holdings
            .iter()
            .map(|holding| holding.rate_usd * holding.amount)
            .sum();
        let holdings = holdings
            .into_iter()
            .map(|h| Holding::new(h.id, h.amount, h.amount * h.rate_usd, h.description))
            .collect();

        Ok(AssetHoldings::new(asset_rate, total_value_usd, holdings))
    }
}
