#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    println!("Server running at: http://127.0.0.1:8000");
    println!("Available endpoints:");
    println!("   GET / - hello world");

    rocket::build().mount("/", routes![index])
}
