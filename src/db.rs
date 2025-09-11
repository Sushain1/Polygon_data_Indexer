use sqlx::{SqlitePool, FromRow};
use log::info;
use chrono::Utc;

// A struct to represent a row in our 'transactions' table
#[derive(Debug, FromRow)]
pub struct Transaction {
    pub tx_hash: String,
    pub block_number: i64,
    pub from_address: String,
    pub to_address: String,
    pub value: f64,
    pub timestamp: String,
}

// Function to insert a new transaction record into the database
pub async fn insert_transaction(
    pool: &SqlitePool,
    tx: &Transaction,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO transactions (tx_hash, block_number, from_address, to_address, value, timestamp) VALUES (?, ?, ?, ?, ?, ?)",
        tx.tx_hash,
        tx.block_number,
        tx.from_address,
        tx.to_address,
        tx.value,
        tx.timestamp
    )
    .execute(pool)
    .await?;

    info!("Inserted transaction {}", tx.tx_hash);
    Ok(())
}

// Function to update the cumulative net-flow value in the database
// In src/db.rs
// ...

// Function to update the cumulative net-flow value in the database
pub async fn update_net_flow(pool: &SqlitePool, value: f64) -> Result<(), sqlx::Error> {
    // First, get the current cumulative net-flow
    let current_flow: f64 = sqlx::query_scalar!(
        "SELECT cumulative_net_flow FROM net_flows LIMIT 1"
    )
    .fetch_one(pool)
    .await
    .unwrap_or(0.0);

    let new_flow = current_flow + value;
    
    // Fix for the E0716 error: Create a variable to hold the timestamp string
    let timestamp = Utc::now().to_rfc3339();

    // Use an UPSERT statement to either insert the first row or update the existing one
    sqlx::query!(
        "INSERT INTO net_flows (id, timestamp, cumulative_net_flow) VALUES (1, ?, ?)
         ON CONFLICT(id) DO UPDATE SET cumulative_net_flow = ?, timestamp = ?",
        timestamp,
        new_flow,
        new_flow,
        timestamp
    )
    .execute(pool)
    .await?;

    info!("Updated cumulative net flow to: {}", new_flow);
    Ok(())
}


// You can add more database-related functions here if needed.