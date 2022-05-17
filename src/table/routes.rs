use warp::Filter;
use crate::{DBPool, with_db};
use crate::t_handler::{list_tables_handler};

pub fn tables_routes(db: DBPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    tables_list(db.clone())
}

fn tables_list(db: DBPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("tables" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(list_tables_handler)
}