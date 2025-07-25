use std::collections::HashMap;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:5400")?;
    println!("Serveur DNS en écoute sur 127.0.0.1:5353");

    // Map des domaines connus
    let mut records = HashMap::new();
    records.insert("google.com", "8.8.8.8");
    records.insert("rust-lang.org", "13.37.13.37");
    records.insert("github.com", "140.82.121.3");
    records.insert("yahoo.com", "98.137.11.163");

    let mut buf = [0u8; 512];

    loop {
        let (size, src) = socket.recv_from(&mut buf)?;
        let domain = String::from_utf8_lossy(&buf[..size]);
        println!("Requête reçue pour : {}", domain);

        let response = match records.get(domain.trim()) {
            Some(ip) => ip.to_string(),
            None => "0.0.0.0".to_string(), // Adresse par défaut si inconnu
        };

        socket.send_to(response.as_bytes(), src)?;
        println!("Réponse envoyée à {} : {}", src, response);
    }
}
