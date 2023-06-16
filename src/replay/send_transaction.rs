use crate::RpcConnection;
use crate::rpc::types::Transaction;
use crate::APP_CONFIG;

// Generic function we use to replay all tx in a block.
pub async fn send_transactions(
    replay_rpc: RpcConnection,
    historical_txs: Vec<Transaction>,
    chain_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let app_config = APP_CONFIG.lock()?;

    let tx_amount = historical_txs.len() as f32;
    let mut fail_tx_amount: f32 = 0.0;

    // TODO: This is really bad, please reimplement this

    if app_config.send_as_raw {
        for tx in historical_txs {
            // Gracefully handle errors so execution doesn't halt on error
            match replay_rpc.send_raw_transaction(tx, chain_id).await {
                Ok(_) => (),
                Err(e) => if app_config.exit_on_tx_fail {
                    return Err(e.into());
                } else {
                    fail_tx_amount += 1.0;
                    println!("!!! \x1b[93mError sending transaction:\x1b[0m {} !!!", e)
                }
            }
        }
    } else {
        for tx in historical_txs {
            // Gracefully handle errors so execution doesn't halt on error
            match replay_rpc.send_unsigned_transaction(tx, chain_id).await {
                Ok(_) => (),
                Err(e) => if app_config.exit_on_tx_fail {
                    return Err(e.into());
                } else {
                    fail_tx_amount += 1.0;
                    println!("!!! \x1b[93mError sending transaction:\x1b[0m {} !!!", e)
                }
            }
        }
    }

    // Calculate the percentage of failed transactions
    let fail_percent = fail_tx_amount / tx_amount;
    if fail_percent > app_config.entropy_threshold {
        println!("!!! \x1b[91mHigh entropy detected!\x1b[0m Fail ratio: {}. Consider restarting the fork\x1b[0m !!!", format!("{:.2}%", fail_percent * 100.0));
    }

    Ok(())
}
