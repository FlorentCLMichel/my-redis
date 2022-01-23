use tokio::io::{ self, AsyncReadExt, AsyncWriteExt };
use tokio::net::TcpListener;

const ADDR: &str = "127.0.0.1";
const PORT: &str = "6142";
const BUFFER_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind(&format!("{}:{}", ADDR, PORT)).await?;
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buf = vec![0; BUFFER_SIZE];
            
            match socket.read(&mut buf).await {
                
                // if the connection is closed, return immediately
                Ok(0) => return, 

                Ok(n) => {
                    if socket.write_all(&buf[..n]).await.is_err() {
                        // unexpected socket error → stop processing
                        return
                    }
                },

                // unexpected socket error → stop processing
                Err(_) => return
            };
        });
    }
}
