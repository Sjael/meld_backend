
use axum::{Extension, extract::Query, Json};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use sqlx::{FromRow, Row, Pool, Postgres, PgPool, Type, types::Json as sqlxJson};
use sqlx::postgres::{PgPoolOptions, PgRow};

use crate::{Params, Patch};


pub async fn get_patch(Extension(pool): Extension<PgPool>, Query(params): Query<Params>) -> Json<Value> {
	
	if let Some(version) = params.bar{
		let select_query = sqlx::query("SELECT version, name FROM game_versions WHERE version=$1")
		.bind(version);
		let tickets: Vec<Patch> = select_query
			.map(|row: PgRow| Patch {
				version: row.get("version"),
				name: row.get("name"),
			})
			.fetch_all(&pool)
			.await
			.expect("no result");
		println!("\n=== select tickets with query.map...:\n{:?}", tickets);
		Json(json!(tickets))
	} else{
		Json(json!(""))
	}
}