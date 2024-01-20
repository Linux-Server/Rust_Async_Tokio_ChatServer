use simple_logger::SimpleLogger;
use std::thread::sleep as sync_sleep;
use tokio::time::{sleep, Duration, Instant};

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let start = Instant::now();

    // asynchronous call
    tokio::join!(async_read_one(), async_read_two());

    use simple_logger::SimpleLogger;
    use std::thread::sleep as sync_sleep;
    use tokio::time::{sleep, Duration, Instant};

    #[tokio::main]
    async fn main() {
        SimpleLogger::new().init().unwrap();

        let start = Instant::now();

        // asynchronous call
        tokio::join!(async_read_one(), async_read_two());

        let end = Instant::now();
        log::info!("Running Time: {:#?}", end - start);
    }

    async fn async_read_one() {
        log::info!("Read One started");
        sleep(Duration::from_secs(1)).await;
        log::info!("Read One completed")
    }

    async fn async_read_two() {
        log::info!("Read two started");
        sleep(Duration::from_secs(2)).await;
        log::info!("Read two completed");
    }

    let end = Instant::now();
    log::info!("Running Time: {:#?}", end - start);
}

async fn async_read_one() {
    log::info!("Read One started");
    sleep(Duration::from_secs(1)).await;
    log::info!("Read One completed")
}

async fn async_read_two() {
    log::info!("Read two started");
    sleep(Duration::from_secs(2)).await;
    log::info!("Read two completed");
}
