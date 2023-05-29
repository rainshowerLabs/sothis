use crate::RpcConnection;

// To replay blocks we:
// 1) Make sure that the replay rpc block is equal to `block`
// 2) Set the `evm_autoMine` mode to create blocks
// 3) Set the `evm_set_interval_mining` to something ridiculously high.
// 4) Get transaction hashes from block
// 5) Get transactions from hashes, `eth_sendTransaction` that to the mempool
// 6) Loop for all transactions in a block
// 7) Set next block timestamp
// 8) `evm_mine` the block
#[allow(dead_code, unused_variables)]
pub async fn replay_blocks(
    historic_rpc: RpcConnection,
    replay_rpc: RpcConnection,
    block: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // make sure that both rpcs have the same chainid to satisfy the replay thingy
    let historical_chainid = historic_rpc.chain_id().await?;
    let replay_chainid = replay_rpc.chain_id().await?;

    if historical_chainid != replay_chainid {
        return Err("Chain IDs don't match".into());
    }

    // set automine to false
    replay_rpc.evm_set_automine(false).await?;
    // set insanely high interval for the blocks
    replay_rpc.evm_set_interval_mining(std::u32::MAX.into()).await?;

    
    
    Ok(())
}
