use std::{sync::mpsc, thread, time::Duration};

#[allow(dead_code)]
enum MyOption<T> {
    Some(T),
    None,
}

#[allow(dead_code)]
impl<T> MyOption<T> {
    // pub fn unwrap_or_else<F>(self, f: F) -> T
    // where
    // `Fn() -> X` seems to just be syntax sugar for the following line, however with some additional
    // functionality that makes it better than the line below.
    //     F: FnOnce<(), Output = T>,
    // {
    //     match self {
    //         MyOption::Some(x) => x,
    //         MyOption::None => f(),
    //     }
    // }

    pub fn unwrap_or_else_val(self, val: T) -> T {
        match self {
            MyOption::Some(x) => x,
            MyOption::None => val,
        }
    }
}

fn main() {
    match std::env::args().nth(1).unwrap_or("_".to_string()).as_str() {
        "spawn" | "spawning" => spawning(),
        "message" | "messaging" => messaging(),
        _ => {
            println!("Valid options: spawn|spawning, message|messaging,")
        }
    }
}

fn messaging() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];

        for val in vals {
            tx.send(val).unwrap();
        }

        tx.send(String::from("Waiting one second...")).unwrap();

        thread::sleep(Duration::from_secs(1));

        tx.send(String::from("Rubber Ducky!")).unwrap();
    });

    for r in rx {
        println!("Got: {r}");
    }
}

fn spawning() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("{i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=5 {
        println!("{i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
