use crate::model::Symbol;
use crate::model::contract::Contract;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TokenOnChain {
    pub symbol: Symbol,
    pub contract: Contract,
}

impl TokenOnChain {
    pub fn new(symbol: Symbol, contract: Contract) -> Self {
        TokenOnChain { symbol, contract }
    }
}

impl Display for TokenOnChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.symbol, self.contract)
    }
}
