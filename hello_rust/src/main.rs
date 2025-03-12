use std::{
    fs,
    io::{BufRead, BufReader, Result, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello_rust::{Status, ThreadPool};

fn main() {
    // unwrap() - bind should only error on ports below 1024 or ports already bound
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        // TODO: Handle errors properly
        let mut stream = stream.unwrap();

        pool.execute(move || match handle_connection(&mut stream) {
            Ok(_) => {}
            Err(_) => {
                let _ = write_response(&mut stream, Status::new(500).unwrap(), "Server I/O Error!");
            }
        });
    }
}

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let buf_reader = BufReader::new(&mut *stream);
    let request_line = match buf_reader.lines().next() {
        Some(val) => val?,
        None => "404".to_string(),
    };

    let (status, body) = match &request_line[..] {
        "GET / HTTP/1.1" => (200, "www/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            (200, "www/index.html")
        }
        _ => (404, "www/404.html"),
    };

    let status = Status::new(status).expect("Developer passed in a weird number code!");
    let body = fs::read_to_string(body)?;

    write_response(stream, status, &body)?;

    Ok(())
}

fn write_response(stream: &mut impl Write, header: Status, body: &str) -> Result<()> {
    let header_text = header.to_string();

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        header_text,
        body.len(),
        body
    );

    stream.write_all(response.as_bytes())?;

    Ok(())
}
