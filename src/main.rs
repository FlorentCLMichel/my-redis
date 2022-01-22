use tokio::net::{ TcpListener, TcpStream };
use mini_redis::{ Connection, Frame, Result };
use mini_redis::Command::{ self, Get, Set };
use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use bytes::Bytes;

// type for a database which can be shared between threads
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "6379";


#[tokio::main]
async fn main() {

    // create a listener bound the listener to the address
    let listener = TcpListener::bind(&format!("{}:{}", ADDRESS, PORT)).await.unwrap();

    // initialize the database
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        
        // get the socket of the new connection
        let (socket, _) = listener.accept().await.unwrap();

        // clone the handle to the database
        let db = db.clone();

        // spawn a task to process the socket
        tokio::spawn(async move {
            process(socket, db).await.unwrap();
        });
    }
}


async fn process(socket: TcpStream, db: Db) -> Result<()> {

    // read/write as redis ‘frames’ instead of a byte stream
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await? {

        let response = match Command::from_frame(frame)? {
            
            // store the value as a `Vec<u8>`
            Set(cmd) => {
                
                // lock the database
                let mut db = db.lock().unwrap();

                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },

            Get(cmd) => {
                
                // lock the database
                let db = db.lock().unwrap();
                
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
