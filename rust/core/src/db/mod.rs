/**
 * This module is used for database operations.
 * - Connect to the database
 * - Manage the track tables
 * - Perform read operations
 */
use tokio_postgres::{Client, NoTls};

mod queries;
mod tracktable;
mod utils;
mod workspace;

#[allow(dead_code)]
async fn connect() -> Result<Client, String> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres", NoTls)
        .await
        .map_err(|e| e.to_string())?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}
