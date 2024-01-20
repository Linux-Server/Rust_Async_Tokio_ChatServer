use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    println!("welcome to pointers");
    let x = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Unbale to listen");
    println!("listening : {:?}", x);

    loop {
        let (y, z) = x.accept().await.unwrap();
        println!("Fully : {:?}", (y, z));
    }
}
