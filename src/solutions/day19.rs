use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sqlx::{
    types::{
        chrono::{self},
        uuid,
    },
    PgPool,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Quote {
    id: uuid::Uuid,
    author: String,
    quote: String,
    created_at: chrono::DateTime<chrono::Utc>,
    version: i32,
}

pub async fn reset(Extension(pool): Extension<PgPool>) {
    sqlx::query!("TRUNCATE TABLE quotes")
        .execute(&pool)
        .await
        .unwrap();
}

pub async fn cite(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let res = sqlx::query_as!(Quote, "SELECT * FROM quotes WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if let Some(quote) = res {
        Json(quote).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn remove(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let res = sqlx::query_as!(Quote, "DELETE FROM quotes WHERE id = $1 RETURNING *", id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if let Some(quote) = res {
        Json(quote).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

#[derive(serde::Deserialize)]
pub struct NewQuote {
    author: String,
    quote: String,
}

pub async fn undo(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<uuid::Uuid>,
    Json(NewQuote { author, quote }): Json<NewQuote>,
) -> impl IntoResponse {
    let res = sqlx::query_as!(
        Quote,
        "UPDATE quotes
		SET
			author = $1,
		 	quote = $2,
			version = version + 1
		WHERE id = $3
		RETURNING *",
        author,
        quote,
        id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if let Some(quote) = res {
        Json(quote).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn draft(
    Extension(pool): Extension<PgPool>,
    Json(NewQuote { author, quote }): Json<NewQuote>,
) -> (StatusCode, Json<Quote>) {
    let new_quote = sqlx::query_as!(
        Quote,
        "INSERT INTO quotes (id, author, quote)
		VALUES (gen_random_uuid(), $1, $2)
		RETURNING *",
        author,
        quote
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(new_quote))
}

#[derive(serde::Deserialize)]
pub struct Pagination {
    token: Option<i64>,
}

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Query(Pagination { token }): Query<Pagination>,
) -> impl IntoResponse {
    let page_num = token.unwrap_or(1);
    let page_offset = (page_num - 1) * 3;

    let quotes = sqlx::query_as!(
        Quote,
        "SELECT *
		FROM quotes
		ORDER BY created_at
		LIMIT 4
		OFFSET $1",
        page_offset
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let token = if quotes.len() > 3 {
        Some(format!("{:0>16}", page_num + 1))
    } else {
        None
    };

    Json(serde_json::json!({
        "quotes": &quotes[0..3.min(quotes.len())],
        "page": page_num,
        "next_token": token
    }))
}
