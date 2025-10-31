use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

pub async fn run_listener<F>(url: Url, mut handler: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(String),
{
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket connected");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = msg?;
        if let Message::Text(text) = msg {
            handler(text);
        }
    }

    Ok(())
}

pub fn handle_mame_message(message: String) {
    println!("Received MAME message: {}", message);
    // Placeholder for MAME message parsing and IPC
}
