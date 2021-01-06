use crate::core::label::Label;
use chrono::prelude::*;
use serde_json;
use uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id: i32,
    pub uuid: Option<uuid::Uuid>,
    pub title: String,
    pub content: Option<String>,
    pub data: Option<serde_json::Value>,
    pub color: Option<String>,
    pub archived: bool,
    pub created_on: Option<DateTime<Utc>>,
    pub updated_on: Option<DateTime<Utc>>,
    pub owner: Option<uuid::Uuid>,
    pub labels: Option<Vec<Label>>,
    pub board: Option<uuid::Uuid>,
}

impl Entry {
    pub fn new(
        id: i32,
        uuid: Option<uuid::Uuid>,
        title: String,
        content: Option<String>,
        data: Option<serde_json::Value>,
        color: Option<String>,
        archived: bool,
        created_on: Option<DateTime<Utc>>,
        updated_on: Option<DateTime<Utc>>,
        owner: Option<uuid::Uuid>,
        labels: Option<Vec<Label>>,
        board: Option<uuid::Uuid>,
    ) -> Entry {
        Entry {
            id: id,
            uuid: uuid,
            title: title,
            content: content,
            data: data,
            color: color,
            archived: archived,
            created_on: created_on,
            updated_on: updated_on,
            owner: owner,
            labels: labels,
            board: board,
        }
    }
}
