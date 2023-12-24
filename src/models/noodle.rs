// noodle.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct BuyNoodleRequest {
    #[validate(length(min = 1, message = "noodle name required"))]
    pub noodle_name: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateNoodleURL {
    pub uuid: String,
}
