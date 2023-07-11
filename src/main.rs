mod rpc;
mod replay;
mod tracker;

use clap::{Command, Arg};
use ethers::types::U256;

use crate::replay::replay::replay_historic_blocks;
use crate::replay::replay::replay_live;
use crate::tracker::tracker::track_state;
use crate::tracker::fast_track::fast_track_state;
use crate::rpc::format::hex_to_decimal;
use crate::rpc::format::format_number_input;
use rpc::rpc::RpcConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("sothis")
        .version("0.4.0")
        .author("makemake <vukasin@gostovic.me>")
        .about("Tool for replaying historical transactions. Designed to be used with anvil or hardhat.")
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
            .required_if_eq("mode", "historic")
            .help("Block we're replaying until"))
        .arg(Arg::new("replay_rpc")
            .long("replay_rpc")
            .short('r')
            .num_args(1..)
            .help("HTTP JSON-RPC of the node we're replaying data to"))
        .arg(Arg::new("mode")
            .long("mode")
            .short('m')
            .num_args(1..)
            .default_value("historic")
            .help("Choose between live, historic, track, or fast_track"))
        .arg(Arg::new("exit_on_tx_fail")
            .long("exit_on_tx_fail")
            .num_args(0..)
            .help("Exit the program if a transaction fails"))
        .arg(Arg::new("block_listen_time")
            .long("block_listen_time")
            .short('t')
            .num_args(1..)
            .default_value("500")
            .help("Time in ms to check for new blocks."))
        .arg(Arg::new("entropy_threshold")
            .long("entropy_threshold")
            .num_args(1..)
            .default_value("0.07")
            .help("Set the percentage of failed transactions to trigger a warning"))
        .arg(Arg::new("replay_delay")
            .long("replay_delay")
            .short('d')
            .num_args(1..)
            .default_value("0")
            .help("Default delay for block replay in ms"))
        .arg(Arg::new("send_as_raw")
            .long("send_as_raw")
            .num_args(0..)
            .help("Exit the program if a transaction fails"))
        .arg(Arg::new("contract_address")
            .long("contract_address")
            .short('c')
            .num_args(1..)
            .required_if_eq("mode", "track")
            .help("Address of the contract we're tracking storage."))
        .arg(Arg::new("storage_slot")
            .long("storage_slot")
            .short('l')
            .num_args(1..)
            .required_if_eq("mode", "track")
            .help("Storage slot for the variable we're tracking"))
        .arg(Arg::new("path")
            .long("path")
            .short('p')
            .num_args(1..)
            .default_value(".")
            .help("Path to file we're writing to"))
        .arg(Arg::new("filename")
            .long("filename")
            .short('f')
            .num_args(1..)
            .default_value("")
            .help("Name of the file."))
        .get_matches();

    let source_rpc: String = matches.get_one::<String>("source_rpc").expect("required").to_string();
    let source_rpc = RpcConnection::new(source_rpc);

    let mode: String = matches.get_one::<String>("mode").expect("required").to_string();    
    match mode.as_str() {
        "historic" => {
            println!("Replaying in historic mode...");
        
            let replay_rpc: String = matches.get_one::<String>("replay_rpc").expect("required").to_string();
            let replay_rpc = RpcConnection::new(replay_rpc);

            let terminal_block: String = matches.get_one::<String>("terminal_block").expect("required").to_string();
            let terminal_block = format_number_input(&terminal_block);

            let entropy_threshold = matches.get_one::<String>("entropy_threshold").expect("required").parse::<f32>()?;
            let exit_on_tx_fail = matches.get_occurrences::<String>("exit_on_tx_fail").is_some();
            let send_as_raw = matches.get_occurrences::<String>("send_as_raw").is_some();
            let replay_delay = matches.get_one::<String>("replay_delay").expect("required").parse::<u64>()?;

            replay_historic_blocks(
                source_rpc,
                replay_rpc,
                hex_to_decimal(&terminal_block)?,
                replay_delay,
                entropy_threshold,
                exit_on_tx_fail,
                send_as_raw,
            ).await?;
        },
        "live" => {
            println!("Replaying live blocks...");

            let replay_rpc: String = matches.get_one::<String>("replay_rpc").expect("required").to_string();
            let replay_rpc = RpcConnection::new(replay_rpc);

            let entropy_threshold = matches.get_one::<String>("entropy_threshold").expect("required").parse::<f32>()?;
            let exit_on_tx_fail = matches.get_occurrences::<String>("exit_on_tx_fail").is_some();
            let send_as_raw = matches.get_occurrences::<String>("send_as_raw").is_some();
            let replay_delay = matches.get_one::<String>("replay_delay").expect("required").parse::<u64>()?;
            let block_listen_time = matches.get_one::<String>("block_listen_time").expect("required").parse::<u64>()?;

            replay_live(
                source_rpc,
                replay_rpc,
                replay_delay,
                block_listen_time,
                entropy_threshold,
                exit_on_tx_fail,
                send_as_raw,
            ).await?;
        },
        "track" => {
            println!("Tracking state variable...");
            println!("Send SIGTERM or SIGINT (ctrl-c) to serialize to JSON, write and stop.");
            
            let contract_address: String = matches.get_one::<String>("contract_address").expect("required").to_string();
            let storage_slot: String = matches.get_one::<String>("storage_slot").expect("required").to_string();
            let storage_slot = U256::from_dec_str(&storage_slot)?;
            
            // If terminal_block is set by the user use that, otherwise have it be none
            let terminal_block: Option<u64> = matches.get_one::<String>("terminal_block").map(|x| x.parse().expect("Invalid terminal block"));
            
            if terminal_block == None {
                println!("No terminal block set, tracking indefinitely.");
            }

            let block_listen_time = matches.get_one::<String>("block_listen_time").expect("required").parse::<u64>()?;
            let path = matches.get_one::<String>("path").expect("required").to_string();
            let filename = matches.get_one::<String>("filename").expect("required").to_string();

            track_state(
                source_rpc,
                storage_slot,
                contract_address,
                terminal_block,
                block_listen_time,
                path,
                filename,
            ).await?;
        },
        "fast_track" => {
            println!("Fast tracking state variable...");
            println!("Send SIGTERM or SIGINT (ctrl-c) to serialize to JSON, write and stop.");
            
            let contract_address: String = matches.get_one::<String>("contract_address").expect("required").to_string();
            let storage_slot: String = matches.get_one::<String>("storage_slot").expect("required").to_string();
            let storage_slot = U256::from_dec_str(&storage_slot)?;
            
            // If terminal_block is set by the user use that, otherwise have it be none
            let terminal_block: Option<u64> = matches.get_one::<String>("terminal_block").map(|x| x.parse().expect("Invalid terminal block"));
            
            if terminal_block == None {
                // print block and newline later when we get the block number so it looks nice
                print!("No terminal block set, tracking until the current head...");
            }

            let block_listen_time = matches.get_one::<String>("block_listen_time").expect("required").parse::<u64>()?;
            let path = matches.get_one::<String>("path").expect("required").to_string();
            let filename = matches.get_one::<String>("filename").expect("required").to_string();

            fast_track_state(
                source_rpc,
                storage_slot,
                contract_address,
                terminal_block,
                block_listen_time,
                path,
                filename,
            ).await?;
        }
        &_ => {
            panic!("Mode does not exist!");
        },
    }
    
    Ok(())
}
