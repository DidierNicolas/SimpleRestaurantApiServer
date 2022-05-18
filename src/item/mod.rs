use crate::db::get_db_con;
use crate::{DBCon, error, error::Error::*};
use crate::{DBPool};
use crate::item::data::{Item, ItemUpdateRequest, MultipleItemRequest};
use crate::tokio_postgres::{Row};
use chrono::prelude::*;

pub mod data;
pub mod handler;
pub(crate) mod routes;

const TABLE_ITEM: &str = "items";
const SELECT_FIELDS_ITEMS: &str = "id, tid, name, created_at, cook_time";
type Result<T> = std::result::Result<T, error::Error>;

pub async fn create_multiple_item(db_pool: &DBPool, body: MultipleItemRequest) -> Result<Vec<Item>> {
    let con = get_db_con(db_pool).await?;
    let rows = iter_item_string(body.tid, &body, con);
    Ok(rows.await?.iter().map(|r| row_to_response(r)).collect())
}

pub async fn get_item(db_pool: &DBPool, tid: i32, id: i32) -> Result<Item> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = $1 and tid = $2", SELECT_FIELDS_ITEMS, TABLE_ITEM);
    let row = con
        .query_one(query.as_str(), &[&id, &tid])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_response(&row))
}

pub async fn fetch_items(db_pool: &DBPool, search: Option<String>) -> Result<Vec<Item>> {
    let con = get_db_con(db_pool).await?;
    let where_clause = match search {
        Some(_) => "WHERE name like $1",
        None => "",
    };
    let query = format!(
        "SELECT {} FROM {} {} ORDER BY created_at DESC",
        SELECT_FIELDS_ITEMS, TABLE_ITEM, where_clause
    );
    let q = match search {
        Some(v) => con.query(query.as_str(), &[&v]).await,
        None => con.query(query.as_str(), &[]).await,
    };
    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_response(&r)).collect())
}

pub async fn update_item(db_pool: &DBPool, tid: i32, id: i32, body: ItemUpdateRequest) -> Result<Item> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "UPDATE {} SET name = $1, cook_time = $2 WHERE id = $3  and tid = $4 RETURNING *",
        TABLE_ITEM
    );
    let row = con
        .query_one(query.as_str(), &[&body.name, &body.cook_time, &id, &tid])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_response(&row))
}

pub async fn delete_item(db_pool: &DBPool, tid: i32, id: i32) -> Result<String> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1 and tid = $2", TABLE_ITEM);
    let row = con.execute(query.as_str(), &[&id, &tid])
        .await
        .map_err(DBQueryError)?;
    let result: String = row.to_string();
    Ok(format!("{}",result))
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
async fn iter_item_string(tid: i32, item: &MultipleItemRequest, con: DBCon) -> Result<Vec<Row>> {
    let items = &item.items;
    let mut i = 1;
    let mut rows: Vec<Row> = Vec::new();
    for column in items{
        let query:String = format!("INSERT INTO {0} (id, tid, name, cook_time) VALUES ((SELECT coalesce(max(id)+1,1) FROM {0} WHERE tid = {1}),$1,$2,$3) RETURNING *",TABLE_ITEM, tid).to_string();
        let row = con
            .query_one(query.as_str(), &[&tid, &column.name, &column.cook_time])
            .await
            .map_err(DBQueryError)?;
        rows.push(row);
        i=i+1;
    }

    Ok(rows)
}

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
