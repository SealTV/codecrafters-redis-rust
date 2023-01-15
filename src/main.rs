use anyhow::Result;
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let incoming = listener.accept().await;

        match incoming {
            Ok((mut stream, _)) => {
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
    let mut buf = [0;512];

    loop {
        let readed = stream.read(&mut buf[..]).await?;
        println!("bytes readed: {}", readed);
        if readed == 0 {
            println!("client close the connection");
            break;
        }


        let s = str::from_utf8(&buf[..readed]).unwrap();
        println!("readed str {}", s);

        stream.write("+PONG\r\n".as_bytes()).await?;
    }

    Ok(())
}
