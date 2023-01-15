use anyhow::Result;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
   
    loop {
        let incoming = listener.accept().await;

        match incoming {
            Ok((mut stream, _))=> {
                println!("accept new connection");

                tokio::spawn(async move {
                    handle_connection(&mut stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("error {}", e);
            }
        }
    }
}


async fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let mut buf = BytesMut::with_capacity(512);

    loop {
        let bytes_readed = stream.read(&mut buf).await?;
        if bytes_readed == 0 {
            print!("client close the connection");
            break;
        }

        stream.write("+PONG\r\n".as_bytes()).await?;
    }

    Ok(())
}