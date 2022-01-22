use tokio::net::{ TcpListener, TcpStream };
use mini_redis::{ Connection, Frame, Result };
use mini_redis::Command::{ self, Get, Set };
use std::collections::HashMap;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "6379";


#[tokio::main]
async fn main() {

    // create a listener bound the listener to the address
    let listener = TcpListener::bind(&format!("{}:{}", ADDRESS, PORT)).await.unwrap();

    loop {
        
        // get the socket of the new connection
        let (socket, _) = listener.accept().await.unwrap();

        // spawn a task to process the socket
        tokio::spawn(async move {
            process(socket).await.unwrap();
        });
    }
}


async fn process(socket: TcpStream) -> Result<()> {

    // read/write as redis ‘frames’ instead of a byte stream
    let mut connection = Connection::new(socket);

    // hashmap used to store data
    let mut db = HashMap::new();

    while let Some(frame) = connection.read_frame().await? {

        let response = match Command::from_frame(frame)? {
            
            // store the value as a `Vec<u8>`
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            },

            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            },

            // in all othe cases, reply with an error
            _ => Frame::Error("unimplemented".to_string())
        };

        connection.write_frame(&response).await?;
    }

    Ok(())
}
