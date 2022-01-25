/// A simplified Select implementation

use tokio::sync::oneshot;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {

        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(());
        }
        
        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(());
        }

        Poll::Pending
    }
}


#[tokio::main]
async fn main() {
    
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn( async move { 
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let _ = tx1.send("test1");
    });
    
    tokio::spawn( async move { 
        std::thread::sleep(std::time::Duration::from_millis(100));
        let _ = tx2.send("test2"); 
    });

    MySelect { rx1, rx2 }.await;
}
