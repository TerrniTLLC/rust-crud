// noodle.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct BuyNoodleRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub noodle_name: String,
    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateNoodleURL {
    pub uuid: String,
}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct Noodle {
    pub uuid: String,
    pub noodle_name: String,
    pub description: String,
}

impl Noodle {
    pub fn new(uuid: String, noodle_name: String, description: String) -> Noodle {
        Noodle {
            uuid,
            noodle_name,
            description,
        }
    }
}
