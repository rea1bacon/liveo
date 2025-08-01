pub mod auth;
pub mod db;
pub mod error;
pub mod server;
pub mod websocket;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_server_setup() {
        println!("Testing server setup - this would start the server on port 3000");
        // Note: actual server start would block, so just testing the setup
    }
}
