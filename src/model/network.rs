use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Network {
    Bitcoin,
    Ethereum,
    Solana,
    Base,
}

impl Network {
    pub fn to_id(&self) -> &'static str {
        match self {
            Network::Bitcoin => "bitcoin",
            Network::Ethereum => "ethereum",
            Network::Solana => "solana",
            Network::Base => "base",
        }
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_id())
    }
}