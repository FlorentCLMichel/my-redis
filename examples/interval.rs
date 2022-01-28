/// A simple implementation of the `Stream` train for an `Interval` structure
///
/// (requires an implementation of `Delay`; one can be found in `simple_future.rs`)

use tokio_stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

const DELAY_MS: usize = 10;

struct Interval {
    rem: usize, 
    delay: Delay
}

impl Stream for Interval {

    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) 
        -> Poll<Option<()>>
    {
        if self.rem = 0 {
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + Duration::from_millis(DELAY_MS);
                self.delay = Delay { when }; 
                self.rem -= 1;
                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
