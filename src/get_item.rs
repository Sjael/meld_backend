use axum::{Extension, extract::Query, Json};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use sqlx::{FromRow, Row, Pool, Postgres, PgPool, Type, types::Json as sqlxJson};
use sqlx::postgres::{PgPoolOptions, PgRow};
use crate::{Item, ItemInfo};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Params {
    foo: Option<i32>,
    bar: Option<String>,
}




pub async fn get_item(Extension(pool): Extension<PgPool>, Query(params): Query<Params>) -> Json<Value>{
	
	if let Some(item) = params.foo{
		let select_query = 
			sqlx::query("SELECT id, name, image_path, info FROM item_table WHERE id = $1")
			.bind(item);
		let tickets: Item = select_query
			.map(|row: PgRow| Item {
				id: row.get("id"),
				name: row.get("name"),
				image: row.get("image_path"), 
				info: row.get("info")
			})
			.fetch_one(&pool)
			.await
			.expect("no result");
		println!("\n=== select tickets with query.map...:\n{:?}", tickets);
		let subres = json!(tickets);
        Json(json!(tickets))
	}else{
        Json(json!(""))
    }
	
}