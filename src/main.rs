use actix_web::web::Data;
use actix_web::{get, patch, post, web::Json, web::Path, App, HttpServer};
use errors::NoodleError;
use uuid;
use validator::{Validate, ValidationErrors};

use crate::db::{noodle_data_trait::NoodleDataTrait, Database};
use crate::models::noodle::{BuyNoodleRequest, Noodle, UpdateNoodleURL};

mod db;
mod errors;
mod models;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[get("/noodles")]
async fn get_noodles(db: Data<Database>) -> Result<Json<Vec<Noodle>>, NoodleError> {
    let result = Database::get_all_noodles(&db).await;
    match result {
        Some(found_noodles) => Ok(Json(found_noodles)),
        None => Err(NoodleError::NoNoodlesFound),
    }
}

#[post("/buy_noodle")]
async fn buy_noodle(body: Json<BuyNoodleRequest>, db: Data<Database>) -> Result<Json<Noodle>, NoodleError> {
    let is_valid: Result<(), ValidationErrors> = body.validate();
    match is_valid {
        Ok(_) => {
            let noodle_name: String = body.noodle_name.clone();
            let description: String = body.description.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let result = 
				Database::add_noodle(&db, Noodle::new(
                    String::from(new_uuid),
                    noodle_name,
                    description,
                ))
                .await;

            match result {
                Some(created) => {
					Ok(Json(created))
				}
				None => Err(NoodleError::NoodleCreationError),
            }
        }
        Err(_) => Err(NoodleError::NoodleCreationError),
    }
}

#[patch("/update_noodle/{uuid}")]
async fn update_noodle(update_noodle_url: Path<UpdateNoodleURL>, db: Data<Database>) -> Result<Json<Noodle>, NoodleError> {
    let uuid: String = update_noodle_url.into_inner().uuid;
	let update_result = Database::update_noodle(&db, uuid).await;
	match update_result {
		Some(updated_noodle) => Ok(Json(updated_noodle)),
		None => Err(NoodleError::NoSuchNoodleError),
	}
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
