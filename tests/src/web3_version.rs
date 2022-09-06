use rust_conflux_sdk::cfx_space::cfx::Cfx;
use rust_conflux_sdk::network::{set_network, Network, CONFLUX_NETWORK};
use std::error::Error;

#[tokio::test]
async fn test_cfx_testnet_web3_client_version() -> Result<(), Box<dyn Error>> {
    set_network(Network::CfxTest, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    let data = Cfx {}.client_version().await?;
    println!("{:?}", data);
    Ok(())
}

#[tokio::test]
async fn test_custom_network() -> Result<(), Box<dyn Error>> {
    set_network(Network::CfxTest, Some("testtttt"));
    println!("{}", CONFLUX_NETWORK.as_str());
    Ok(())
}
