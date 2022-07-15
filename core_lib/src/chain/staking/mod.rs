use super::{asset::Asset, transaction::Transaction};

mod stake;
mod unstake;


#[derive(Debug)]
pub struct StakeData{
    hash: String,
    timestamp: u64,
    asset: Asset,
    transaction: Transaction,
    amount: u64,
}

impl StakeData{
    pub fn new(hash: String, timestamp: u64, asset: Asset, transaction: Transaction, amount:u64) -> Self{
        StakeData{
            hash,
            timestamp,
            asset,
            transaction,
            amount,
        }
    }
}

trait StakeCalculation<T>{
    fn total(all_data:Vec<T>) -> u64;
    fn total_fee_cost(datas:Vec<T>) -> u64;
}


