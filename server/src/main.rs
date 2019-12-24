mod commands;
mod storage;
mod thread_pool;

use commands::Command;
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind::InvalidInput;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::from_utf8;
use std::sync::{Arc, RwLock};
use storage::Store;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9736").unwrap();
    let pool = thread_pool::ThreadPool::new(4);
    let storage = Arc::new(RwLock::new(Store::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let storage = storage.clone();
        pool.execute(|| {
            handle_connection(storage, stream);
        });
    }
}

fn handle_connection(storage: std::sync::Arc<std::sync::RwLock<Store>>, mut stream: TcpStream) {
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    //  connection was closed
                    break;
                }

                let received_string = from_utf8(&read)
                    .expect("Found invalid UTF-8")
                    .trim_matches(char::from(0));
                println!("Received {}", received_string);

                match get_command(received_string.split(' ')) {
                    Ok(command) => match command {
                        Command::Get { key } => {
                            let c = storage.read().unwrap();
                            let stored: Option<&String> = c.get(key);
                            let result = if let Some(val) = stored {
                                val.as_bytes()
                            } else {
                                b"Not Found"
                            };
                            stream.write_all(result)
                        }
                        Command::Set { key, val } => {
                            storage
                                .write()
                                .unwrap()
                                .set(key.to_string(), val.to_string());
                            stream.write_all(b"Set Successfully")
                        }
                        Command::Del { key } => {
                            storage.write().unwrap().del(key);
                            stream.write_all(b"Deleted Successfully")
                        }
                        Command::FlushAll => {
                            storage.write().unwrap().flushall();
                            stream.write_all(b"Flushed Successfully")
                        }
                    },
                    Err(error) => stream.write_all(error.get_ref().unwrap().to_string().as_bytes()),
                }
                .unwrap();

                stream.flush().unwrap();
            }
            Err(err) => {
                println!("Failed to read stream. {}", err);
            }
        }
    }
}

fn get_command<'a, T: Iterator<Item = &'a str>>(mut args: T) -> Result<Command<'a>, Error> {
    match args.next() {
        Some("get") => {
            if let Some(ref key) = args.next() {
                Ok(Command::Get { key })
            } else {
                Err(Error::new(InvalidInput, "Key is not given"))
            }
        }
        Some("set") => {
            if let Some(key) = args.next() {
                if let Some(val) = args.next() {
                    Ok(Command::Set { key, val })
                } else {
                    Err(Error::new(InvalidInput, "Value is not given"))
                }
            } else {
                Err(Error::new(InvalidInput, "Key is not given"))
            }
        }
        Some("del") => {
            if let Some(ref key) = args.next() {
                Ok(Command::Del { key })
            } else {
                Err(Error::new(InvalidInput, "Key is not given"))
            }
        }
        Some("flushall") => Ok(Command::FlushAll),
        Some(_) => Err(Error::new(InvalidInput, "Unknown Command")),
        None => Err(Error::new(InvalidInput, "Command is not given")),
    }
}
