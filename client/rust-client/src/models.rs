use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub author: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Local>>,
}
