use std::io::{Write, Read};
use std::net::TcpStream;

fn main() {

    let mut stream = TcpStream::connect("google.com:80").unwrap();


    let request = "GET / HTTP/1.1\r\nHost: google.com\r\nConnection: close\r\n\r\n";
    stream.write_all(request.as_bytes()).unwrap();


    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();


    println!("{}", String::from_utf8_lossy(&buffer));
}
