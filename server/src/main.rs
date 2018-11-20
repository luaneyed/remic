mod storage;
mod thread_pool;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::str::from_utf8;
use std::io::ErrorKind::{InvalidInput, NotFound};
use std::io::Error;

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
                if n == 0 { //  connection was closed
                    break;
                }

                let received_string = from_utf8(&read).expect("Found invalid UTF-8").trim_matches(char::from(0));
                println!("Received {}", received_string);
                
                let result_string = match execute(received_string.split(' ')) {
                    Ok(ok_result) => {
                        ok_result.to_string()
                    },
                    Err(error) => {
                        error.get_ref().unwrap().to_string()
                    }
                };

                stream.write(result_string.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(err) => {
                println!("Failed to read stream. {}", err);
            }
        }
    }
}

fn execute<'a, 'b, T: Iterator<Item=&'a str>> (mut args: T)-> Result<&'b str, Error> {
    match args.next() {
        Some("get") => {
            if let Some(ref key) = args.next() {
                let stored: Option<&'b str> = storage::get(key);
                if let Some(val) = stored {
                    return Ok(val);
                }
                
                return Err(Error::new(NotFound, "Not Found"));
            }

            Err(Error::new(InvalidInput, "Key is not given"))
        },
        Some("set") => {
            if let Some(key) = args.next() {
                if let Some(val) = args.next() {
                    storage::set(key, val);
                    return Ok("Set Successfully");
                }
                return Err(Error::new(InvalidInput, "Value is not given"));
            }
            
            Err(Error::new(InvalidInput, "Key is not given"))
        },
        Some(_) => {
            Err(Error::new(InvalidInput, "Unknown Command"))
        },
        None => Err(Error::new(InvalidInput, "Command is not given"))
    }
}
