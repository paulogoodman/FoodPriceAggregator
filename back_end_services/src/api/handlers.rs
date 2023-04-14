use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ExampleData {
    id: u32,
    name: String,
}


#[get("/example/{id}")]
async fn get_example_data(web::Path(id): web::Path<u32>) -> impl Responder {
    let example_data = ExampleData {
        id,
        name: format!("Example name for ID {}", id),
    };
    HttpResponse::Ok().json(example_data)
}

#[derive(Serialize, Deserialize)]
struct CreateExampleData {
    name: String,
}


#[post("/example")]
async fn create_example_data(new_data: web::Json<CreateExampleData>) -> impl Responder {
    let example_data = ExampleData {
        id: rand::random(),
        name: new_data.name.clone(),
    };
    HttpResponse::Created().json(example_data)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_example_data).service(create_example_data);
}