mod table;
mod item;
mod db;
mod error;

use mobc::{Connection, Pool};
use mobc_postgres::{PgConnectionManager, tokio_postgres};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};
use table::handler as t_handler;
use item::handler as i_handler;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    let tables_routes = table::routes::tables_routes(db_pool.clone());
    let items_routes = item::routes::items_routes(db_pool.clone());
    let routes = tables_routes
        .or(items_routes)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
