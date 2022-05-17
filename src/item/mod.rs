use crate::db::get_db_con;
use crate::{error, error::Error::*};
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
    let rows = iter_item_string(body.tid, &body);
    let rows = con
        .query(rows.as_str(), &[])
        .await
        .map_err(DBQueryError)?;
    Ok(rows.iter().map(|r| row_to_response(&r)).collect())
}

pub async fn get_item(db_pool: &DBPool, id: i32) -> Result<Item> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = $1", SELECT_FIELDS_ITEMS, TABLE_ITEM);
    let row = con
        .query_one(query.as_str(), &[&id])
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

pub async fn update_item(db_pool: &DBPool, id: i32, body: ItemUpdateRequest) -> Result<Item> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "UPDATE {} SET name = $1, cook_time = $2 WHERE id = $3 RETURNING *",
        TABLE_ITEM
    );
    let row = con
        .query_one(query.as_str(), &[&body.name, &body.cook_time, &id])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_response(&row))
}

pub async fn delete_item(db_pool: &DBPool, id: i32) -> Result<String> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE_ITEM);
    let row = con.execute(query.as_str(), &[&id])
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
fn iter_item_string(tid: i32, item: &MultipleItemRequest) -> String {
    let items = &item.items;
    let mut query:String = format!("INSERT INTO {} (tid, name, cook_time) VALUES", TABLE_ITEM).to_string();
    let mut i = 1;
    for column in items{
        if i == 1 {
            query = format!("{} ({},'{}',{})",query,tid, column.name, column.cook_time);
        }else{
            query = format!("{}, ({},'{}',{})",query,tid, column.name, column.cook_time);

        }
        i=i+1;
    }
    query = format!("{} RETURNING *", query);
    return query;
}

