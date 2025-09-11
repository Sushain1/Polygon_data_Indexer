use sqlx::SqlitePool;
use log::info;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    env_logger::init();
    
    // Load the database URL from the environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await?;

    // Fetch the cumulative net flow
    let cumulative_net_flow: f64 = sqlx::query_scalar!(
        "SELECT cumulative_net_flow FROM net_flows LIMIT 1"
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(0.0);

    info!("Current cumulative net flow for Binance hot wallets: {} POL", cumulative_net_flow);

    // Fetch and display the 5 most recent transactions
    let transactions = sqlx::query!(
        "SELECT tx_hash, block_number, value FROM transactions ORDER BY block_number DESC LIMIT 5"
    )
    .fetch_all(&pool)
    .await?;

    if transactions.is_empty() {
        info!("No recent transactions found.");
    } else {
        info!("--- 5 Most Recent Transactions ---");
        for tx in transactions {
            info!(
                "Tx Hash: {:.10}... | Block: {} | Value: {} POL",
                tx.tx_hash, tx.block_number, tx.value
            );
        }
    }

    Ok(())
}