use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub struct Message {
    #[serde(alias = "type")]
    pub message_type: String,
    pub pin: Option<u16>,
    #[serde(alias = "pinType")]
    pub pin_type: Option<String>,
    pub value: Option<f32>,
}

#[derive(Serialize, Debug)]
pub struct Reply {
    pub status: String,
    pub message: String,
}
