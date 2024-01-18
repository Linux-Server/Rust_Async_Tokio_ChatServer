use mini_redis::{client, Result};
use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() -> Result<()> {
    println!("The starting...");
    let mut client = client::connect("0.0.0.0:3000").await.unwrap();

    // Set the key "hello" with value "world"
    let x = client.set("bello", "world".into()).await;

    match x {
        Ok(val) => println!("The val is {:?}", val),
        Err(err) => println!("the custom error is {:?}", err),
    }

    println!("The end...");

    // Get key "hello"
    let result = client.get("bello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
