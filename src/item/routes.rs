use warp::Filter;
use crate::{DBPool, with_db};
use crate::i_handler::{create_item_handler, delete_item_handler, get_item_handler, list_items_handler, list_tables_handler, update_item_handler};

pub fn items_routes(db: DBPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_item(db.clone())
        .or(update_item(db.clone()))
        .or(create_item(db.clone()))
        .or(delete_item(db.clone()))
        .or(items_list(db.clone()))
        .or(tables_list(db.clone()))
}

fn items_list(db: DBPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("items")
        .and(warp::query())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db(db))
        .and_then(list_items_handler)
}

fn create_item(db: DBPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("items")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(create_item_handler)
}

fn get_item(db: DBPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tables" / i32 / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_item_handler)
}

fn delete_item(db: DBPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tables" / i32 / i32)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(delete_item_handler)
}

fn update_item(db: DBPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tables" / i32 / i32)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(update_item_handler)
}

fn tables_list(db: DBPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tables" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(list_tables_handler)
}