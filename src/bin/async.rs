use std::net::TcpStream;
use std::io::{Result};

fn main()-> Result<()>{
    println!("Async functiopn test");
    let mut stream = TcpStream::connect("127.0.0.1:3000")?;
    println!("Conection stat : {:?}", stream);
    Ok(())

}

