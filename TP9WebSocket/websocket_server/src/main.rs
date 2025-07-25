use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Erreur : impossible de se lier à l'adresse");

    println!("Serveur WebSocket lancé sur ws://{}", addr);

    loop {
        let (stream, addr) = listener.accept().await.expect("Erreur d'acceptation");

        println!("Client connecté depuis : {}", addr);

        tokio::spawn(async move {
            let ws_stream = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    eprintln!("Handshake WebSocket échoué : {}", e);
                    return;
                }
            };

            println!("Handshake réussi avec {}", addr);

            let (mut write, mut read) = ws_stream.split();

            loop {
                tokio::select! {
                    Some(result) = read.next() => {
                        match result {
                            Ok(Message::Text(text)) => {
                                println!("Reçu de {} : {}", addr, text);
                                let response = format!("Serveur : tu as dit '{}'", text);
                                if let Err(e) = write.send(Message::Text(response)).await {
                                    eprintln!("Erreur d'envoi : {}", e);
                                    break;
                                }
                            }
                            Ok(_) => {} 
                            Err(e) => {
                                eprintln!("Erreur de lecture : {}", e);
                                break;
                            }
                        }
                    }

                    _ = sleep(Duration::from_secs(5)) => {
                    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
                    let ping = format!("Message automatique du serveur à {}", timestamp);
                    if let Err(e) = write.send(Message::Text(ping.to_string())).await {
                        eprintln!("Erreur d'envoi du message automatique : {}", e);
                        break;
                    }
                }

                }
            }

            println!("Client déconnecté : {}", addr);
        });
    }
}
