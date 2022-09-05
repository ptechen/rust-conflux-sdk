use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfxStatus {
    #[serde(rename = "bestHash")]
    pub best_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "epochNumber")]
    pub epoch_number: String,
    #[serde(rename = "latestCheckpoint")]
    pub latest_checkpoint: String,
    #[serde(rename = "latestConfirmed")]
    pub latest_confirmed: String,
    #[serde(rename = "latestState")]
    pub latest_state: String,
    #[serde(rename = "networkId")]
    pub network_id: String,
    #[serde(rename = "pendingTxNumber")]
    pub pending_tx_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionReceipt {
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "contractCreated")]
    pub contract_created: Value,
    #[serde(rename = "epochNumber")]
    pub epoch_number: String,
    pub from: String,
    #[serde(rename = "gasCoveredBySponsor")]
    pub gas_covered_by_sponsor: bool,
    #[serde(rename = "gasFee")]
    pub gas_fee: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    pub index: String,
    pub logs: Vec<Value>,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: String,
    #[serde(rename = "outcomeStatus")]
    pub outcome_status: String,
    #[serde(rename = "stateRoot")]
    pub state_root: String,
    #[serde(rename = "storageCollateralized")]
    pub storage_collateralized: String,
    #[serde(rename = "storageCoveredBySponsor")]
    pub storage_covered_by_sponsor: bool,
    #[serde(rename = "storageReleased")]
    pub storage_released: Vec<Value>,
    pub to: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "txExecErrorMsg")]
    pub tx_exec_error_msg: Value,
}

