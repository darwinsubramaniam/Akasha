use crate::chain::asset::Asset;


#[derive(Debug)]
pub struct Fee{
    amount: u64,
    asset: Asset,
}

#[derive(Debug)]
pub struct TransactionFee{
    start_network_fee: Fee,
    end_network_fee: Option<Fee>,
}