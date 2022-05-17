use crate::{item::*, item::data::*, DBPool, Result};
use warp::{reject, reply::json, Reply};
use serde_derive::{Deserialize};

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
}

pub async fn create_item_handler(body: MultipleItemRequest, db_pool: DBPool) -> Result<impl Reply> {
    let items = create_multiple_item(&db_pool, body)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &items.into_iter().map(|t| ItemResponse::of(t)).collect(),
    ))
}

pub async fn get_item_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    let item = get_item(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json(&ItemResponse::of(item)))
}

pub async fn list_items_handler(query: SearchQuery, db_pool: DBPool) -> Result<impl Reply> {
    let items = fetch_items(&db_pool, query.search)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &items.into_iter().map(|t| ItemResponse::of(t)).collect(),
    ))
}

pub async fn update_item_handler(
    id: i32,
    body: ItemUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&ItemResponse::of(
        update_item(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_item_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    Ok(delete_item(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?
    )

}