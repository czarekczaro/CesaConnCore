use core::net::SocketAddr;
use std::sync::Arc;
use tokio::select;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub enum TcpNetworkerErrors {
    FailedToBindSocket,
    FailedToAcceptConnection,
}

pub static DEFAULT_ADDR: &str = "127.0.0.1:6969";
pub static AUTH_BUFFER_SIZE: usize = 1024;
pub static BUFFER_SIZE: usize = 4096;

// TODO : Auth
pub async fn recv_handler(
    listener: &TcpListener,
    addr: &str,
    key: Arc<Mutex<[u8; 32]>>,
    trusted_addrs: Arc<Mutex<Vec<SocketAddr>>>,
    cancellation_token: &CancellationToken,
) {

    //logic here
}

// TODO : Auth
pub async fn connect_handler(
    connection: (TcpStream, SocketAddr),
    key: &mut [u8; 32],
    trusted_addrs: &mut Vec<u8>,
) {

    //logic here
}

pub async fn recv(
    listener: Arc<Mutex<TcpListener>>,
    addr: &str,
    key: Arc<Mutex<[u8; 32]>>,
    trusted_addrs: Arc<Mutex<Vec<SocketAddr>>>,
    cancellation_token: &CancellationToken,
) -> Result<(), TcpNetworkerErrors> {
    let cloned_token = cancellation_token.clone();

    println!("Listening on: {addr}");

    loop {
        let incoming_connection = listener
            .accept()
            .await
            .map_err(|_| TcpNetworkerErrors::FailedToAcceptConnection)?;

        let join_handle = tokio::spawn(async move {
            select! {
                _ = cloned_token.cancelled() => {
                // The token was cancelled
                println!("Cancelled");
                5
                },
                _ = recv_handler(incoming_connection, key, trusted_addrs) => {
                    println!("Finished");
                    99
                }
            }
        });
    }

    Ok(())
}

pub async fn connect(addr: &str) {
    match TcpStream::connect(addr).await {
        Ok(s) => {
            let addr: SocketAddr = addr.parse().unwrap();
            let connection = (s, addr);

            tokio::spawn(async move { connect_handler(connection).await });
        }

        Err(_) => {
            eprintln!("Failed to connect!");
        }
    };
}
