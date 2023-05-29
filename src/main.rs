mod rpc;
mod replay;

use clap::{arg, Command};
use rpc::rpc::RpcConnection;
use crate::replay::*;

#[allow(unused_variables)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("sothis")
        .version("0.1.0")
        .author("makemake <vukasin@gostovic.me>")
        .about("Tool for replaying historical transactions. Designed to be used with anvil/hh node")
        .arg(arg!(--historical_rpc <VALUE>).required(true).help("HTTP JSON-RPC of the nde we're querrying data from"))
        .arg(arg!(--block <VALUE>).required(true).help("Block from which we're replaying"))
        .arg(arg!(--replay_rpc <VALUE>).required(true).help("HTTP JSON-RPC of the node we're replaying data to"))
        .get_matches();

    let historical_rpc: String = matches.get_one::<String>("historical_rpc").expect("required").to_string();
    let block: String = matches.get_one::<String>("block").expect("required").to_string();
    let replay_rpc: String = matches.get_one::<String>("replay_rpc").expect("required").to_string();

    let historical_rpc = RpcConnection::new(historical_rpc);
    let replay_rpc = RpcConnection::new(replay_rpc);

    replay_blocks(historical_rpc, replay_rpc, &block).await?;

    Ok(())
}