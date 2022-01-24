use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use futures::task::{self, ArcWake};
use crossbeam::channel; 
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay::new(when);
        let out = future.await;
        assert_eq!(out, ());
    });
}

struct Delay {
    when: Instant,
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Delay {
    fn new(when: Instant) -> Self {
        Self {
            when, 
            waker: Option::<Arc<Mutex<Waker>>>::None
        }
    }
}

impl Future for Delay {
    type Output = (); 

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<()> 
    {
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }

        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

struct MiniTokio {
    scheduled: channel::Receiver::<Arc<Task>>, 
    sender: channel::Sender::<Arc<Task>>,
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>, 
    executor: channel::Sender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone());
    }

    fn poll(self: Arc<Self>) {
        
        // create a waker from the `Task` instance
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        // no other thread ever tries to lock the future
        let mut future = self.future.try_lock().unwrap();

        // poll the future
        let _ = future.as_mut().poll(&mut cx);
    }

    // spawn a new task with the given future
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
        where F: Future<Output = ()> + Send + 'static
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)), 
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}

impl MiniTokio {
    fn new() -> Self {
        let (sender, scheduled) = channel::unbounded();
        Self { scheduled, sender }
    }

    // spawn a future
    fn spawn<F>(&mut self, future: F)
        where F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    fn run(&self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}
