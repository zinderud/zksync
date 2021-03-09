use web3::{Transport, Web3};

use zksync_types::operations::ZkSyncOp;

use crate::contract;
use crate::eth_tx_helpers::{get_ethereum_transaction, get_input_data_from_ethereum_transaction};
use crate::events::BlockEvent;
use zksync_types::{AccountId, BlockNumber};

/// Description of a Rollup operations block
#[derive(Debug, Clone)]
pub struct RollupOpsBlock {
    /// Rollup block number
    pub block_num: BlockNumber,
    /// Rollup operations in block
    pub ops: Vec<ZkSyncOp>,
    /// Fee account
    pub fee_account: AccountId,
}

impl RollupOpsBlock {
    /// Returns a Rollup operations block description
    ///
    /// # Arguments
    ///
    /// * `web3` - Web3 provider url
    /// * `event_data` - Rollup contract event description
    ///
    ///
    pub async fn get_rollup_ops_blocks<T: Transport>(
        web3: &Web3<T>,
        event_data: &BlockEvent,
    ) -> anyhow::Result<Vec<Self>> {
        let transaction = get_ethereum_transaction(web3, &event_data.transaction_hash).await?;
        let input_data = get_input_data_from_ethereum_transaction(&transaction)?;
        let blocks = if let Ok(block) =
            contract::default::rollup_ops_blocks_from_bytes(input_data.clone())
        {
            vec![block]
        } else {
            contract::v4::rollup_ops_blocks_from_bytes(input_data)?
        };
        Ok(blocks)
    }
}
