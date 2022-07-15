use crate::chain::network::Network;

#[derive(Debug)]
pub struct TransactionNetwork{
    start_network: Network,
    end_network: Network,
}