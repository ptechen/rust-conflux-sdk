use rust_conflux_sdk::cfx_space::cfx::Cfx;
use rust_conflux_sdk::network::{set_network, Network, CONFLUX_NETWORK};
use std::error::Error;

#[tokio::test]
async fn test_cfx_testnet() -> Result<(), Box<dyn Error>> {
    set_network(Network::CfxTest, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
    let data = Cfx {}.get_status().await?;
    println!("{:?}", data);
    Ok(())
}
