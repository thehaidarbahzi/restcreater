use axum::{ routing::get, Router };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(root));

    println!("Server running at: http://127.0.0.1:8000");
    println!("Available endpoints:");
    println!("   GET / - Hello, World!");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
