use bytes::Bytes;
use futures::sink::SinkExt;
use futures::try_join;
use futures::StreamExt;
use rotmg_networking::{accept, connect, PolicyFile};
use rotmg_packets::RawPacket;
use std::net::{Ipv4Addr, Shutdown};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const KEYS: &[u8] = b"abcd";
static DATA: Bytes = Bytes::from_static(&[0, 0, 0, 6, 5, 6]);

#[tokio::test]
async fn test_basic_connections() {
    // start listener
    let mut listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
    let address = Box::leak(Box::new(listener.local_addr().unwrap())) as &'static _;
    println!("Listener started on {}", address);

    tokio::spawn(async move {
        loop {
            let (conn, address) = listener.accept().await.unwrap();
            println!("Accepting connection from {}", address);

            if let Some(mut conn) = accept(conn, KEYS, &PolicyFile::ALLOW_ALL).await.unwrap() {
                println!("ROTMG connection established with {}", address);
                for i in 0..3 {
                    let received = conn.next().await.unwrap().unwrap();
                    println!("Server received packet {}: {:x?}", i, received.bytes());
                    assert_eq!(received.bytes(), DATA);
                    conn.send(RawPacket::new(DATA.clone())).await.unwrap();
                    println!("Server sent packet {}", i);
                }
            } else {
                println!("Sent policy file to {}", address);
            }
        }
    });

    // start by sending a policy file request
    let policy_response = tokio::spawn(async move {
        let mut conn = TcpStream::connect(address).await.unwrap();
        conn.write_all(PolicyFile::REQUEST_MESSAGE).await.unwrap();
        conn.shutdown(Shutdown::Write).unwrap();

        let mut response = Vec::new();
        conn.read_to_end(&mut response).await.unwrap();
        let response = String::from_utf8(response).unwrap();

        assert_eq!(PolicyFile::ALLOW_ALL.contents(), response);
    });

    let rotmg_response = tokio::spawn(async move {
        let mut conn = connect(address, KEYS).await.unwrap();
        println!(
            "Client connected to {}",
            conn.get_ref().peer_addr().unwrap()
        );
        for i in 0..3 {
            conn.send(RawPacket::new(DATA.clone())).await.unwrap();
            println!("Client sent packet {}", i);
            let received = conn.next().await.unwrap().unwrap();
            println!("Client received packet {}: {:x?}", i, received.bytes());
            assert_eq!(received.bytes(), DATA);
        }
    });

    try_join!(policy_response, rotmg_response).unwrap();
}
