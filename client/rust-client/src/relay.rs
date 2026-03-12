use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio::sync::broadcast;

pub async fn run_relay(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("Relay listening on: {}", addr);

    let (tx, _rx) = broadcast::channel::<String>(100);

    while let Ok((stream, addr)) = listener.accept().await {
        let tx = tx.clone();
        tokio::spawn(handle_connection(stream, addr, tx));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr, tx: broadcast::Sender<String>) {
    println!("New connection: {}", addr);

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Error during WebSocket handshake: {}", e);
            return;
        }
    };

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut rx = tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Err(e) = ws_sender.send(tokio_tungstenite::tungstenite::Message::Text(msg.into())).await {
                eprintln!("Error sending message: {}", e);
                break;
            }
        }
    });

    let tx_clone = tx.clone();
    let mut receive_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
                // For now, we just broadcast every text message we receive.
                // In the future, we will verify signatures here.
                if let Err(e) = tx_clone.send(text.to_string()) {
                    eprintln!("Error broadcasting message: {}", e);
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => receive_task.abort(),
        _ = (&mut receive_task) => send_task.abort(),
    };

    println!("Connection closed: {}", addr);
}
