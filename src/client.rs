use std::io;
use std::str;
use std::time::Duration;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server_addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(server_addr).await?;
    println!("UDP Client connected to {}", server_addr);
    let message = "Hello from the client!";
    socket.send(message.as_bytes()).await?;
    println!("Client sent '{}'", message);
    let mut buf = [0; 1024];
    //set a timeout for receiving to prevent indefinite blocking if the server doesn't respond
    let recv_result = tokio::time::timeout(Duration::from_secs(5), socket.recv(&mut buf)).await;
    match recv_result {
        Ok(Ok(len)) => {
            let received_data = &buf[..len];
            let response = str::from_utf8(received_data).unwrap_or("Invalid UTF-8");
            println!("Client received response: {}", response);
        }
        Ok(Err(e)) => {
            eprintln!("Client received error: {}", e);
        }
        Err(_) => {
            eprintln!("Client timed out waiting for server response.");
        }
    }
    //give the server a moment to process the previous message before sending exit
    tokio::time::sleep(Duration::from_millis(100)).await;
    let exit_message = "exit";
    socket.send(exit_message.as_bytes()).await?;
    println!("Client sent 'exit' message to server.");
    Ok(())
}
