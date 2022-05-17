use item::data::Item;
use crate::db::get_db_con;
use crate::error::Error::DBQueryError;
use crate::{DBPool, error, item};
use crate::tokio_postgres::{Row};
use chrono::prelude::*;
pub mod handler;
pub(crate) mod routes;

const TABLE_ITEM: &str = "items";
const SELECT_FIELDS_ITEMS: &str = "id, tid, name, created_at, cook_time";
type Result<T> = std::result::Result<T, error::Error>;

pub async fn get_items_by_table_id(db_pool: &DBPool, tid: i32) -> Result<Vec<Item>> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "SELECT {} FROM {} WHERE tid = $1 ORDER BY created_at DESC",
        SELECT_FIELDS_ITEMS, TABLE_ITEM
    );

    let rows = con
        .query(query.as_str(), &[&tid])
        .await
        .map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_response(&r)).collect())
}

fn row_to_response(row: &Row) -> Item {
    let id: i32 = row.get(0);
    let tid: i32 = row.get(1);
    let name: String = row.get(2);
    let created_at: DateTime<Utc> = row.get(3);
    let cook_time: i32 = row.get(4);
    Item {
        id,
        tid,
        name,
        created_at,
        cook_time,
    }
}
