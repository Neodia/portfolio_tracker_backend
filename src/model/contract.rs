use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Contract(String);

impl Display for Contract {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Contract {
    fn from(s: &str) -> Self {
        Contract(s.to_string())
    }
}

impl From<String> for Contract {
    fn from(value: String) -> Self {
        Contract(value)
    }
}
