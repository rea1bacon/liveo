use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
    routing::any,
    Router,
};

pub async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // Client disconnected
            return;
        };

        // Echo the message back
        if socket.send(msg).await.is_err() {
            // Client disconnected
            return;
        }
    }
}

pub fn websocket_routes() -> Router {
    Router::new().route("/ws", any(websocket_handler))
}
