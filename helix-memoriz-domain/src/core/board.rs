use chrono::prelude::*;
use serde_json;
use uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub uuid: Option<uuid::Uuid>,
    pub title: String,
    pub data: Option<serde_json::Value>,
    pub color: Option<String>,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
    pub owner: Option<uuid::Uuid>,
}

impl Board {
    pub fn new(
        uuid: Option<uuid::Uuid>,
        title: String,
        data: Option<serde_json::Value>,
        color: Option<String>,
        created_on: Option<DateTime<Utc>>,
        updated_on: Option<DateTime<Utc>>,
        owner: Option<uuid::Uuid>,
    ) -> Board {
        Board {
            uuid: uuid,
            title: title,
            data: data,
            color: color,
            created_on: created_on,
            updated_on: updated_on,
            owner: owner,
        }
    }
}
