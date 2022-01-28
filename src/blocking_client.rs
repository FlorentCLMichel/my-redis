/// A simple synchronous interface to the mini-redis

use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;

pub use crate::client::Message;

/// established connection with a Redis server
pub struct BlockingClient {

    // the asynchronous client
    inner: crate::client::Client,

    // a `current_thread` runtime for executing operations on the client in a blocking manner
    rt: Runtime,
}

pub fn conect<T: ToCocketAddrs>(addr: T) -> crate::Result<BlockingClient> {
    let rt = tokio::runtime::Builder::new_current_thread()
              .enable_all()
              .build()?;
    let inner = rt.block_on(crate::client::connect(addr))?;
    Ok(BlockingClient { inner, rt })
}
