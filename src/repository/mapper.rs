use crate::model::{BlockchainAsset, Network};
use crate::repository::DBError;
use crate::repository::model::BlockchainAssetDTO;

impl TryFrom<BlockchainAssetDTO> for BlockchainAsset {
    type Error = DBError;

    fn try_from(value: BlockchainAssetDTO) -> Result<Self, Self::Error> {
        let network = Network::from_id(value.network.as_str())
            .ok_or(DBError::NetworkDeserializeError(value.network))?;
        Ok(BlockchainAsset::new(
            value.symbol,
            value.name,
            network,
            value.contract_address,
        ))
    }
}
