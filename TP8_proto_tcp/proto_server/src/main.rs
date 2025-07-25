use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Échec du bind");

    println!("Serveur TCP démarré sur le port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Erreur connexion : {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client déconnecté");
                return;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Erreur de lecture : {}", e);
                return;
            }
        };

        // Lecture protocole
        let code = buffer[0];
        let username_len = buffer[1] as usize;
        let username = String::from_utf8_lossy(&buffer[2..2 + username_len]);

        let msg_len = u16::from_be_bytes([buffer[2 + username_len], buffer[3 + username_len]]) as usize;
        let msg_start = 4 + username_len;
        let message = String::from_utf8_lossy(&buffer[msg_start..msg_start + msg_len]);

        println!("[{}] {}: {}", code, username, message);

        if code == 3 {
            println!("{} s'est déconnecté", username);
            return;
        }

        // Réponse du serveur
        let response = format!("Serveur a reçu ton message, {}", username);
        let _ = stream.write(response.as_bytes());
    }
}
