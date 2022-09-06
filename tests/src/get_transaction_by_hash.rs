use rust_conflux_sdk::cfx_space::cfx::Cfx;
use rust_conflux_sdk::eth_space::eth::Eth;
use rust_conflux_sdk::network::{set_network, Network, CONFLUX_NETWORK};
use rust_conflux_sdk::error::CfxResult;

#[tokio::test]
async fn test_cfx_testnet() -> CfxResult<()> {
    set_network(Network::CfxTest, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
    let data = Cfx {}
        .get_transaction_by_hash(
            "0x21430b1706883a696ff0c96d71459e765923b9da0c18ec4706ff4828a2d8fbbb",
        )
        .await?;
    println!("{:?}", data);
    Ok(())
}

#[tokio::test]
async fn test_eth_testnet() -> CfxResult<()> {
    set_network(Network::EthTest, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
    let data = Eth {}
        .get_transaction_by_hash(
            "0xb42eae78f3a63d99c23676e9659a8a6cb504dcd4c0e70f54f5f7a96a711ad26e",
        )
        .await?;
    println!("{:?}", data);
    Ok(())
}
