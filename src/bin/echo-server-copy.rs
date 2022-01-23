use tokio::io; 
use tokio::net::TcpListener;

const ADDR: &str = "127.0.0.1";
const PORT: &str = "6142";

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind(&format!("{}:{}", ADDR, PORT)).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            
            // split the TCP stream into a reader handle and a writer handle
            let (mut rd, mut wr) = socket.split();
            
            // copy the reader to the writer and print an error if unduccessful
            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}
