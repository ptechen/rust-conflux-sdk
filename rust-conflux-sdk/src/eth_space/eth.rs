use crate::error::CfxResult;
use crate::eth_space::types::{BlockByHash, TransactionByHash};
use crate::network::CONFLUX_ENV;
use crate::traits::params::CfxParams;
use crate::traits::rpc_req::CommonRpcReq;
use serde_json::Value;

#[derive(Debug, Default)]
pub struct Eth {}

impl CommonRpcReq for Eth {}

impl Eth {
    /// eth_getBalance
    pub async fn balance(&self, address: &str) -> CfxResult<Option<String>> {
        let params = CfxParams::new(
            1,
            &format!("{}_getBalance", CONFLUX_ENV.as_str()),
            Some(vec![address]),
        );
        self.post(&params).await
    }

    /// eth_getTransactionReceipt
    pub async fn get_transaction_receipt(&self, tx_id: &str) -> CfxResult<Option<Value>> {
        let params = CfxParams::new(
            1,
            &format!("{}_getTransactionReceipt", CONFLUX_ENV.as_str()),
            Some(vec![tx_id]),
        );
        self.post(&params).await
    }

    /// eth_getTransactionByHash
    pub async fn get_transaction_by_hash(
        &self,
        tx_id: &str,
    ) -> CfxResult<Option<TransactionByHash>> {
        let params = CfxParams::new(
            1,
            &format!("{}_getTransactionByHash", CONFLUX_ENV.as_str()),
            Some(vec![tx_id]),
        );
        self.post(&params).await
    }

    /// eth_getBlockByHash
    pub async fn get_block_by_hash(&self, block_hash: &str, tag: bool) -> CfxResult<Option<BlockByHash>> {
        let params = CfxParams::new(
            1,
            &format!("{}_getBlockByHash", CONFLUX_ENV.as_str()),
            Some(vec![Value::from(block_hash), Value::from(tag)]),
        );
        self.post(&params).await
    }
}
