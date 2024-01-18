#![allow(non_snake_case)]
use std::thread;
use tokio::time::{sleep, Duration};
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Frame,Connection};
#[tokio::main]
async fn main() {
    // let _x = sam().await;
    // we nee a TCP listener
    let listener = TcpListener::bind("0.0.0.0:3000").await;
    match listener{
        Ok(val)=> {
            println!("Listening on 3000");
            loop{
                let (socket,addr) = val.accept().await.unwrap();

                let response = tokio::spawn(async move{
                    process(socket).await;
                }).await;
             
            
               
            }


        },
        Err(err) => println!("Its a failure")
    }




}


async fn process(socket:TcpStream){
    let mut conn = Connection::new(socket);
    let data = conn.read_frame().await;
    match data{
        Ok(val)=>{
            match val{
                Some(frame) => {
                    println!("The frame received : {:?}", frame);
                    let res = Frame::Error("Got the package...".to_owned());
                    let x = conn.write_frame(&res).await;
                   
                },
                None => println!("Not frame found")
            }
        },
        Err(err) => {
            println!("Error occured : {:?}",err)
        }
    }

}



