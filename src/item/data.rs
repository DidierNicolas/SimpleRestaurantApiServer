use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: i32,
    pub tid: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub cook_time: i32,
}

#[derive(Deserialize)]
pub struct ItemRequest {
    pub name: String,
    pub cook_time: i32,
}

#[derive(Deserialize)]
pub struct MultipleItemRequest {
    pub tid: i32,
    pub items: Vec<ItemRequest>
}

#[derive(Deserialize)]
pub struct ItemUpdateRequest {
    pub name: String,
    pub cook_time: i32,
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub id: i32,
    pub tid: i32,
    pub name: String,
    pub cook_time: i32,
}

impl ItemResponse {
    pub fn of(item: Item) -> ItemResponse {
        ItemResponse {
            id: item.id,
            tid: item.tid,
            name: item.name,
            cook_time: item.cook_time,
        }
    }
}