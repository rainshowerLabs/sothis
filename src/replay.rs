use clap::{arg, Command};
use crate::rpc::RpcConnection;


// To replay blocks we:
// 1) Make sure that the replay rpc block is equal to `block`
// 2) Set the `evm_autoMine` mode to create blocks
// 3) Set the `evm_set_interval_mining` to something ridiculously high.
// 4) Get transaction hashes from block
// 5) Get transactions from hashes, `eth_sendTransaction` that to the mempool
// 6) Loop for all transactions in a block
// 7) Set next block timestamp
// 8) `evm_mine` the block
pub fn replay_blocks(historic_rpc: RpcConnection, replay_rpc: RpcConnection, block: &str) -> RetType {
	unimplemented!()
}
