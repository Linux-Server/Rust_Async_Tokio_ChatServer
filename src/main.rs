#![allow(non_snake_case)]
use std::thread;
use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() {
    let _x = sam().await;

}


async fn sam()->i32{
    println!("Before sleep..");
    // thread::sleep(Duration::from_secs(5));
    let _b = sleep(Duration::from_secs(5)).await;
    println!("After sleep..");
    10

}



