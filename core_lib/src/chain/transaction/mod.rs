use self::{network::TransactionNetwork, fee::TransactionFee};

mod fee;
mod network;

#[derive(Debug)]
pub enum TransactionType{
    STAKE,
    UNSTAKE,
    CHILL,
    BOND,
    TRANSFER_IN,
    TRANSFER_OUT,
    BURN,
    EXCHANGE,
    BRIDGE,
    XCM,
    OTHER(String)
}


/// Each Action done in the blockchain is a form of Transaction.
#[derive(Debug)]
pub struct Transaction{
    hash: String,
    timestamp: u64,
    transaction_type: TransactionType,
    network_transacted: TransactionNetwork,
    network_fees: TransactionFee
}

