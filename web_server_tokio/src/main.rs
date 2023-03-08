use std::net::SocketAddr;

use tokio::{net::TcpListener, io::{AsyncWriteExt, AsyncBufReadExt, BufReader}, sync::broadcast};

// esta macro permite que la función main pueda ser Async
#[tokio::main]
async fn main() {
    // chat server multiuser
    
    // crear servidor que escuche por conexiones
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);
    
    loop {
        // quedarnos esperando conexiones
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // crear hilos con tokio
        tokio::spawn(async move {
            // separar el socket en dos partes
            let (reader, mut writer) = socket.split();
        
            // crear un buffer para recibir los datos que nos envían desde las conexiones
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            
            // para que se mantenga escuchando
            loop {

                tokio::select! {
                    _ = reader.read_line(&mut line) => {
                        tx.send((line.clone(), _addr)).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, msg_addr) = result.unwrap();
                        if _addr != msg_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}