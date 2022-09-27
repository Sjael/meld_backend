#![allow(unused)] // silence unused warnings while exploring (to comment out)

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row, Pool, Postgres, PgPool};
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

const PG_HOST: &str = "containers-us-west-67.railway.app:7616";
const PG_ROOT_DB: &str = "railway";
const PG_ROOT_USER: &str = "postgres";
const PG_ROOT_PWD: &str = "hsyKFBEu7UDcuISu7XHo";


#[derive(Debug, FromRow, Serialize, Default)]
struct Item {
	id: i32,
	name: String,
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
        .layer(Extension(pool))
		.layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    foo: Option<i32>,
    bar: Option<String>,
}

#[derive(Serialize)]
struct Rewrap{
	response: Item,
}

async fn get_item(Extension(pool): Extension<PgPool>, Query(params): Query<Params>) -> Json<Value>{
	
	let mut response = Rewrap{
		response: Item::default()
	};
	if let Some(item) = params.foo{
		let select_query = 
			sqlx::query("SELECT id, name FROM item_table WHERE id = $1")
			.bind(item);
		let tickets: Item = select_query
			.map(|row: PgRow| Item {
				id: row.get("id"),
				name: row.get("name"),
			})
			.fetch_one(&pool)
			.await
			.expect("no result");
		println!("\n=== select tickets with query.map...:\n{:?}", tickets);
		let subres = json!(tickets);
		response.response = tickets;
	}
	
	Json(json!(response))
}

async fn add_items(Extension(pool): Extension<PgPool>, ) -> Json<Value> {
	println!("nah");
	// 2) Create table if not exist yet
	sqlx::query(
		r#"
        CREATE TABLE IF NOT EXISTS item_table (
        id INT,
        key SERIAL PRIMARY KEY,
        name TEXT,
        info JSONB
        );"#,
	)
	.execute(&pool)
	.await;

	// 3) Insert a new ticket
	let row: (i32,) = sqlx::query_as("insert into item_table (id, name) values ($1, $2) returning id")
		.bind(3201)
        .bind("Sabi's Revenge")
		.fetch_one(&pool)
		.await
		.expect("no result");
    Json(json!(""))
}

async fn get_items(Extension(pool): Extension<PgPool>, ) -> Json<Value> {
    
    /* 
    println!("nah");
	// 2) Create table if not exist yet
	sqlx::query(
		r#"
        CREATE TABLE IF NOT EXISTS item_table (
        id INT,
        key SERIAL PRIMARY KEY,
        name TEXT,
        info JSONB
        );"#,
	)
	.execute(&pool)
	.await?;

	// 3) Insert a new ticket
	let row: (i32,) = sqlx::query_as("insert into item_table (id, name) values ($1, $2) returning id")
		.bind(2506)
        .bind("baton's Revenge")
		.fetch_one(&pool)
		.await?;
    
	// 4) Select all tickets
	let rows = sqlx::query("SELECT * FROM item_table").fetch_all(&pool).await;
	let str_result = rows
		.iter()
		.map(|r| format!("{} - {}", r.get::<i32, _>("id"), r.get::<String, _>("name")))
		.collect::<Vec<String>>()
		.join(", ");
	println!("\n== select tickets with PgRows:\n{}", str_result);
    */
	// 5) Select query with map() (build the Ticket manually)
	let select_query = sqlx::query("SELECT id, name FROM item_table");
	let tickets: Vec<Item> = select_query
		.map(|row: PgRow| Item {
			id: row.get("id"),
			name: row.get("name"),
		})
		.fetch_all(&pool)
		.await
        .expect("no result");
	println!("\n=== select tickets with query.map...:\n{:?}", tickets);
        /* 
	// 6) Select query_as (using derive FromRow)
	let select_query = sqlx::query_as::<_, Item>("SELECT id, name FROM item_table");
	let tickets: Vec<Item> = select_query.fetch_all(&pool).await?;
	println!("\n=== select tickets with query.map...: \n{:?}", tickets);
    */
    Json(json!(tickets))
}


