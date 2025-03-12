use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let a = Arc::new(Mutex::new(5i32));
    let b = Arc::new(Mutex::new(5i32));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);
    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);

    let a_first_handle = thread::spawn(move || {
        let mut a = a1.lock().unwrap();
        *a = 4;

        thread::sleep(Duration::from_millis(10));

        let mut b = b1.lock().unwrap();
        *b += *a;
    });

    let b_first_handle = thread::spawn(move || {
        let mut b = b2.lock().unwrap();
        *b = 4;

        thread::sleep(Duration::from_millis(10));

        let mut a = a2.lock().unwrap();
        *a += *b;
    });

    // Deadlock mitigation strategy 1:
    // Make sure the resources are acquired in a common order. So, b_first_handle should acquire the lock on a first.
    // Apparently, this is guaranteed to work.
    // https://stackoverflow.com/questions/6012640/locking-strategies-and-techniques-for-preventing-deadlocks-in-code
    // Apparently, this is the only real way to avoid it.

    a_first_handle.join().unwrap();
    b_first_handle.join().unwrap();

    println!("a: {}, b: {}", a.lock().unwrap(), b.lock().unwrap());
}

#[allow(dead_code)]
fn add() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
