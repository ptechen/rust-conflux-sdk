use std::error::Error;
use rust_conflux_sdk::cfx_space::cfx::Cfx;
use rust_conflux_sdk::eth_space::eth::Eth;
use rust_conflux_sdk::network::{CONFLUX_NETWORK, Network, set_network};
use rust_conflux_sdk::traits::rpc_req::CommonRpcReq;

#[tokio::test]
async fn test_cfx_gas_price_testnet() -> Result<(), Box<dyn Error>> {
    set_network(Network::CfxTest, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
    let data  = Cfx{}.gas_price().await?;
    println!("{:?}", data);
    Ok(())
}

#[tokio::test]
async fn test_eth_gas_price_testnet() -> Result<(), Box<dyn Error>> {
    set_network(Network::EthTest, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
    let data  = Eth{}.gas_price().await?;
    println!("{:?}", data);
    Ok(())
}
