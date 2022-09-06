use crate::cfx_space::types::{BlockByHash, CfxStatus, TransactionByHash, TransactionReceipt};
use crate::error::CfxResult;
use crate::error::Error::RpcResError;
use crate::network::{Network, CONFLUX_ENV, NETWORK_ID};
use crate::traits::params::CfxParams;
use crate::traits::rpc_req::CommonRpcReq;
use std::sync::atomic::Ordering;
use serde_json::Value;

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
            return Err(RpcResError("epoch_number not supported eth".to_string()));
        }
        let tag = tag.unwrap_or("latest_mined");
        let params = CfxParams::new(
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
        let params = CfxParams::new(
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
        let params = CfxParams::new(
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
        let params:CfxParams<Value> =
            CfxParams::new(1, &format!("{}_clientVersion", CONFLUX_ENV.as_str()), None);
        self.post(&params).await
    }

    pub async fn get_status(&self) -> CfxResult<Option<CfxStatus>> {
        let params:CfxParams<Value>  = CfxParams::new(1, &format!("{}_getStatus", CONFLUX_ENV.as_str()), None);
        self.post(&params).await
    }

    /// cfx_getTransactionReceipt
    /// Returns a transaction receipt, identified by the corresponding transaction hash.
    ///
    /// Parameters:
    /// DATA, 32 Bytes - hash of a transaction
    ///
    /// Returns:
    /// Object - a transaction receipt object, or null when no transaction was found or the transaction was not executed yet:
    ///
    /// transactionHash: DATA, 32 Bytes - hash of the given transaction.
    /// index: QUANTITY - transaction index within the block.
    /// blockHash: DATA, 32 Bytes - hash of the block where this transaction was in and got executed.
    /// epochNumber: QUANTITY - epoch number of the block where this transaction was in and got executed.
    /// from: BASE32 - address of the sender.
    /// to: BASE32 - address of the receiver. null when it is a contract deployment transaction.
    /// gasUsed: QUANTITY - gas used for executing the transaction.
    /// gasFee: QUANTITY - gas charged to the sender's account. If the provided gas (gas limit) is larger than the gas used, at most 1/4 of it is refunded.
    /// gasCoveredBySponsor: Boolean, true if this transaction's gas fee was covered by the sponsor.
    /// storageCollateralized: QUANTITY, the amount of storage collateral this transaction required.
    /// storageCoveredBySponsor: Boolean, true if this transaction's storage collateral was covered by the sponsor.
    /// storageReleased: Array, array of storage change objects, each specifying an address and the corresponding amount of storage collateral released, e.g., [{ 'address': 'CFX:TYPE.USER:AARC9ABYCUE0HHZGYRR53M6CXEDGCCRMMYYBJGH4XG', 'collaterals': '0x280' }]
    /// contractCreated: BASE32 - address of the contract created. null when it is not a contract deployment transaction.
    /// stateRoot: DATA, 32 Bytes - hash of the state root after the execution of the corresponding block. 0 if the state root is not available.
    /// outcomeStatus: QUANTITY - the outcome status code. 0x0 means success. 0x1 means failed. 0x2 means skipped
    /// logsBloom: DATA, 256 Bytes - bloom filter for light clients to quickly retrieve related logs.
    /// logs: Array - array of log objects that this transaction generated, see cfx_getLogs.
    /// txExecErrorMsg: String, tx exec fail message, if transaction exec success this will be null.
    pub async fn get_transaction_receipt(
        &self,
        tx_id: &str,
    ) -> CfxResult<Option<TransactionReceipt>> {
        let params = CfxParams::new(
            1,
            &format!("{}_getTransactionReceipt", CONFLUX_ENV.as_str()),
            Some(vec![tx_id]),
        );
        self.post(&params).await
    }

    /// cfx_getTransactionByHash
    /// Returns information about a transaction, identified by its hash.
    ///
    /// Parameters:
    /// DATA, 32 Bytes - hash of a transaction
    ///
    /// Returns:
    /// Object - a transaction object, or null when no transaction was found:
    ///
    /// blockHash: DATA, 32 Bytes - hash of the block where this transaction was in and got executed. null when the transaction is pending.
    /// chainId: QUANTITY - the chain ID specified by the sender.
    /// contractCreated: BASE32 - address of the contract created. null when it is not a contract deployment transaction.
    /// data: DATA - the data sent along with the transaction.
    /// epochHeight: QUANTITY - the epoch proposed by the sender. Note that this is NOT the epoch of the block containing this transaction.
    /// from: BASE32 - address of the sender.
    /// gas: QUANTITY - gas provided by the sender.
    /// gasPrice: QUANTITY - gas price provided by the sender in Drip.
    /// hash: DATA, 32 Bytes - hash of the transaction.
    /// nonce: QUANTITY - the number of transactions made by the sender prior to this one.
    /// r: DATA, 32 Bytes - ECDSA signature r.
    /// s: DATA, 32 Bytes - ECDSA signature s.
    /// status: QUANTITY - 0 for success, 1 if an error occurred, 2 for skiped, null when the transaction is skipped or not packed.
    /// storageLimit: QUANTITY - the storage limit specified by the sender.
    /// to: BASE32 - address of the receiver. null when it is a contract deployment transaction.
    /// transactionIndex: QUANTITY - the transaction's position in the block. null when the transaction is pending.
    /// v: QUANTITY - ECDSA recovery id.
    /// value: QUANTITY - value transferred in Drip.
    /// Note that the fields blockHash, contractCreated, status, and transactionIndex are provided by the node as they depend on the transaction's position within the ledger. The rest of the fields are included in or derived from the original transaction.
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

    /// cfx_getBlockByHash
    pub async fn get_block_by_hash(&self, block_hash: &str, tag: bool) -> CfxResult<Option<BlockByHash>> {
        let params = CfxParams::new(
            1,
            &format!("{}_getBlockByHash", CONFLUX_ENV.as_str()),
            Some(vec![Value::from(block_hash), Value::from(tag)]),
        );
        self.post(&params).await
    }
}
