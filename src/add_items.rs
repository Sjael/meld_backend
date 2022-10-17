
use axum::{Extension, extract::Query, Json};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use sqlx::{FromRow, Row, Pool, Postgres, PgPool, Type, types::Json as sqlxJson};
use sqlx::postgres::{PgPoolOptions, PgRow};

use crate::Params;


pub async fn add_items(Extension(pool): Extension<PgPool>, Query(params): Query<Params>) -> Json<Value> {
	// 2) Create table if not exist yet
	let test_jsonb = r#"{
        "phys_power" : 70,
        "cost" : 2800,
        "phys_pen" : 10
    }"#;
	let v: Value = serde_json::from_str::<Value>(test_jsonb).unwrap();

	// 3) Insert a new ticket
	sqlx::query("update item_table set info = $1 where id='3101'")
		.bind(v)
		.execute(&pool)
		.await;
    Json(json!(""))
}