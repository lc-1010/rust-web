use std::{net::{TcpListener}, io::Read,io::Write};

fn main() {

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    
    println!("Hello, world! runing on port 3000..");
    
    for stream in  listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0;1024]; 
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }    
}
