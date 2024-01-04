use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

extern crate dotenv;

use dotenv::dotenv;
use std::env;

use crate::models::noodle::Noodle;

#[derive(Clone, Debug)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

// TODO: add impl of utils with method below
/* #[derive(Clone, Debug)]
pub struct Utils;

impl Utils {
    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }
} */

impl Database {
    pub async fn init() -> Result<Self, Error> {
        dotenv().ok();

        pub fn string_to_static_str(s: String) -> &'static str {
            Box::leak(s.into_boxed_str())
        }

        let db_url = env::var("DB_URL").expect("Database URL must be set.");
        let db_username =
            string_to_static_str(env::var("DB_USERNAME").expect("Database username must set."));
        let db_password =
            string_to_static_str(env::var("DB_PASSWORD").expect("Database password must set"));

        let client = Surreal::new::<Ws>(db_url).await?;
        client
            .signin(Root {
                username: db_username,
                password: db_password,
            })
            .await?;

        client.use_ns("public").use_db("noodles").await.unwrap();
        Ok(Database {
            client,
            name_space: String::from("public"),
            db_name: String::from("noodles"),
        })
    }

    pub async fn get_all_noodles(&self) -> Option<Vec<Noodle>> {
        let result: Result<Vec<Noodle>, Error> = self.client.select("noodle").await;
        match result {
            Ok(all_noodles) => Some(all_noodles),
            Err(_) => None,
        }
    }

    pub async fn add_noodle(&self, new_noodle: Noodle) -> Option<Noodle> {
        let result = self
            .client
            .create(("noodle", new_noodle.uuid.clone()))
            .content(new_noodle)
            .await;
        match result {
            Ok(created) => created,
            Err(_) => None,
        }
    }
}
