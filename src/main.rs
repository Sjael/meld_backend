#![allow(unused)] // silence unused warnings while exploring (to comment out)

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row, Pool, Postgres, PgPool, Type, types::Json as sqlxJson};
use axum::{
    extract::{Extension, Query, FromRequest, RequestParts},
    routing::get,
    Router,
    Json
};
use tower_http::cors::{Any, CorsLayer};
use std::{fmt, str::FromStr};
use std::net::SocketAddr;   
use serde_json::{Value, json};
use serde::{de, Serialize, Deserialize, Deserializer};

pub type Db = Pool<Postgres>;

const PG_HOST: &str = "containers-us-west-78.railway.app:7066";
const PG_ROOT_DB: &str = "railway";
const PG_ROOT_USER: &str = "postgres";
const PG_ROOT_PWD: &str = "sWetkbxfLgS7Sw0fnogs";


pub mod get_item;
pub mod get_items;
pub mod add_items;
pub mod get_patch;
pub mod get_patches;
use get_item::get_item;
use get_items::get_items;
use add_items::add_items;
use get_patch::get_patch;
use get_patches::get_patches;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Params {
    foo: Option<i32>,
    bar: Option<String>,
}

#[derive(Serialize)]
pub struct Rewrap{
	response: Item,
}

#[derive(Debug, FromRow, Serialize, Default)]
pub struct Item {
	id: i32,
	name: String,
	image: String,
	info: sqlxJson<ItemInfo>,
}


#[derive(Debug, FromRow, Serialize, Default)]
pub struct Patch {
	version: String,
	name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ItemInfo{
	cost: i32,
	phys_pen: i32,
	phys_power: i32,
}





async fn new_db_pool(host: &str, db: &str, user: &str, pwd: &str, max_con: u32) -> Result<Db, sqlx::Error> {
    
    let con_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
	PgPoolOptions::new()
        .max_connections(max_con)
        .connect(&con_string)
        .await
}

#[tokio::main]
async fn main()  {

    let pool = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1)
    .await
    .expect("can connect to database");

    let app = Router::new()
        .route("/", get(get_items))
		.route("/get_item", get(get_item))
		.route("/add_items", get(add_items))
		.route("/get_patches", get(get_patches))
		.route("/get_patch", get(get_patch))
        .layer(Extension(pool))
		.layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
        
	

}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

