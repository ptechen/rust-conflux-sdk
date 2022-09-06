use rust_conflux_sdk::cfx_space::cfx::Cfx;
use rust_conflux_sdk::eth_space::eth::Eth;
use rust_conflux_sdk::network::{set_network, Network, CONFLUX_NETWORK};
use rust_conflux_sdk::error::CfxResult;

#[tokio::test]
async fn test_cfx_testnet() -> CfxResult<()> {
    set_network(Network::CfxTest, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    let data = Cfx {}
        .get_block_by_hash(
            "0x7b3cd0620f366cb136a5de9d1c22dc9a56cd23fb0619c79e6df3bc3448dd07fd",
            true
        )
        .await?;
    println!("{:?}", data);
    Ok(())
}

#[tokio::test]
async fn test_eth_testnet() -> CfxResult<()> {
    set_network(Network::EthTest, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    let data = Eth {}
        .get_block_by_hash(
            "0x2d4086e8fca0a4fb8b2a3d539009cdd50af8daa02fa5629d8843c7b62842f432",
            false
        )
        .await?;
    println!("{:?}", data);
    Ok(())
}
