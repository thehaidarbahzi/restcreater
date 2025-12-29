use axum::{ routing::get, Router };

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    println!("Server running at: http://127.0.0.1:8000");
    println!("Available endpoints:");
    println!("   GET / - hello world");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await;
}

async fn root() -> &'static str {
    "Hello, World!"
}
