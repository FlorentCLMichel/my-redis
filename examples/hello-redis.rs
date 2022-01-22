use mini_redis::{client, Result};

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "6379";

#[tokio::main]
async fn main() -> Result<()> {

    // Open a connection to the mini-redis
    let mut client = client::connect(&format!("{}:{}", ADDRESS, PORT)).await?;

    // Set the key "hello" with value "How do you do?"
    let key = "hello";
    let value = "How do you do?";
    client.set(key, value.into()).await?;

    // Get the key "hello"
    let result = client.get(key).await?;
    
    if let Some(result) = result {
        println!("The value for the key {} is: {}", key, std::str::from_utf8(&result)?);
    } else {
        println!("No value for the key {}", key);
    }

    Ok(())
}
