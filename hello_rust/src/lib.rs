use std::fmt::Display;
use std::io::{Error, Result};
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(thread_count: u8) -> ThreadPool {
        assert!(thread_count > 0);

        let mut workers = Vec::with_capacity(thread_count as usize);

        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));

        for i in 0..thread_count {
            workers.push(Worker::new(i, Arc::clone(&rx)));
        }

        ThreadPool { workers, tx }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.tx.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // drain(range) will take ownership of everything in the vec.
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: u8,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u8, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect(
                    "Mutex could be poisoned! A thread may have panicked while holding the lock!",
                )
                .recv()
                .unwrap();

            println!("Worker {id} got a job!");

            job();
        });

        Worker { id, thread }
    }
}

#[allow(dead_code)]
enum HTTPVerb {
    GET,
    HEAD,
    PUT,
    POST,
    TRACE,
    Nil,
}

// There is DEFINITELY a better way to do this.
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
#[allow(dead_code)]
pub struct Status {
    num: u16,
    verb: HTTPVerb,
}

impl Status {
    pub fn new(num: u16) -> Result<Status> {
        let _ = description_from_num(num)?;

        // Verbs don't have much functionality, here...
        let verb = match num {
            200 => HTTPVerb::GET,
            _ => HTTPVerb::Nil,
        };

        Ok(Status { num, verb })
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.num,
            // unwrap() - This should never be an error because this fn can only be called on
            // a valid Status object.
            description_from_num(self.num).unwrap()
        )?;

        Ok(())
    }
}

fn description_from_num(num: u16) -> Result<&'static str> {
    Ok(match num {
        100 => "Continue",
        101 => "Switching Protocols",
        102 => "Processing",
        103 => "Early Hints",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        207 => "Multi-Status",
        208 => "Already Reported",
        226 => "IM Used",
        300 => "Multiple Choices",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        406 => "Not Acceptable",
        407 => "Proxy Authentication Required",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "Length Required",
        412 => "Precondition Failed",
        413 => "Content Too Large",
        414 => "URI Too Long",
        415 => "Unsupported Media Type",
        416 => "Range Not Satisfiable",
        417 => "Expectation Failed",
        418 => "I'm a teapot",
        421 => "Misdirected Request",
        422 => "Unprocessable Content",
        423 => "Locked",
        424 => "Failed Dependency",
        425 => "Too Early",
        426 => "Upgrade Required",
        428 => "Precondition Required",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        451 => "Unavailable For Legal Reasons",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        506 => "Variant Also Negotiates",
        507 => "Insufficient Storage",
        508 => "Loop Detected",
        510 => "Not Extended",
        511 => "Network Authentication Required",
        c => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("{c} is not a valid HTTP code!"),
            ))
        }
    })
}
