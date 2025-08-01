use crate::websocket;
use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Liveo is running</h1>")
}

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(hello_world))
        .merge(websocket::websocket_routes());

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("Server running on http://127.0.0.1:{}", port);
    println!("WebSocket available at ws://127.0.0.1:{}/ws", port);

    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server(3000).await
}
