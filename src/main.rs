use actix_web::web::Data;
use actix_web::{get, patch, post, web::Json, web::Path, App, HttpResponse, HttpServer, Responder};
use errors::NoodleError;
use uuid;
mod db;
use crate::db::Database;
mod errors;
mod models;
use crate::models::noodle::{BuyNoodleRequest, Noodle, UpdateNoodleURL};
use validator::{Validate, ValidationErrors};

extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[get("/noodles")]
async fn get_noodles(db: Data<Database>) -> Result<Json<Vec<Noodle>>, NoodleError> {
    let result = db.get_all_noodles().await;
    match result {
        Some(found_noodles) => Ok(Json(found_noodles)),
        None => Err(NoodleError::NoNoodlesFound),
    }
}

#[post("/buy_noodle")]
async fn buy_noodle(body: Json<BuyNoodleRequest>, db: Data<Database>) -> impl Responder {
    let is_valid: Result<(), ValidationErrors> = body.validate();
    match is_valid {
        Ok(_) => {
            let noodle_name: String = body.noodle_name.clone();
            let description: String = body.description.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let result = db
                .add_noodle(Noodle::new(
                    String::from(new_uuid),
                    noodle_name,
                    description,
                ))
                .await;

            match result {
                Some(created) => HttpResponse::Ok().body(format!("{:?} is ordered !", created)),
                None => HttpResponse::Ok().body("Error !"),
            }
        }
        Err(_) => HttpResponse::Ok().body("Noodle data is required"),
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
