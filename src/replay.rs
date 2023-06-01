use crate::EXIT_ON_TX_FAIL;
use crate::RpcConnection;
use crate::rpc::format::*;
use crate::rpc::types::*;

// To replay blocks we:
// 1) Make sure that the replay rpc block is equal to `block`
// 2) Set the `evm_autoMine` mode to create blocks
// 3) Set the `evm_set_interval_mining` to something ridiculously high.
// 4) Get transaction hashes from block
// 5) Get transactions from hashes, `eth_sendTransaction` that to the mempool
// 6) Loop for all transactions in a block
// 7) Set next block timestamp
// 8) `evm_mine` the block
//EXIT_ON_TX_FAIL
async fn send_transactions(
    replay_rpc: RpcConnection,
    historical_txs: Vec<Transaction>,
    historical_chainid: String,
) -> Result<(), Box<dyn std::error::Error>> {
    for tx in historical_txs {
        let tx = TransactionParams {
            from: tx.from,
            to: tx.to,
            value: tx.value,
            gas: tx.gas,
            gasPrice: tx.gasPrice,
            data: tx.input,
            nonce: tx.nonce,
            chainId: Some(historical_chainid.clone())
        };

        // Gracefully handle errors so execution doesnt halt on error
        match replay_rpc.send_unsigned_transaction(tx).await {
            Ok(_) => (),
            Err(e) => if unsafe { EXIT_ON_TX_FAIL } {
                return Err(e.into());
            } else {
                println!("!!! \x1b[93mError sending transaction:\x1b[0m {} !!!", e)
            }
        }
    }

    Ok(())
}

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
        send_transactions(replay_rpc.clone(), historical_txs, historical_chainid.clone()).await?;

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
