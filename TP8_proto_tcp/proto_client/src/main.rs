use std::net::TcpStream;
use std::io::{self, Write, Read};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Connexion impossible");

    print!("Entrez votre nom : ");
    io::stdout().flush().unwrap();

    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    loop {
        print!("Message : ");
        io::stdout().flush().unwrap();

        let mut message = String::new();
        io::stdin().read_line(&mut message).unwrap();
        let message = message.trim();

        let code = if message == "/quit" { 3 } else { 2 };
        let username_bytes = username.as_bytes();
        let message_bytes = message.as_bytes();

        let username_len = username_bytes.len() as u8;
        let message_len = (message_bytes.len() as u16).to_be_bytes();

        let mut packet = vec![code, username_len];
        packet.extend_from_slice(username_bytes);
        packet.extend_from_slice(&message_len);
        packet.extend_from_slice(message_bytes);

        stream.write_all(&packet).unwrap();

        if code == 3 {
            println!("Déconnexion...");
            break;
        }

        let mut response = [0u8; 128];
        let size = stream.read(&mut response).unwrap();
        println!("Réponse serveur : {}", String::from_utf8_lossy(&response[..size]));
    }
}
