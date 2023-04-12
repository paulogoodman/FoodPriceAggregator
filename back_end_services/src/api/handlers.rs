use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct AddResponse {
    sum: i32,
}


#[derive(Debug, Serialize, Deserialize)]
struct EchoResponse {
    message: String,
}


#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello")
}


#[get("/add")]
async fn add(query: web::Query<(i32, i32)>) -> impl Responder {
    let sum = query.0 .0 + query.0 .1;
    let response = AddResponse { sum };
    HttpResponse::Ok().json(response)
}

#[post("/echo")]
async fn echo(body: String) -> impl Responder {
    let response = EchoResponse { message: body };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(add)
            .service(echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}