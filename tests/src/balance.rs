use rust_conflux_sdk::cfx_space::cfx::Cfx;
use rust_conflux_sdk::eth_space::eth::Eth;
use rust_conflux_sdk::network::{set_network, Network, CONFLUX_NETWORK};
use std::error::Error;

#[tokio::test]
async fn test_cfx_testnet() -> Result<(), Box<dyn Error>> {
    set_network(Network::CfxTest, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
    let data = Cfx {}
        .balance("cfxtest:aas53w71g7ahuztj66d9nzmjbz1s3gdceym5pn7gb9", None)
        .await?;
    println!("{:?}", data);
    Ok(())
}

#[tokio::test]
async fn test_cfx_mainnet() -> Result<(), Box<dyn Error>> {
    set_network(Network::CfxMain, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
    let data = Cfx {}
        .balance("cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz", None)
        .await?;
    let s = data.unwrap();
    println!("{:?}", u64::from_str_radix(&s[2..], 16));
    Ok(())
}

#[tokio::test]
async fn test_eth_mainnet() -> Result<(), Box<dyn Error>> {
    set_network(Network::EthMain, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
    let data = Eth {}
        .balance("0x3304aEC3eD8060467af6eEb6F031F1B7c2E8C564")
        .await?;
    println!("{:?}", data);
    Ok(())
}

#[tokio::test]
async fn test_eth_testnet() -> Result<(), Box<dyn Error>> {
    set_network(Network::EthTest, None);
    println!("{}", CONFLUX_NETWORK.as_str());
    // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
    let data = Eth {}
        .balance("0x2e0ac82E798C1C235585F27475949c35EEBf789D")
        .await?;
    println!("{:?}", data);
    Ok(())
}
