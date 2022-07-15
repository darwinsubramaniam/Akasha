use super::network::Network;


#[derive(Debug)]
pub struct Asset{
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub origin_network: Network,
}