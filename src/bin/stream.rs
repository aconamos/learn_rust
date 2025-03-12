use std::pin::pin;
use std::time::Duration;

use trpl::{ReceiverStream, Stream, StreamExt};

struct Screen {
    field: Option<i128>,
}

fn main() {
    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(data) => println!("{data}"),
                Err(err) => eprintln!("error: {}", err),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
