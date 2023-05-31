mod rpc;
mod replay;

use clap::{Command, Arg};

use crate::rpc::format::hex_to_decimal;
use crate::rpc::format::format_number_input;
use rpc::rpc::RpcConnection;
use crate::replay::*;

static mut EXIT_ON_TX_FAIL: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("sothis")
        .version("0.1.2")
        .author("makemake <vukasin@gostovic.me>")
        .about("Tool for replaying historical transactions. Designed to be used with anvil")
        .arg(Arg::new("source_rpc")
            .long("source_rpc")
            .short('s')
            .num_args(1..)
            .required(true)
            .help("HTTP JSON-RPC of the node we're querying data from"))
        .arg(Arg::new("terminal_block")
            .long("terminal_block")
            .short('b')
            .num_args(1..)
            .required(true)
            .help("Block we're replaying until"))
        .arg(Arg::new("replay_rpc")
            .long("replay_rpc")
            .short('r')
            .num_args(1..)
            .required(true)
            .help("HTTP JSON-RPC of the node we're replaying data to"))
        .arg(Arg::new("mode")
            .long("mode")
            .short('m')
            .num_args(1..)
            .default_value("historic")
            .help("Choose between live replay or historic"))
        .arg(Arg::new("exit_on_tx_fail")
            .long("exit_on_tx_fail")
            .num_args(1..)
            .help("Exit the program if a transaction fails"))
        .get_matches();

    let source_rpc: String = matches.get_one::<String>("source_rpc").expect("required").to_string();
    let block: String = matches.get_one::<String>("terminal_block").expect("required").to_string();
    let replay_rpc: String = matches.get_one::<String>("replay_rpc").expect("required").to_string();

    let mode: String = matches.get_one::<String>("mode").expect("required").to_string();

    // this is shit but so is the clap crate
    let vals: bool = matches.get_one::<String>("exit_on_tx_fail").is_some();
    if vals == true {
        unsafe {
            EXIT_ON_TX_FAIL = true;
        }
    }

    let source_rpc = RpcConnection::new(source_rpc);
    let replay_rpc = RpcConnection::new(replay_rpc);
    let block = format_number_input(&block);

    if mode == "historic" {
        replay_historic_blocks(source_rpc, replay_rpc, hex_to_decimal(&block)?).await?;
    } else {
        unimplemented!();
    }

    Ok(())
}