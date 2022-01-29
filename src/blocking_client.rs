/// A simple synchronous interface to the mini-redis

use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;
use bytes::Bytes;
use std::time::Duration;

pub use crate::client::Message;

/// established connection with a Redis server
pub struct BlockingClient {

    // the asynchronous client
    inner: crate::client::Client,

    // a `current_thread` runtime for executing operations on the client in a blocking manner
    rt: Runtime,
}

/// a client that has entered pub/sub mode
pub struct BlockingSubscriber {
    
    // the asynchronous subscriber
    inner: crate::client::Subscriber,

    // a `current_thread` runtime for executing operations on the client in a blocking manner
    rt: Runtime,
}

pub fn connect<T: ToCocketAddrs>(addr: T) -> crate::Result<BlockingClient> {
    let rt = tokio::runtime::Builder::new_current_thread()
              .enable_all()
              .build()?;
    let inner = rt.block_on(crate::client::connect(addr))?;
    Ok(BlockingClient { inner, rt })
}

impl BlockingClient {

    pub fn get(&mut self, key: &str) -> crate::Result<Option<Bytes>> {
        self.rt.block_on(self.inner.get(key))
    }

    pub fn get(&mut self, key: &str, vaue: Bytes) -> crate::Result<()> {
        self.rt.block_on(self.inner.set(key, value))
    }

    pub fn set_expires(&mut self, key: &str, value: Bytes, expiration: Duration) 
        -> crate::Result<()> 
    {
        self.rt.block_on(self.inner.set_expires(key, value, expiration))
    }

    pub fn publish(&mut self, channel: &str, message: Bytes) -> crate::Result<u64>
    {
        self.rt.block_on(self.inner.publish(channel, message))
    }

    pub fn subscribe(self, channels: Vec<String>) -> crate::Result<BlockingSubscriber>
    {
        let subscriber = self.rt.block_on(self.inner.subscribe(channels))?;
        Ok(BlockingSubscriber { 
            inner: subscriber,
            rt: self.rt
        })
    }
}

impl BlockingSubscriber {

    pub fn get_subscribed(&self) -> &{String] {
        self.inner.get_suscribed()
    }
    
    pub fn next_message(&mut self) -> crate::Result<Option<Message>> {
        self.rt.block_on(self.inner.next_message())
    }
    
    pub fn subscribe(&mut self, channels: &[String]) -> crate::Result<()> {
        self.rt.block_on(self.inner.subscriber(channels))
    }
    
    pub fn unsubscribe(&mut self, channels: &[String]) -> crate::Result<()> {
        self.rt.block_on(self.inner.unsubscriber(channels))
    }
}

