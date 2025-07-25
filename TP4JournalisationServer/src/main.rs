use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, BufReader};
use std::sync::{Arc, Mutex};
use std::fs::{OpenOptions, create_dir_all};
use chrono::Utc;
use std::io::Write;

#[tokio::main]
async fn main() {
    
    create_dir_all("logs").expect("Erreur création dossier logs");

   
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/server.log")
        .expect("Impossible d'ouvrir le fichier log");

    let log_file = Arc::new(Mutex::new(file)); // Partagé entre les tâches

    // Démarrer le serveur TCP
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Impossible de lier le port");

    println!("Serveur en écoute sur 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await.expect("Échec de connexion");
        println!("Client connecté : {}", addr);

        let log_file_clone = Arc::clone(&log_file);
        tokio::spawn(async move {
            handle_client(socket, log_file_clone).await;
        });
    }
}

async fn handle_client(stream: TcpStream, log_file: Arc<Mutex<std::fs::File>>) {
    let peer = stream.peer_addr().unwrap();
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let timestamp = Utc::now().to_rfc3339();
        let log = format!("[{}] {}\n", timestamp, line);

        println!("Reçu de {} : {}", peer, line);

        let mut file = log_file.lock().unwrap();
        file.write_all(log.as_bytes()).expect("Écriture impossible");
    }

    println!("Client déconnecté : {}", peer);
}
