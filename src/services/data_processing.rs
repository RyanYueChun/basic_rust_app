use actix_web::{web, Responder, HttpRequest};

//#[get("/hello/{name}")]
pub(crate) async fn greet(name: web::Path<String>) -> impl Responder {
    println!("request received");
    format!("Hello {name}!")
}

pub(crate) async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}