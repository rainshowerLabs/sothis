mod cli_arg;
mod replay;
mod rpc;
mod tracker;

use ethers::types::U256;

use crate::replay::replay::replay_historic_blocks;
use crate::replay::replay::replay_live;
use crate::replay::setup::contract_setup;

use crate::tracker::call_track::call_track;
use crate::tracker::fast_track::fast_track_state;
use crate::tracker::tracker::track_state;

use crate::rpc::format::format_number_input;
use crate::rpc::format::hex_to_decimal;
use rpc::rpc::RpcConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli_arg::create_match().get_matches();

    let source_rpc = matches
        .get_one::<String>("source_rpc")
        .expect("Invalid source_rpc")
        .to_string();
    let source_rpc = RpcConnection::new(source_rpc);

    let mode: String = matches
        .get_one::<String>("mode")
        .expect("Invalid mode")
        .to_string();
    match mode.as_str() {
        "historic" => {
            println!("Replaying in historic mode...");

            let replay_rpc: String = matches
                .get_one::<String>("replay_rpc")
                .expect("Invalid replay_rpc")
                .to_string();
            let replay_rpc = RpcConnection::new(replay_rpc);

            let terminal_block: String = matches
                .get_one::<String>("terminal_block")
                .expect("No valid terminal_block set!")
                .to_string();
            let terminal_block = format_number_input(&terminal_block);

            let entropy_threshold = matches
                .get_one::<String>("entropy_threshold")
                .expect("required")
                .parse::<f32>()?;
            let exit_on_tx_fail = matches
                .get_occurrences::<String>("exit_on_tx_fail")
                .is_some();
            let send_as_unsigned = matches
                .get_occurrences::<String>("send_as_unsigned")
                .is_some();
            let replay_delay = matches
                .get_one::<String>("replay_delay")
                .expect("required")
                .parse::<u64>()?;

            let no_setup = matches.get_occurrences::<String>("no_setup").is_some();
            if !no_setup {
                contract_setup(replay_rpc.clone()).await?;
            }

            replay_historic_blocks(
                source_rpc,
                replay_rpc,
                hex_to_decimal(&terminal_block)?,
                replay_delay,
                entropy_threshold,
                exit_on_tx_fail,
                send_as_unsigned,
            )
            .await?;
        }
        "live" => {
            println!("Replaying live blocks...");

            let replay_rpc: String = matches
                .get_one::<String>("replay_rpc")
                .expect("Invalid replay_rpc supplied!")
                .to_string();
            let replay_rpc = RpcConnection::new(replay_rpc);

            let entropy_threshold = matches
                .get_one::<String>("entropy_threshold")
                .expect("Invalid entropy_threshold")
                .parse::<f32>()?;
            let exit_on_tx_fail = matches
                .get_occurrences::<String>("exit_on_tx_fail")
                .is_some();
            let send_as_unsigned = matches
                .get_occurrences::<String>("send_as_unsigned")
                .is_some();
            let replay_delay = matches
                .get_one::<String>("replay_delay")
                .expect("Invalid replay_delay")
                .parse::<u64>()?;
            let block_listen_time = matches
                .get_one::<String>("block_listen_time")
                .expect("Invalid block_listen_time")
                .parse::<u64>()?;

            let no_setup = matches.get_occurrences::<String>("no_setup").is_some();
            if !no_setup {
                contract_setup(replay_rpc.clone()).await?;
            }

            replay_live(
                source_rpc,
                replay_rpc,
                replay_delay,
                block_listen_time,
                entropy_threshold,
                exit_on_tx_fail,
                send_as_unsigned,
            )
            .await?;
        }
        "track" => {
            println!("Tracking state variable...");
            println!("Send SIGTERM or SIGINT (ctrl-c) to serialize to JSON, write and stop.");

            let contract_address: String = matches
                .get_one::<String>("contract_address")
                .expect("Invalid contract_address")
                .to_string();
            let storage_slot: String = matches
                .get_one::<String>("storage_slot")
                .expect("Invalid storage_slot")
                .to_string();
            let storage_slot = U256::from_dec_str(&storage_slot)?;

            // If terminal_block is set by the user use that, otherwise have it be none
            let terminal_block: Option<u64> = matches
                .get_one::<String>("terminal_block")
                .map(|x| x.parse().expect("Invalid terminal block"));

            if terminal_block.is_none() {
                println!("No terminal block set, tracking indefinitely.");
            }

            let block_listen_time = matches
                .get_one::<String>("block_listen_time")
                .expect("required")
                .parse::<u64>()?;
            let decimal = matches.get_occurrences::<String>("decimal").is_some();
            let path = matches
                .get_one::<String>("path")
                .expect("required")
                .to_string();
            let filename = matches
                .get_one::<String>("filename")
                .expect("required")
                .to_string();

            track_state(
                source_rpc,
                storage_slot,
                contract_address,
                terminal_block,
                block_listen_time,
                decimal,
                path,
                filename,
            )
            .await?;
        }
        "fast_track" => {
            println!("Fast tracking state variable...");
            println!("Send SIGTERM or SIGINT (ctrl-c) to serialize to JSON, write and stop.");

            let contract_address: String = matches
                .get_one::<String>("contract_address")
                .expect("Invalid contract_address")
                .to_string();
            let storage_slot: String = matches
                .get_one::<String>("storage_slot")
                .expect("Invalid storage_slot")
                .to_string();
            let storage_slot = U256::from_dec_str(&storage_slot)?;

            // If terminal_block is set by the user use that, otherwise have it be none
            let terminal_block = matches
                .get_one::<String>("terminal_block")
                .map(|x| x.parse().expect("Invalid terminal block"));

            let origin_block = matches
                .get_one::<String>("origin_block")
                .expect("Invalid origin_block")
                .parse::<u64>()?;
            let query_interval = matches
                .get_one::<String>("query_interval")
                .map(|x| x.parse().expect("Invalid query interval"));
            let decimal = matches.get_occurrences::<String>("decimal").is_some();
            let path = matches
                .get_one::<String>("path")
                .expect("Invalid path")
                .to_string();
            let filename = matches
                .get_one::<String>("filename")
                .expect("Invalid filename")
                .to_string();

            fast_track_state(
                source_rpc,
                storage_slot,
                contract_address,
                terminal_block,
                origin_block,
                query_interval,
                decimal,
                path,
                filename,
            )
            .await?;
        }
        "call_track" => {
            println!("Call tracking...");
            println!("Send SIGTERM or SIGINT (ctrl-c) to serialize to JSON, write and stop.");

            let contract_address: String = matches
                .get_one::<String>("contract_address")
                .expect("Invalid contract_address")
                .to_string();
            let calldata: String = matches
                .get_one::<String>("calldata")
                .expect("Invalid calldata")
                .to_string();

            // If terminal_block is set by the user use that, otherwise have it be none
            let terminal_block = matches
                .get_one::<String>("terminal_block")
                .map(|x| x.parse().expect("Invalid terminal block"));

            let origin_block = matches
                .get_one::<String>("origin_block")
                .expect("Invalid origin_block")
                .parse::<u64>()?;
            let query_interval = matches
                .get_one::<String>("query_interval")
                .map(|x| x.parse().expect("Invalid query interval"));
            let decimal = matches.get_occurrences::<String>("decimal").is_some();
            let path = matches
                .get_one::<String>("path")
                .expect("Invalid path")
                .to_string();
            let filename = matches
                .get_one::<String>("filename")
                .expect("Invalid filename")
                .to_string();

            call_track(
                source_rpc,
                calldata,
                contract_address,
                terminal_block,
                origin_block,
                query_interval,
                decimal,
                path,
                filename,
            )
            .await?;
        }
        &_ => {
            panic!("Mode does not exist!");
        }
    }

    Ok(())
}
