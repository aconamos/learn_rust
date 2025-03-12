use std::future::Future;
use std::pin::*;
use std::time::Duration;

use trpl::Either;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx2 = tx.clone();

        let tx_fut_1 = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let tx_fut_2 = pin!(async move {
            let vals = vec![
                String::from("More"),
                String::from("data"),
                String::from("for"),
                String::from("you!"),
            ];

            for val in vals {
                tx2.send(val).unwrap();
                trpl::sleep(Duration::from_millis(300)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut_1, tx_fut_2, rx_fut];

        trpl::join_all(futures).await;
    });
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    let time_future = trpl::sleep(max_time);

    match trpl::race(future_to_try, time_future).await {
        Either::Left(out) => Ok(out),
        Either::Right(_) => Err(max_time),
    }
}
