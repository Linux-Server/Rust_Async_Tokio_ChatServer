#![allow(non_snake_case)]
use std::thread;
use tokio::time::{sleep, Duration};
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    // let _x = sam().await;
    // we nee a TCP listener
    let listener = TcpListener::bind("127.0.0.1:3000").await;
    match listener{
        Ok(val)=> {
            println!("Its a success");
            loop{
                let (socker,addr) = val.accept().await.unwrap();
                println!("The stream : {:?}", socker);
                println!("The addr : {:?}", addr);
            }


        },
        Err(err) => println!("Its a failure")
    }




}


async fn sam()->i32{
    println!("Before sleep..");
    // thread::sleep(Duration::from_secs(5));
    let _b = sleep(Duration::from_secs(5)).await;
    println!("After sleep..");
    10

}



