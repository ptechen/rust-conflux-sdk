use std::fmt::Debug;
use std::time::Duration;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use crate::error::CfxResult;
use crate::error::Error::RpcResError;
use crate::network::{CONFLUX_ENV, CONFLUX_NETWORK};
use crate::traits::params::ConfluxParams;

const TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Deserialize, Debug, Default)]
pub struct ConfluxRes<R> {
    pub id: Option<String>,
    pub jsonrpc: String,
    pub result: Option<R>,
    pub error: Option<CodeError>,
}

#[derive(Deserialize, Debug, Default)]
pub struct CodeError {
    pub code: i64,
    pub message: String,
}

#[async_trait]
pub trait CommonRpcReq
    where Self: Debug
{
    #[tracing::instrument]
    async fn post<T, R>(&self, params: &T) -> CfxResult<Option<R>>
        where
            T: Serialize + Debug + Sync,
            R: DeserializeOwned + Debug,
    {
        let client = Client::builder()
            .timeout(TIMEOUT)
            .build()?
            .post(CONFLUX_NETWORK.as_str())
            .header("Content-Type", "application/json")
            .json(params);
        let data = client.send().await?.json::<ConfluxRes<R>>().await?;
        if data.error.is_some() {
            let err = data.error.unwrap();
            return Err(RpcResError(format!(
                "code: {}, message: {}",
                err.code, err.message
            )));
        }
        Ok(data.result)
    }

    /// cfx/eth_gasPrice:
    /// Returns the current price per gas in Drip.
    ///
    /// Parameters:
    /// None.
    ///
    /// Returns:
    /// QUANTITY - integer of the current gas price in Drip.
    async fn gas_price(&self) -> CfxResult<Option<String>>
    {
        let params = ConfluxParams::new(
            1,
            &format!("{}_gasPrice", CONFLUX_ENV.as_str()),
            None,
        );
        self.post(&params).await
    }
}
