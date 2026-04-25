use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum Network {
    Bitcoin,
    Ethereum,
    Solana,
    Base,
    Sonic,
    Sui,
    Mode,
    Aptos,
    Polygon,
    Linea,
    Avalanche,
}

impl Network {
    pub fn to_id(&self) -> &'static str {
        match self {
            Network::Bitcoin => "bitcoin",
            Network::Ethereum => "eth",
            Network::Solana => "solana",
            Network::Base => "base",
            Network::Sonic => "sonic",
            Network::Sui => "sui-network",
            Network::Mode => "mode",
            Network::Aptos => "aptos",
            Network::Polygon => "polygon_pos",
            Network::Linea => "linea",
            Network::Avalanche => "avax",
        }
    }

    pub fn from_id(id: &str) -> Option<Network> {
        Network::iter().find(|network| network.to_id() == id)
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_id())
    }
}
