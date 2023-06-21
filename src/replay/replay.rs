use crate::APP_CONFIG;
use crate::replay::send_transaction::send_transactions;
use crate::RpcConnection;
use crate::rpc::format::*;
use crate::rpc::types::*;

// To replay historic blocks we:
// 0) Make sure that the chainids match
// 1) Set the `evm_autoMine` mode to create blocks
// 2) Set the `evm_set_interval_mining` to something ridiculously high.
// 3) Get transaction hashes from block
// 4) Get transactions from hashes, `eth_sendTransaction` that to the mempool
// 5) Loop for all transactions in a block
// 6) Set next block timestamp
// 7) `evm_mine` the block
pub async fn replay_historic_blocks(
    source_rpc: RpcConnection,
    replay_rpc: RpcConnection,
    until: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // make sure that both rpcs have the same chainid to satisfy the replay thingy
    let historical_chainid = source_rpc.chain_id().await?;
    let replay_chainid = replay_rpc.chain_id().await?;

    if historical_chainid != replay_chainid {
        return Err("Chain IDs don't match".into());
    }

    // get block mumber of replay node
    let mut replay_block = hex_to_decimal(&replay_rpc.block_number().await?)?;
    if replay_block > until {
        return Err("Replay node block must be less than termination block".into());
    }

    // set automine to false
    replay_rpc.evm_set_automine(false).await?;
    // set insanely high interval for the blocks
    replay_rpc.evm_set_interval_mining(std::u32::MAX.into()).await?;

    while until > replay_block {
        // we write a bit of illegible code
        let hex_block = decimal_to_hex(replay_block + 1);
        // get block from historical node
        let historical_block = source_rpc.get_block_by_number(hex_block.clone()).await?;

        // get transaction hashes from block
        let historical_block: BlockResult = serde_json::from_str(&historical_block)?;
        let historical_txs = historical_block.transactions;

        // send transactions to mempool
        send_transactions(replay_rpc.clone(), historical_txs, hex_to_decimal(&replay_chainid)?).await?;

        // set next block timestamp
        replay_rpc.evm_set_next_block_timestamp(
            hex_to_decimal(&historical_block.timestamp)?
            ).await?;

        // mine the block
        replay_rpc.evm_mine().await?;
        println!("Successfully replayed block {}", hex_to_decimal(&hex_block)?);

        replay_block = hex_to_decimal(&replay_rpc.block_number().await?)?;
    }
    println!("Done replaying blocks");
    Ok(())
}

// To replay live blocks we:
// 0) Assume that we are lagging behind the head.
// 1) Catch up to the head block by using `replay_historic_blocks`. 
// 2) Once we caught up, listen for new blocks.
// 3) Repeat from 2.
#[allow(dead_code, unused_variables)]
pub async fn replay_live(
    replay_rpc: RpcConnection,
    source_rpc: RpcConnection,
    ) -> Result<(), Box<dyn std::error::Error>> {
    let app_config = APP_CONFIG.lock()?;
    loop {
        let latest_block = source_rpc.listen_for_blocks(app_config.block_listen_time).await?;
        if latest_block != replay_rpc.block_number().await? {
            println!("New block detected, replaying...");
            replay_historic_blocks(source_rpc.clone(), replay_rpc.clone(), hex_to_decimal(&latest_block)?).await?;
        }
    }
}
