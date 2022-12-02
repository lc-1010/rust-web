
use std::{time};
use std::io::Read;
use std::{net::TcpStream, io::Write};
use std::str;
fn main() {

    let t  = time::SystemTime::now();
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    let duration = t.elapsed().map_err(|e| 
        format!("{:?}",e));
    let hello = format!("{:?}",duration.unwrap()); 
     let mut hi = String::from(" hi");
     hi.push_str(&hello);
    stream.write(  hi.as_bytes()).unwrap();
    let l = hi.chars().count();
    println!("l={}",l);
    let mut buffer = vec![0;l];
    stream.read(&mut buffer).unwrap();
    
    println!(
        "Hello, world! from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
