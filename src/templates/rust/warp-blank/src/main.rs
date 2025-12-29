use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path("hello").map(|| "Hello, world!");

    println!("Server running at: http://127.0.0.1:8000");
    println!("Available endpoints:");
    println!("   GET /hello - hello world");

    warp::serve(hello).run(([127, 0, 0, 1], 8000)).await;
}
