use std::io::{Read, Write};
use std::net::TcpStream;
use wjp::{Deserialize, Serialize};

use pet_store::Test;

fn main() {
    for iter in 0..100 {
        let example = Test::from(iter.to_string());
        let mut stream = TcpStream::connect("0.0.0.0:6969").unwrap();

        let _ = stream.write_all(example.json().as_bytes());

        let mut string = String::new();
        let _ = stream.read_to_string(&mut string);
        println!("{}", string);
        println!("{:?}", Test::deserialize(string));
    }
}