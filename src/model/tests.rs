use super::network::*;

#[test]
fn network_from_known_id() {
    assert_eq!(Network::from_id("solana"), Some(Network::Solana));
}

#[test]
fn network_from_unknown_id() {
    assert_eq!(Network::from_id("unknown"), None);
}