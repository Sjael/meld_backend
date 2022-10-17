
use axum::{Extension, extract::Query, Json};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use sqlx::{FromRow, Row, Pool, Postgres, PgPool, Type, types::Json as sqlxJson};
use sqlx::postgres::{PgPoolOptions, PgRow};

use crate::Item;

pub async fn get_items(Extension(pool): Extension<PgPool>, ) -> Json<Value> {
    
    /* 

		sqlx::query(
		r#"
        CREATE TABLE IF NOT EXISTS patch_notes_class (
        key SERIAL PRIMARY KEY,
		version TEXT,
        class TEXT,
		old TEXT,
        new TEXT,
		attr TEXT,
		quote TEXT
        );"#,
	)
	.execute(&pool)
	.await;

	// 3) Insert a new ticket
	let row: (i32,) = sqlx::query_as("insert into game_versions (id, name) values ($1, $2) returning id")
		.bind(3201)
        .bind("Sabi's Revenge")
		.fetch_one(&pool)
		.await
		.expect("no result");
    Json(json!(""))

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
	let select_query = sqlx::query("SELECT id, name, image_path, info FROM item_table ORDER BY id DESC");
	let tickets: Vec<Item> = select_query
		.map(|row: PgRow| Item {
			id: row.get("id"),
			name: row.get("name"),
			image: row.get("image_path"),
			info: row.get("info")
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

