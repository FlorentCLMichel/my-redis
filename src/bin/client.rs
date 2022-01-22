use mini_redis::client;
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "6379";
const MAX_CAPACITY: usize = 32;

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {

    // create a new channel with fixed maximum capacity
    let (tx, mut rx) = mpsc::channel(MAX_CAPACITY);

    // spawn a task to manage commands
    let manager = tokio::spawn(async move {
    
        // establish a connection to the server
        let mut client = client::connect(&format!("{}:{}", ADDRESS, PORT)).await.unwrap();

        // start receiving messages
        while let Some(message) = rx.recv().await {
            use Command::*;
            match message {
                Get { key, resp } => {

                    let res = client.get(&key).await;

                    // ignore errors
                    let _ = resp.send(res);
                },
                Set { key, val, resp } => {

                    let res = client.set(&key, val).await;

                    // ignore errors
                    let _ = resp.send(res);
                }
            }
        }

    });

    // spawn a task to get a key
    let txc = tx.clone();
    let t1 = tokio::spawn(async move {
        
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get { 
            key: "hello".to_string(),
            resp: resp_tx
        };

        // send the GET request
        txc.send(cmd).await.unwrap();

        // await the response
        println!("GOT: {:?}", resp_rx.await);
    });
    
    // spawn a task to set a key
    let txc = tx.clone();
    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set { 
            key: "foo".to_string(), 
            val: "bar".into(),
            resp: resp_tx
        };
        
        // send the SET request
        txc.send(cmd).await.unwrap();

        // await the response
        println!("GOT: {:?}", resp_rx.await);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}


#[derive(Debug)]
enum Command {
    Get { 
        key: String ,
        resp: Responder<Option<Bytes>>
    }, 
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>
    }
}
