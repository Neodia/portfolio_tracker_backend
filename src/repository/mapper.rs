use crate::repository::error::DBError;
use crate::repository::model::BlockchainAssetDTO;
use crate::model::Asset;
use crate::model::Network;

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
