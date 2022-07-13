use crate::network::Network;

/// Transaction information will tell the direction of the flow.
/// It will also can help to track the fee of the transaction into the wallet if it is paid by the user's wallet.
#[derive(Debug)]
pub struct Transaction{
    id: String,
    timestamp: u64,
    fee: Option<u64>,
    transaction_type: TransactionType,
    // The network that the transaction was made from.
    start_network: Network,
    // The network that the transaction was ended to.
    // useful to track if it was bridged or via XCM
    end_network: Network,
}

#[derive(Debug)]
pub enum TransactionType{
    STAKE,
    UNSTAKE,
    TRANSFER,
    BURN,
    EXCHANGE,
    BRIDGE,
    XCM
}