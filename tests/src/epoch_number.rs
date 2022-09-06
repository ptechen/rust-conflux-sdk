// use std::error::Error;
// use rust_conflux_sdk::network::{CONFLUX_NETWORK, Network, set_network};
//
// #[tokio::test]
// async fn test_cfx_testnet() -> CfxResult<()> {
//     set_network(Network::ConfluxTest, None);
//     println!("{}", CONFLUX_NETWORK.as_str());
//     // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
//     let data  = ConfluxClient.epoch_number(None).await?;
//     let s = data.unwrap();
//     println!("{:?}", u64::from_str_radix(&s[2..], 16) );
//     Ok(())
// }
//
// #[tokio::test]
// async fn test_eth_testnet() -> CfxResult<()> {
//     set_network(Network::EthTest, None);
//     println!("{}", CONFLUX_NETWORK.as_str());
//     // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
//     let data  = ConfluxClient.epoch_number(None).await?;
//     println!("{:?}", data);
//     Ok(())
// }
