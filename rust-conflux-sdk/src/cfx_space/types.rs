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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionByHash {
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "contractCreated")]
    pub contract_created: Value,
    pub data: String,
    #[serde(rename = "epochHeight")]
    pub epoch_height: String,
    pub from: String,
    pub gas: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    pub hash: String,
    pub nonce: String,
    pub r: String,
    pub s: String,
    pub status: String,
    #[serde(rename = "storageLimit")]
    pub storage_limit: String,
    pub to: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String,
    pub v: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockByHash {
    pub adaptive: bool,
    pub blame: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    pub custom: Vec<String>,
    #[serde(rename = "deferredLogsBloomHash")]
    pub deferred_logs_bloom_hash: String,
    #[serde(rename = "deferredReceiptsRoot")]
    pub deferred_receipts_root: String,
    #[serde(rename = "deferredStateRoot")]
    pub deferred_state_root: String,
    pub difficulty: String,
    #[serde(rename = "epochNumber")]
    pub epoch_number: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    pub hash: String,
    pub height: String,
    pub miner: String,
    pub nonce: String,
    #[serde(rename = "parentHash")]
    pub parent_hash: String,
    #[serde(rename = "posReference")]
    pub pos_reference: String,
    #[serde(rename = "powQuality")]
    pub pow_quality: String,
    #[serde(rename = "refereeHashes")]
    pub referee_hashes: Vec<String>,
    pub size: String,
    pub timestamp: String,
    pub transactions: Vec<Value>,
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "contractCreated")]
    pub contract_created: Value,
    pub data: String,
    #[serde(rename = "epochHeight")]
    pub epoch_height: String,
    pub from: String,
    pub gas: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    pub hash: String,
    pub nonce: String,
    pub r: String,
    pub s: String,
    pub status: String,
    #[serde(rename = "storageLimit")]
    pub storage_limit: String,
    pub to: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String,
    pub v: String,
    pub value: String,
}
