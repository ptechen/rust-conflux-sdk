use std::sync::atomic::Ordering;
use crate::error::CfxResult;
use crate::error::Error::RpcResError;
use crate::network::{CONFLUX_ENV, Network, NETWORK_ID};
use crate::traits::params::ConfluxParams;
use crate::traits::rpc_req::CommonRpcReq;
use crate::cfx_space::types::{CfxStatus, TransactionReceipt};

#[derive(Debug, Default)]
pub struct Cfx {}

impl CommonRpcReq for Cfx {}

impl Cfx {
    /// cfx_epochNumber
    /// Returns the epoch number corresponding to the given tag.
    /// Parameters:
    /// TAG - (optional, default: "latest_mined") String "latest_mined", "latest_state", "latest_confirmed", "latest_checkpoint" or "earliest", see the epoch number parameter.
    /// Returns:
    /// QUANTITY - the integer epoch number corresponding to the given tag.
    pub async fn epoch_number(&self, tag: Option<&str>) -> CfxResult<Option<String>> {
        let network_id = NETWORK_ID.load(Ordering::Relaxed);
        if network_id != Network::CfxTest as u16 && network_id != Network::CfxMain as u16 {
            return Err(RpcResError("epoch_number not supported eth".to_string()))
        }
        let tag = tag.unwrap_or("latest_mined");
        let params = ConfluxParams::new(
            1,
            &format!("{}_epochNumber", CONFLUX_ENV.as_str()),
            Some(vec![tag]),
        );
        self.post(&params).await
    }

    /// cfx_getBalance
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
    pub async fn balance(&self, address: &str, tag: Option<&str>) -> CfxResult<Option<String>> {
        let tag = tag.unwrap_or("latest_state");
        let params = ConfluxParams::new(
            1,
            &format!("{}_getBalance", CONFLUX_ENV.as_str()),
            Some(vec![address, tag]),
        );
        self.post(&params).await
    }

    /// cfx_getAdmin
    /// Returns the admin of the specified contract.
    ///
    /// Parameters:
    /// BASE32 - address of the contract.
    /// QUANTITY|TAG - (optional, default: "latest_state") integer epoch number,
    /// or the string "latest_state", "latest_confirmed", "latest_checkpoint"
    /// or "earliest", see the epoch number parameter
    ///
    /// Returns:
    /// BASE32 - address of admin, or null if the contract does not exist.
    pub async fn admin(&self, address: &str, tag: Option<&str>) -> CfxResult<Option<String>> {
        let tag = tag.unwrap_or("latest_state");
        let params = ConfluxParams::new(
            1,
            &format!("{}_getAdmin", CONFLUX_ENV.as_str()),
            Some(vec![address, tag]),
        );
        self.post(&params).await
    }

    pub async fn sponsor_info(&self) -> CfxResult<()> {
        Ok(())
    }

    pub async fn client_version(&self) -> CfxResult<Option<String>> {
        let params =
            ConfluxParams::new(1, &format!("{}_clientVersion", CONFLUX_ENV.as_str()), None);
        self.post(&params).await
    }

    pub async fn get_status(&self) -> CfxResult<Option<CfxStatus>> {
        let params = ConfluxParams::new(
            1,
            &format!("{}_getStatus", CONFLUX_ENV.as_str()),
            None,
        );
        self.post(&params).await
    }

    /// cfx_getTransactionReceipt
    pub async fn get_transaction_receipt(&self, tx_id: &str) -> CfxResult<Option<TransactionReceipt>> {
        let params = ConfluxParams::new(
            1,
            &format!("{}_getTransactionReceipt", CONFLUX_ENV.as_str()),
            Some(vec![tx_id]),
        );
        self.post(&params).await
    }
}
