use actix_web::web::Data;
use actix_web::{get, patch, post, web::Json, web::Path, App, HttpResponse, HttpServer, Responder};
mod db;
use crate::db::Database;
mod models;
use crate::models::noodle::{BuyNoodleRequest, UpdateNoodleURL};
use validator::{Validate, ValidationErrors};

extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[get("/noodles")]

async fn get_noodles(db: Data<Database>) -> impl Responder {
    // HttpResponse::Ok().body("Noodles avaliable !!")
    let result = db.get_all_noodles().await;
    match result {
        Some(found_noodles) => HttpResponse::Ok().body(format!("{:?}", found_noodles)),
        None => HttpResponse::Ok().body("Error !"),
    }
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
    dotenv().ok();

    let db = Database::init()
        .await
        .expect("Error connection to database");

    let db_data: Data<Database> = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_noodles)
            .service(buy_noodle)
            .service(update_noodle)
    })
    .bind(env::var("BASE_URL").expect("Base URL must be set."))?
    .run()
    .await
}
