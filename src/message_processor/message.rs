use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "type")]
    pub message_type: String,
    pub pin: Option<u16>,
    #[serde(rename = "pinType")]
    pub pin_type: Option<String>,
    pub value: Option<f32>,
}

#[derive(Serialize, Debug)]
pub struct StatusResult {
    pub pin_count: usize,
}

#[derive(Serialize, Debug)]
pub struct QueryResult {
    pub pin: u16,
    #[serde(rename = "pinType")]
    pub pin_type: String,
    pub value: f32,
}

#[derive(Serialize, Debug)]
pub struct MessageResult {
    pub status: String,
    pub message: String,
    #[serde(rename = "daemonStatus", skip_serializing_if = "Option::is_none")]
    pub daemon_status: Option<StatusResult>,
    #[serde(rename = "queryResult", skip_serializing_if = "Option::is_none")]
    pub query_result: Option<Vec<QueryResult>>,
}
