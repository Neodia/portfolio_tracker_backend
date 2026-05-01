use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter, strum::Display)]
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
impl Serialize for Network {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = s.serialize_struct("Network", 2)?;
        state.serialize_field("id", self.to_id())?;
        state.serialize_field("display_name", &self.to_string())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Network {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Network::from_id(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("unknown network id: {}", s)))
    }
}
