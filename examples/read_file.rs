use tokio::fs::{ File, remove_file };
use tokio::io::{ AsyncWriteExt, AsyncReadExt };

const N_BYTES: usize = 15;
const F_NAME: &str = "examples/foo.txt";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // create the file
    let mut file = File::create(F_NAME).await?;

    // write to the file
    let message = "Hello, World!";
    let n = file.write(message.as_bytes()).await?;
    if n < message.len() {
        eprintln!("Cold only write {} bytes of {}", n, message.len());
    }

    // read the file
    let mut file = File::open(F_NAME).await?;
    let mut buffer: [u8; N_BYTES] = [0; N_BYTES];
    let n = file.read(&mut buffer).await?;
    println!("Bytes read: {:?}", &buffer[..n]);
    println!("Conversion to string: {}", &String::from_utf8(buffer[..n].to_vec())?);

    // delete the file
    remove_file(F_NAME).await?;
    
    Ok(())
}
