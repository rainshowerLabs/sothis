use crate::RpcConnection;

// We use this to prepare the local node for replaying transactions.
// Sets the mining params, and waits for the user to start replaying. 
pub async fn contract_setup(replay_rpc: RpcConnection) -> Result<(), Box<dyn std::error::Error>> {
    // set automine to false
    replay_rpc.evm_set_automine(false).await?;
    // set insanely high interval for the blocks
    replay_rpc.evm_set_interval_mining(std::u32::MAX.into()).await?;

    // Wait for user input from keyboard to proceed
    println!("Please deploy your contracts, and prepare to start replaying.");
    println!("Use the --no_setup flag to skip this step.");
    println!("Press the return(enter) key to start replaying transactions...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
	println!("Starting replay...");

	Ok(())
}