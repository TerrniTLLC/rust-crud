// db.rs
pub mod database;
pub use database::Database;
#[path = "db/traits/noodle_data_trait.rs"] pub mod noodle_data_trait;

