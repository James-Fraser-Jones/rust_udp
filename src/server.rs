use std::io;
use std::str;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind(addr).await?;
    println!("UDP Server listening on {}", addr);
    let mut buf = [0; 1024];
    loop {
        let (len, peer) = socket.recv_from(&mut buf).await?;
        let received_data = &buf[..len];
        let message = str::from_utf8(received_data).unwrap_or("Invalid UTF-8");
        println!("Server received '{}' from {}", message, peer);
        let response = format!("Hello, {}! I received your message.", peer);
        socket.send_to(response.as_bytes(), peer).await?;
        println!("Server sent response to {}", peer);
        if message == "exit" {
            println!("Server received 'exit' message, shutting down.");
            break;
        }
    }
    Ok(())
}
