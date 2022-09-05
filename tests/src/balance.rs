// use std::error::Error;
// use rust_conflux_sdk::network::{CONFLUX_NETWORK, Network, set_network};
//
// #[tokio::test]
// async fn test_cfx_testnet() -> Result<(), Box<dyn Error>> {
//     set_network(Network::ConfluxTest, None);
//     println!("{}", CONFLUX_NETWORK.as_str());
//     // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
//     let data  = ConfluxClient.balance("cfxtest:aas53w71g7ahuztj66d9nzmjbz1s3gdceym5pn7gb9", None).await?;
//     println!("{:?}", data);
//     Ok(())
// }
//
// #[tokio::test]
// async fn test_cfx_mainnet() -> Result<(), Box<dyn Error>> {
//     set_network(Network::ConfluxMain, None);
//     println!("{}", CONFLUX_NETWORK.as_str());
//     // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
//     let data  = ConfluxClient.balance("cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz", None).await?;
//     let s = data.unwrap();
//     println!("{:?}", u64::from_str_radix(&s[2..], 16) );
//     Ok(())
// }
//
// #[tokio::test]
// async fn test_eth_mainnet() -> Result<(), Box<dyn Error>> {
//     set_network(Network::EthMain, None);
//     println!("{}", CONFLUX_NETWORK.as_str());
//     // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
//     let data  = ConfluxClient.balance("0x3304aEC3eD8060467af6eEb6F031F1B7c2E8C564", None).await?;
//     println!("{:?}", data);
//     Ok(())
// }
//
// #[tokio::test]
// async fn test_eth_testnet() -> Result<(), Box<dyn Error>> {
//     set_network(Network::EthTest, None);
//     println!("{}", CONFLUX_NETWORK.as_str());
//     // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
//     let data  = ConfluxClient.balance("0x2e0ac82E798C1C235585F27475949c35EEBf789D", None).await?;
//     println!("{:?}", data);
//     Ok(())
// }
