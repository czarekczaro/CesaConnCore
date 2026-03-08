use tokio::net::{UdpSocket, TcpListener, TcpStream};
use tokio::time::{sleep, timeout, Duration};
use std::io::{self, Read};
use core::net::SocketAddr;

static MAX_BROADCAST_DURATION: u8 = 20;
static BROADCAST_NAME: &str = "CesaConn Brodcast";

#[tokio::main]
async fn main() {
    let _handle = tokio::spawn(async {
        upd_broadcast_presence(BROADCAST_NAME, 5).await;
        udp_find_broadcaster().await;
    });

    println!("CesaConn Networker is running...");
    
    let _ = _handle.await;
}

async fn bind_socket(_addr: &str) -> UdpSocket {
    match UdpSocket::bind(_addr).await {
        Ok(s) => {
            println!("Succesfully binded socket.");
            s
        },
        Err(e) => {
            panic!("Failed to bind socket! | Error: {}", e)
        }
    }
}



async fn upd_broadcast_presence(_message: &str, _duration: u8) {

    let _duration = if _duration > MAX_BROADCAST_DURATION { MAX_BROADCAST_DURATION } else { _duration };
    let socket = bind_socket("0.0.0.0:6363").await;

    match socket.set_broadcast(true) {
        Ok(_) => println!("Succesfully enabled broadcast mode."),
        Err(e) => eprintln!("Failed to enable broadcast mode! | Error: {}", e),
    };

    for _tick in 0.._duration {

        let message = _message.as_bytes();

        match socket.send_to(message, "255.255.255.255:6363").await {

            Ok(message) => println!("Succesfully broadcasted: {} bytes | Data: {}", message, _message),
            Err(e) => eprintln!("Failed to broadcast presence: {}", e),

        }

        sleep(Duration::from_secs(1)).await;
    };

    match socket.set_broadcast(false) {
        Ok(_) => println!("Succefully disabled broadcast mode."),
        Err(e) => eprintln!("Failed disable broadcast mode! | Error: {}", e),
    };
}

async fn udp_find_broadcaster() -> Result<SocketAddr, io::Error> {
    let socket = bind_socket("0.0.0.0:6363").await;
    let mut buf = [0; 1024];

    println!("Searching for devices on network...");

    let result = timeout(
        Duration::from_secs(5),
        socket.recv_from(&mut buf)
    ).await;

    match result {
        Ok(Ok((len, addr))) => {
            let name = String::from_utf8_lossy(&buf[..len]);
            
            if name.starts_with(&BROADCAST_NAME) {
                println!("Found device: {} at IP: {}", name, addr.ip());
                Ok(addr)
            } else {
                eprintln!("Found unknown device: {} from {}", name, addr);
                Err(io::Error::new(io::ErrorKind::NotFound, "Device name mismatch"))
            }
        }
        Ok(Err(e)) => {
            eprintln!("Socket error: {}", e);
            Err(e)
        }
        Err(e) => {
            eprintln!("Timeout: Didnt find any device in 5 seconds!");
            Err(io::Error::new(io::ErrorKind::TimedOut, "Discovery timed out"))
        }
    }
}
