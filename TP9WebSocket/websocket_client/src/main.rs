use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() {
    let url = "ws://127.0.0.1:9001";
    let (mut ws_stream, _) = connect_async(url).await.expect("Connexion échouée");

    println!("Connecté au serveur WebSocket : {}", url);

    loop {
        print!("Entrez un message : ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            break;
        }

        ws_stream.send(Message::Text(input.trim().to_string())).await.unwrap();

        if let Some(msg) = ws_stream.next().await {
            match msg {
                Ok(Message::Text(text)) => println!("Réponse : {}", text),
                Ok(_) => {},
                Err(e) => {
                    println!("Erreur : {}", e);
                    break;
                }
            }
        }
    }

    println!("Fin du client");
}
