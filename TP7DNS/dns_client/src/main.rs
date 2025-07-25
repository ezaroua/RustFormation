use std::net::UdpSocket;
use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.connect("127.0.0.1:5400")?;

    print!("Entrez le domaine à résoudre : ");
    io::stdout().flush().unwrap();

    let mut domaine = String::new();
    io::stdin().read_line(&mut domaine)?;
    let domaine = domaine.trim();

    socket.send(domaine.as_bytes())?;

    let mut buf = [0u8; 512];
    let size = socket.recv(&mut buf)?;
    let ip = String::from_utf8_lossy(&buf[..size]);

    println!("Adresse IP de {} : {}", domaine, ip);
    Ok(())
}
