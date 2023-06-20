mod rpc;
mod replay;

use crate::replay::replay::replay_historic_blocks;
use crate::replay::replay::replay_live;
use clap::{Command, Arg};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::rpc::format::hex_to_decimal;
use crate::rpc::format::format_number_input;
use rpc::rpc::RpcConnection;

// Settings flags
#[derive(Default)]
pub struct AppConfig {
    exit_on_tx_fail: bool,
    send_as_raw: bool,
    entropy_threshold: f32,
}

lazy_static! {
    static ref APP_CONFIG: Mutex<AppConfig> = Mutex::new(AppConfig::default());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("sothis")
        .version("0.2.0")
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
            .num_args(0..)
            .help("Exit the program if a transaction fails"))
        .arg(Arg::new("entropy_threshold")
            .long("entropy_threshold")
            .num_args(1..)
            .default_value("0.07")
            .help("Set the percentage of failed transactions to trigger a warning"))
        .arg(Arg::new("send_as_raw")
            .long("send_as_raw")
            .num_args(0..)
            .help("Exit the program if a transaction fails"))
        .arg(Arg::new("track_state")
            .long("track_state")
            .short('t')
            .num_args(0..)
            .help("Exit the program if a transaction fails"))

        .get_matches();

    let source_rpc: String = matches.get_one::<String>("source_rpc").expect("required").to_string();
    let replay_rpc: String = matches.get_one::<String>("replay_rpc").expect("required").to_string();
    let mode: String = matches.get_one::<String>("mode").expect("required").to_string();

    // Set settings
    {
        let mut app_config = APP_CONFIG.lock()?;
        app_config.exit_on_tx_fail = matches.get_occurrences::<String>("exit_on_tx_fail").is_some();
        app_config.send_as_raw = matches.get_occurrences::<String>("send_as_raw").is_some();
        app_config.entropy_threshold = matches.get_one::<String>("entropy_threshold").expect("required").parse::<f32>()?;
    }

    let source_rpc = RpcConnection::new(source_rpc);
    let replay_rpc = RpcConnection::new(replay_rpc);
    
    match mode.as_str() {
        "historic" => {
            println!("Replaying in historic mode...");
            
            let block: String = matches.get_one::<String>("terminal_block").expect("required").to_string();
            let block = format_number_input(&block);

            replay_historic_blocks(source_rpc, replay_rpc, hex_to_decimal(&block)?).await?;
        },
        "live" => {
            println!("Replaying live blocks...");
            replay_live(replay_rpc, source_rpc).await?;
        }
        &_ => {
            // handle this properly later
            panic!("Mode does not exist!");
        },
    }
    
    Ok(())
}
