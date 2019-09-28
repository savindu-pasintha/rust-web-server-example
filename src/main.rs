mod pool;
use pool::WorkerPool;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("localhost:8000").unwrap();
    let workers = WorkerPool::new(300);
    for connection_attempt in listener.incoming() {
        let stream = connection_attempt.unwrap();
        workers.run(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();
    let request_text = String::from_utf8_lossy(&buffer);
    if request_text.starts_with("GET / ") {
        stream.write(make_landing()).unwrap();
    }
    else if request_text.starts_with("GET /sleep ") {
        thread::sleep(Duration::from_secs(5));
        stream.write(make_landing()).unwrap();
    }
    else {
        stream.write(make_404()).unwrap();
    }
    stream.flush().unwrap();
}

fn make_landing() -> &'static [u8] {
    return b"HTTP/1.1 200 OK\nContent-Type: text/html\n
    <!DOCTYPE html>
    <html lang=\"en\">
    <head>
        <meta charset=\"utf-8\">
        <title>Hello!</title>
    </head>
    <body>
        <h1>Hello!</h1>
        <p>Hi from Rust</p>
    </body>
    </html>"
}

fn make_404() -> &'static [u8] {
    return b"HTTP/1.1 404 NOT FOUND\nContent-Type: text/html\n
    <!DOCTYPE html>
    <html lang=\"en\">
    <head>
        <meta charset=\"utf-8\">
        <title>Not Found</title>
    </head>
    <body>
        <h1>Not Found!</h1>
        <p>Get outta here!</p>
    </body>
    </html>"
}
