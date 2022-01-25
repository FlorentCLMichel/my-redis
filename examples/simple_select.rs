use tokio::sync::oneshot;

const N_TASKS: usize = 3;

#[tokio::main]
async fn main() {

    let mut channels = Vec::with_capacity(N_TASKS); 
    for _ in 0..N_TASKS {
        channels.push(oneshot::channel());
    }
    
    // spawn the tasks
    let mut i = 0;
    let mut receivers = Vec::with_capacity(N_TASKS);
    while let Some(channel) = channels.pop() {
        i += 1;
        let tx = channel.0;
        tokio::spawn(async move {
            let _ = tx.send(i.to_string());
        });
        receivers.push(channel.1);
    }

    // see which task completes first
    tokio::select! {
        Ok(res) = receivers.pop().unwrap() => {
            println!("Task {} completed first", res);
        }
        Ok(res) = receivers.pop().unwrap() => {
            println!("Task {} completed first", res);
        }
        Ok(res) = receivers.pop().unwrap() => {
            println!("Task {} completed first", res);
        }
    }
}
