use crate::{table::*, DBPool, Result};
use warp::{reject, reply::json, Reply};
use crate::item::data::ItemResponse;

pub async fn list_tables_handler(tid: i32, db_pool: DBPool) -> Result<impl Reply> {
    let items = get_items_by_table_id(&db_pool, tid)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &items.into_iter().map(|t| ItemResponse::of(t)).collect(),
    ))
}