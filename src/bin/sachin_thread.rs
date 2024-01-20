use tokio::runtime;

fn main() {
    let rt = runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()
        .unwrap();
    rt.spawn(async move {
        loop {
            println!("hell");
        }
    });
}
