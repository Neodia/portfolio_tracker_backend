use crate::model::{BlockchainAsset, Network};
use crate::repository::DBError;
use crate::repository::model::BlockchainAssetDTO;

impl TryFrom<BlockchainAssetDTO> for BlockchainAsset {
    type Error = DBError;

    fn try_from(value: BlockchainAssetDTO) -> Result<Self, Self::Error> {
        let network = Network::from_id(value.chain.as_str())
            .ok_or(DBError::NetworkDeserializeError(value.chain))?;
        Ok(BlockchainAsset::new(
            value.ticker,
            network,
            value.contract_address,
        ))
    }
}
