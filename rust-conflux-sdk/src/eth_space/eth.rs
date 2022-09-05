use serde_json::Value;
use crate::error::CfxResult;
use crate::network::CONFLUX_ENV;
use crate::traits::params::ConfluxParams;
use crate::traits::rpc_req::CommonRpcReq;

#[derive(Debug, Default)]
pub struct Eth {}

impl CommonRpcReq for Eth {}

impl Eth {
    /// eth_getBalance
    /// Returns the balance of the given account, identified by its address.
    ///
    /// Parameters:
    /// BASE32 - address to check for balance.
    /// QUANTITY|TAG - (optional, default: "latest_state") integer epoch number,
    /// or the string "latest_state", "latest_confirmed", "latest_checkpoint"
    /// or "earliest", see the epoch number parameter
    ///
    /// Returns:
    /// QUANTITY - integer of the current balance in Drip.
    pub async fn balance(&self, address: &str) -> CfxResult<Option<String>> {
        let params = ConfluxParams::new(
            1,
            &format!("{}_getBalance", CONFLUX_ENV.as_str()),
            Some(vec![address]),
        );
        self.post(&params).await
    }

    /// cfx_getTransactionReceipt
    pub async fn get_transaction_receipt(&self, tx_id: &str) -> CfxResult<Option<Value>> {
        let params = ConfluxParams::new(
            1,
            &format!("{}_getTransactionReceipt", CONFLUX_ENV.as_str()),
            Some(vec![tx_id]),
        );
        self.post(&params).await
    }
}
