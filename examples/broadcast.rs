use tokio_stream::StreamExt;
use mini_redis::client;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "6379";

async fn publish() -> mini_redis::Result<()> {

    // connect to the server
    let mut client = client::connect(&format!("{}:{}", ADDRESS, PORT)).await?;
    
    // publish some data
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;

    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {

    // connect to the server
    let client = client::connect(&format!("{}:{}", ADDRESS, PORT)).await?;

    // get the published messages as a stream
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    let messages = subscriber.into_stream()
                             .filter(|msg| msg.is_ok())
                             .map(|msg| { msg.unwrap().content });

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got: {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    
    // spawn a task to publish the messages
    tokio::spawn(async {
        publish().await
    });

    // read the messages
    subscribe().await?;

    println!("DONE");

    Ok(())
}
