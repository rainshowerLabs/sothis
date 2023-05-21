mod rpc;

use clap::{arg, Command};

// TODO:
// add logic to send transactions from blocks
// add logic to set local evm to mine blocks, and force mine when done with a block

#[allow(unused_variables)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("sothis")
        .version("0.1.0")
        .author("makemake <vukasin@gostovic.me>")
        .about("Tool for replaying historical transactions")
        .arg(arg!(--historical_rpc <VALUE>).required(true))
        .arg(arg!(--block <VALUE>).required(true))
        .arg(arg!(--replay_rpc <VALUE>).required(true))
        .get_matches();

    let historical_rpc: String = matches.get_one::<String>("historical_rpc").expect("required").to_string();
    let block: String = matches.get_one::<String>("block").expect("required").to_string();
    let replay_rpc: String = matches.get_one::<String>("replay_rpc").expect("required").to_string();

    //post(historical_rpc, "eth_blockNumber".to_string(), "".to_string()).await?;

    Ok(())
}