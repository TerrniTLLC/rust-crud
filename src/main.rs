use actix_web::{get, patch, post, web::Json, web::Path, App, HttpResponse, HttpServer, Responder};
mod models;
use crate::models::noodle::{BuyNoodleRequest, UpdateNoodleURL};
use validator::{Validate, ValidationErrors};

#[get("/noodles")]

async fn get_noodles() -> impl Responder {
    HttpResponse::Ok().body("Noodles avaliable !!")
}

#[post("/buy_noodle")]
async fn buy_noodle(body: Json<BuyNoodleRequest>) -> impl Responder {
    let is_valid: Result<(), ValidationErrors> = body.validate();
    match is_valid {
        Ok(_) => {
            let noodle_name: String = body.noodle_name.clone();
            HttpResponse::Ok().body(format!("noodle name entered is {noodle_name}"))
        }
        Err(_) => HttpResponse::Ok().body("Noodle name required !"),
    }
}

#[patch("/update_noodle/{uuid}")]
async fn update_noodle(update_noodle_url: Path<UpdateNoodleURL>) -> impl Responder {
    let uuid: String = update_noodle_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Update then noodle with this uuid {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_noodles)
            .service(buy_noodle)
            .service(update_noodle)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
