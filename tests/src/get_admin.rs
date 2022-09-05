// use std::error::Error;
// use rust_conflux_sdk::network::{CONFLUX_NETWORK, Network, set_network};
//
// #[tokio::test]
// async fn test_eth_testnet() -> Result<(), Box<dyn Error>> {
//     set_network(Network::ConfluxTest, None);
//     println!("{}", CONFLUX_NETWORK.as_str());
//     // let params = ConfluxParams::new("cfx_getBalance", vec!["cfx:aas53w71g7ahuztj66d9nzmjbz1s3gdceyau955pfz"]);
//     let data  = ConfluxClient.admin("cfxtest:acc963ada85ysj057t1xnfycwg6d81avfeus5pfwk9", None).await?;
//     println!("{:?}", data);
//     Ok(())
// }
