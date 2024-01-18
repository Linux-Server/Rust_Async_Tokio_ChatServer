use simple_logger::SimpleLogger;
use tokio::time::{sleep, Duration, Instant};

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let start = Instant::now();

    sleep(Duration::from_secs(5)).await;

    let end = Instant::now();
    println!("{:?}", end - start);
    log::warn!("Running Time: {:?}", end - start);
}
