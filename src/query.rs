use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Response {
    Good { val: Option<String> },
    Bad { message: String },
}

pub type Result<A> = std::result::Result<A, Box<dyn std::error::Error>>;
