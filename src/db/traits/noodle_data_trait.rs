use crate::models::Noodle;
use crate::{db::Database};
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait NoodleDataTrait {
    async fn get_all_noodles(db: &Data<Database>) -> Option<Vec<Noodle>>;
    async fn add_noodle(db: &Data<Database>, new_noodle: Noodle) -> Option<Noodle>;
    async fn update_noodle(db: &Data<Database>, uuid: String) -> Option<Noodle>;
}

#[async_trait] 
impl NoodleDataTrait for Database {
 	async fn get_all_noodles(db: &Data<Database>) -> Option<Vec<Noodle>> {
        let result: Result<Vec<Noodle>, Error> = db.client.select("noodle").await;
        match result {
            Ok(all_noodles) => Some(all_noodles),
            Err(_) => None,
        }
    }

    async fn add_noodle(db: &Data<Database>, new_noodle: Noodle) -> Option<Noodle> {
        let result = db 
            .client
            .create(("noodle", new_noodle.uuid.clone()))
            .content(new_noodle)
            .await;
        match result {
            Ok(created) => created,
            Err(_) => None,
        }
    }

					
	async fn update_noodle(db: &Data<Database>, uuid: String) -> Option<Noodle> {
        let find_noodle: Result<Option<Noodle>, Error> = db.client.select(("noodle", &uuid)).await;

        match find_noodle {
            Ok(found) => {
                match found {
                    Some(_found_noodle) => {
                        // and if found the noodle 
                        let updated_noodle: Result<Option<Noodle>, Error> = db 
                            .client
                            .update(("noodle", &uuid))
                            .merge(Noodle {
                                uuid,
                                noodle_name: String::from("sold"),
								description: String::from("sold")
                            })
                            .await;
                        match updated_noodle {
                            Ok(updated) => updated,
                            Err(_) => None,
                        }
                    }
                    None => None,
                }
            }
            Err(_) => None,
        }
    }
}

