#![deny(clippy::unwrap_used)]
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{
    routing::{get, post},
    Extension, Router,
};

pub mod solutions {
    pub mod day01;
    pub mod day02;
    pub mod day05;
    pub mod day09;
}

use solutions::*;

pub fn router() -> Router {
    Router::new()
        .merge(
            Router::new()
                .route("/", get(day01::p1))
                .route("/-1/seek", get(day01::p2)),
        )
        .merge(
            Router::new()
                .route("/2/dest", get(day02::p1))
                .route("/2/key", get(day02::p2))
                .route("/2/v6/dest", get(day02::p3a))
                .route("/2/v6/key", get(day02::p3b)),
        )
        .route("/5/manifest", post(day05::manifest))
        .merge(
            Router::new()
                .route("/9/milk", post(day09::milk))
                .route("/9/refill", post(day09::refill))
                .layer(Extension(Arc::new(Mutex::new(
                    leaky_bucket::RateLimiter::builder()
                        .initial(5)
                        .max(5)
                        .interval(Duration::from_millis(1_000))
                        .build(),
                )))),
        )
}

#[cfg(test)]
pub mod test_utils {
    use axum::{body::Body, response::Response};
    use http_body_util::BodyExt as _;

    pub async fn collect_body(response: Response<Body>) -> String {
        let body = response
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec();

        String::from_utf8(body).unwrap()
    }
}
