use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder };

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at: http://127.0.0.1:8000");
    println!("Available endpoints:");
    println!("   GET / - hello world");

    HttpServer::new(|| { App::new().service(hello) })
        .bind(("127.0.0.1", 8000))?
        .run().await
}
