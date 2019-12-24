use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind::ConnectionAborted;
use std::net::TcpStream;
use std::str;
use std::string::String;

pub fn get(key: &str) -> Result<String, Error> {
    execute(&format!("get {}", key))
}

pub fn set(key: &str, val: &str) -> Result<String, Error> {
    execute(&format!("set {} {}", key, val))
}

pub fn del(key: &str) -> Result<String, Error> {
    execute(&format!("del {}", key))
}

pub fn flush() -> Result<String, Error> {
    execute("flush")
}

pub fn execute(command: &str) -> Result<String, Error> {
    let mut stream = TcpStream::connect("127.0.0.1:9736").unwrap();
    let _ = stream.write(command.as_bytes());
    stream.flush().unwrap();

    let mut read = [0; 1028];

    match stream.read(&mut read) {
        Ok(n) => {
            if n != 0 {
                return Ok(String::from_utf8(read.to_vec()).unwrap());
            }

            Err(Error::new(ConnectionAborted, "Connection was closed!"))
        }
        Err(err) => Err(err),
    }
}
