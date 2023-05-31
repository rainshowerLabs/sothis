mod rpc;
mod replay;

use clap::{arg, Command};

use crate::rpc::format::hex_to_decimal;
use crate::rpc::format::format_number_input;
use rpc::rpc::RpcConnection;
use crate::replay::*;

#[allow(unused_variables)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("sothis")
        .version("0.1.2")
        .author("makemake <vukasin@gostovic.me>")
        .about("Tool for replaying historical transactions. Designed to be used with anvil/hh node")
        .arg(arg!(--source_rpc <VALUE>).required(true).help("HTTP JSON-RPC of the nde we're querrying data from"))
        .arg(arg!(--terminal_block <VALUE>).required(true).help("Block we're replaying until"))
        .arg(arg!(--replay_rpc <VALUE>).required(true).help("HTTP JSON-RPC of the node we're replaying data to"))
        .get_matches();

    let source_rpc: String = matches.get_one::<String>("source_rpc").expect("required").to_string();
    let block: String = matches.get_one::<String>("terminal_block").expect("required").to_string();
    let replay_rpc: String = matches.get_one::<String>("replay_rpc").expect("required").to_string();

    let source_rpc = RpcConnection::new(source_rpc);
    let replay_rpc = RpcConnection::new(replay_rpc);
    let block = format_number_input(&block);

    replay_historic_blocks(source_rpc, replay_rpc, hex_to_decimal(&block)?).await?;

    Ok(())
}