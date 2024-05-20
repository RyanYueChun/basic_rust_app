use actix_web::{web, Responder, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

pub(crate) async fn greet(req: HttpRequest, name: web::Path<String>) -> impl Responder {
    println!("request received");
    println!("REQ: {req:?}");
    format!("Hello {name}!")
}

pub(crate) async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DtoRequest {
    name: String,
    number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DtoResponse {
    name: String,
    number: i32,
    good: bool,
}

/// This handler uses json extractor
pub(crate) async fn process(item: web::Json<DtoRequest>) -> HttpResponse {
    println!("model: {:?}", &item);
    let DtoRequest { name, number } = item.into_inner();
    HttpResponse::Ok().json(DtoResponse{
        name: name + " Lol",
        number: number + 1,
        good: number % 2 == 0,
    }) // <- send response
}