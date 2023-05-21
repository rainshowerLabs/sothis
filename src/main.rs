use clap::{arg, Command};

#[allow(unused_variables)]
fn main() {
    let matches = Command::new("sothis")
        .version("0.1.0")
        .author("makemake <vukasin@gostovic.me>")
        .about("Tool for replaying historical transactions")
        .arg(arg!(--historical_rpc <VALUE>).required(true))
        .arg(arg!(--block <VALUE>).required(true))
        .arg(arg!(--replay_rpc <VALUE>).required(true))
        .get_matches();

    let historical_rpc = matches.get_one::<String>("historical_rpc").expect("required");
    let block = matches.get_one::<String>("block").expect("required");
    let replay_rpc = matches.get_one::<String>("replay_rpc").expect("required");

}