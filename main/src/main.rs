use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio;
use tokio::sync::mpsc;

use core::client::ClientType;
use reader::client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, mut rx) = mpsc::channel(32);
    
    tokio::spawn(async move {
        while let Some(i) = rx.recv().await {
            println!("got = {}", i);
        }
    });    

    loop {
        let (mut socket, _) = listener.accept().await?;
        let local_tx = tx.clone();

        tokio::spawn(async move {
            let client_type = client::read_type(&mut socket).await;
            match client_type {
                Ok(ClientType::Consumer) => {
                    read_messages_loop(&mut socket, local_tx).await;
                }
                Ok(ClientType::Producer) => {}
                Err(err) => panic!(err),
            }
        });
    }
}

async fn read_message_x(stream: &mut TcpStream) -> Vec<u8> {
    let mut message_size = match reader::client::read_message_size_header(stream).await {
        Ok(_message_size) => _message_size,
        Err(err) => panic!(err),
    };

    let mut result = vec![];

    loop {
        let (_usize, _message_part) = match reader::client::read_message_part(stream).await {
            Ok((_usize, _message_part)) => (_usize, _message_part),
            Err(err) => panic!(err),
        };

        message_size = message_size - (_usize as u32);
        for x in 0.._usize {
            result.push(_message_part[x]);
        }
        if message_size == 0 {
            break;
        }
    }

    return result;
}

async fn read_messages_loop(stream: &mut TcpStream, target_channel: mpsc::Sender<u8>) {
    loop {
        let message = read_message_x(stream).await;
        if let Err(_) = target_channel.send(22).await {
            println!("receiver dropped");
            return;
        }
    }
}
