use super::{StakeData, StakeCalculation};

#[derive(Debug)]
struct Unstaking {
    unstake_info: StakeData,
}

impl Unstaking {
    pub fn new(unstake_info: StakeData) -> Self {
        Unstaking { unstake_info }
    }
}

impl StakeCalculation<Unstaking> for Unstaking {
    fn total(all_data: Vec<Unstaking>) -> u64 {
        all_data.iter().fold(0,|acc,x| acc + x.unstake_info.amount)
    }

    fn total_fee_cost(datas:Vec<Unstaking>) -> u64 {
        todo!()
    }    
}


#[cfg(test)]
mod test{

    #[test]
    fn get_total_unstake(){
        
    }
}
