mod thread_pool;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::str::from_utf8;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9736").unwrap();
    let pool = thread_pool::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }

                println!("Received {}", from_utf8(&read).unwrap());

                let response = &read[0..n];
                stream.write(response).unwrap();
                stream.flush().unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}
