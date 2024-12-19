use std::{sync::Arc, time::Duration};

use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension,
};
use leaky_bucket::RateLimiter;
use serde_json::json;
use tokio::sync::RwLock;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MilkMeasure {
    Gallons(f32),
    Liters(f32),
    Litres(f32),
    Pints(f32),
}

pub async fn milk(
    Extension(milk_bucket): Extension<Arc<RwLock<RateLimiter>>>,
    headers: HeaderMap,
    body: Option<String>,
) -> impl IntoResponse {
    let bucket = milk_bucket.read().await;
    let aquired = bucket.try_acquire(1);
    drop(bucket);

    if let Some("application/json") = headers
        .get("content-type")
        .and_then(|content| content.to_str().ok())
    {
        let Ok(measurement) = serde_json::from_str::<MilkMeasure>(&body.unwrap()) else {
            return StatusCode::BAD_REQUEST.into_response();
        };

        if !aquired {
            return (StatusCode::TOO_MANY_REQUESTS, "No milk available\n").into_response();
        }

        return match measurement {
            MilkMeasure::Gallons(g) => json!({"liters": g * 3.78541}),
            MilkMeasure::Liters(l) => json!({"gallons": l * 0.26417}),
            MilkMeasure::Litres(l) => json!({"pints": l * 1.759754}),
            MilkMeasure::Pints(p) => json!({"litres": p * 0.568261291}),
        }
        .to_string()
        .into_response();
    }

    if aquired {
        (StatusCode::OK, "Milk withdrawn\n")
    } else {
        (StatusCode::TOO_MANY_REQUESTS, "No milk available\n")
    }
    .into_response()
}

pub async fn refill(Extension(milk_bucket): Extension<Arc<RwLock<RateLimiter>>>) {
    let mut bucket = milk_bucket.write().await;
    *bucket = RateLimiter::builder()
        .initial(5)
        .max(5)
        .interval(Duration::from_millis(1_000))
        .build();
}
