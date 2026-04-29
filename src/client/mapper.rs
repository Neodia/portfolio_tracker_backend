use crate::client::cg_model::CGTokenData;
use crate::client::model::BlockchainAssetPrice;
use crate::model::Network;

pub fn map_cg_to_domain(
    cg_token_data: CGTokenData,
    network: Network,
) -> Option<BlockchainAssetPrice> {
    let price_usd = cg_token_data.attributes.price_usd?;
    Some(BlockchainAssetPrice::new(
        cg_token_data.attributes.symbol,
        cg_token_data.attributes.name,
        network,
        cg_token_data.attributes.address,
        price_usd,
    ))
}
