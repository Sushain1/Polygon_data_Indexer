use std::sync::Arc;
use ethers_core::types::Address;
use ethers_providers::{Provider, Ws, Middleware};
use futures_util::stream::StreamExt;
use sqlx::sqlite::SqlitePool;
use log::{info, error};
use std::collections::HashSet;
use chrono::Utc;

// We'll use a `db` module to handle all database operations
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Load environment variables from the .env file
    dotenvy::dotenv().ok();

    // Database connection pool
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await?;
    info!("Database connection pool created.");

    // Binance addresses for quick lookups
    let binance_addresses: HashSet<Address> = [
        "0xF977814e90dA44bFA03b6295A0616a897441aceC",
        "0xe7804c37c13166fF0b37F5aE0BB07A3aEbb6e245",
        "0x505e71695E9bc45943c58adEC1650577BcA68fD9",
        "0x290275e3db66394C52272398959845170E4DCb88",
        "0xD5C08681719445A5Fdce2Bda98b341A49050d821",
        "0x082489A616aB4D46d1947eE3F912e080815b08DA"
    ].iter().map(|s| s.parse().expect("Invalid address")).collect();

    // Polygon RPC provider using WebSocket for real-time updates
    let rpc_url = std::env::var("POLYGON_RPC_URL")
        .expect("POLYGON_RPC_URL must be set");
    let provider = Arc::new(Provider::<Ws>::connect(&rpc_url).await?);
    info!("Connected to Polygon RPC at {}", rpc_url);

    // Subscribe to new block headers
    let mut stream = provider.subscribe_blocks().await?;
    info!("Subscribed to new block headers.");

    // Main indexing loop
    while let Some(block) = stream.next().await {
        if let Some(block_number) = block.number {
            info!("New block header received: {}", block_number);

            // Fetch the full block with all transactions
            if let Some(full_block) = provider.get_block_with_txs(block_number).await? {
                // Iterate through all transactions in the block
                for tx in full_block.transactions {
                    // Check if the transaction involves a Binance address
                    if let Some(to_addr) = tx.to {
                        let mut net_flow_change: f64 = 0.0;

                        // Handle outflows from Binance
                        if binance_addresses.contains(&tx.from) {
                            // Convert value from Wei (U256) to POL (f64) for simplicity
                            let value_pol = tx.value.as_u128() as f64 / 1_000_000_000_000_000_000.0;
                            info!("Detected Outflow from Binance: {} POL", value_pol);
                            net_flow_change -= value_pol;
                        }

                        // Handle inflows to Binance
                        if binance_addresses.contains(&to_addr) {
                            let value_pol = tx.value.as_u128() as f64 / 1_000_000_000_000_000_000.0;
                            info!("Detected Inflow to Binance: {} POL", value_pol);
                            net_flow_change += value_pol;
                        }
                        
                        // If the transaction involved a Binance address, process it
                        if net_flow_change != 0.0 {
                            // Create a new transaction struct to insert into the database
                            let new_tx = db::Transaction {
                                tx_hash: format!("{:?}", tx.hash),
                                block_number: block_number.as_u64() as i64,
                                from_address: format!("{:?}", tx.from),
                                to_address: format!("{:?}", to_addr),
                                value: net_flow_change.abs(), // Store positive value
                                timestamp: Utc::now().to_rfc3339(),
                            };
                            
                            // Insert the transaction into the 'transactions' table
                            if let Err(e) = db::insert_transaction(&pool, &new_tx).await {
                                error!("Failed to insert transaction: {}", e);
                            }

                            // Update the cumulative net-flow in the 'net_flows' table
                            if let Err(e) = db::update_net_flow(&pool, net_flow_change).await {
                                error!("Failed to update net flow: {}", e);
                            }
                        }
                    }
                }
            } else {
                error!("Failed to fetch full block {}", block_number);
            }
        }
    }
    
    Ok(())
}