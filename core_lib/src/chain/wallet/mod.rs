pub struct Wallet{
    name: String,
    address: String,
}

impl Wallet{
    pub fn new(name: String, address: String) -> Self{
        Wallet{
            name,
            address,
        }
    }
}