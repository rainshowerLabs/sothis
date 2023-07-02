use crate::rpc::error::RequestError;
use crate::RpcConnection;
use crate::rpc::types::Transaction;
use crate::APP_CONFIG;

// Abstract over the return types of send functions
impl RpcConnection {
    async fn send(&self, tx: Transaction, chain_id: u64) -> Result<String, RequestError> {
        if APP_CONFIG.lock().unwrap().send_as_raw {
            self.send_raw_transaction(tx, chain_id).await
        } else {
            self.send_unsigned_transaction(tx, chain_id).await
        }
    }
}

// Generic function we use to replay all tx in a block.
pub async fn send_transactions(
    replay_rpc: RpcConnection,
    historical_txs: Vec<Transaction>,
    chain_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let exit_on_tx_fail;
    let entropy_threshold;
    {
        let app_config = APP_CONFIG.lock()?;
        exit_on_tx_fail = app_config.exit_on_tx_fail;
        entropy_threshold = app_config.entropy_threshold;
    }

    let tx_amount = historical_txs.len() as f32;
    let mut fail_tx_amount: f32 = 0.0;

    for tx in historical_txs {
        // Gracefully handle errors so execution doesn't halt on error
        match replay_rpc.send(tx, chain_id).await {
            Ok(_) => (),
            Err(e) => if exit_on_tx_fail {
                return Err(e.into());
            } else {
                fail_tx_amount += 1.0;
                println!("!!! \x1b[93mError sending transaction:\x1b[0m {} !!!", e)
            }
        }
    }

    // Calculate the percentage of failed transactions
    let fail_percent = fail_tx_amount / tx_amount;
    if fail_percent > entropy_threshold {
        println!("!!! \x1b[91mHigh entropy detected!\x1b[0m Fail ratio: {}. Consider restarting the fork\x1b[0m !!!", format!("{:.2}%", fail_percent * 100.0));
    }

    Ok(())
}
