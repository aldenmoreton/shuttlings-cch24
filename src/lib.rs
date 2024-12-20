#![deny(clippy::unwrap_used)]
use std::{sync::Arc, time::Duration};

use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};

pub mod solutions {
    pub mod day01;
    pub mod day02;
    pub mod day05;
    pub mod day09;
    pub mod day12;
    pub mod day16;
    pub mod day19;
}

use solutions::*;

pub fn router() -> Router {
    Router::new()
        .merge(
            Router::new()
                .route("/", get(day01::p1))
                .route("/-1/seek", get(day01::p2)),
        )
        .nest(
            "/2",
            Router::new()
                .route("/dest", get(day02::p1))
                .route("/key", get(day02::p2))
                .route("/v6/dest", get(day02::p3a))
                .route("/v6/key", get(day02::p3b)),
        )
        .route("/5/manifest", post(day05::manifest))
        .nest(
            "/9",
            Router::new()
                .route("/milk", post(day09::milk))
                .route("/refill", post(day09::refill))
                .layer(Extension(Arc::new(tokio::sync::RwLock::new(
                    leaky_bucket::RateLimiter::builder()
                        .initial(5)
                        .max(5)
                        .interval(Duration::from_millis(1_000))
                        .build(),
                )))),
        )
        .nest(
            "/12",
            Router::new()
                .route("/board", get(day12::board))
                .route("/reset", post(day12::reset))
                .route("/place/:team/:column", post(day12::place))
                .route("/random-board", get(day12::random_board))
                .layer(Extension(Arc::new(tokio::sync::RwLock::new(
                    day12::Board::default(),
                ))))
                .layer(Extension(Arc::new(tokio::sync::RwLock::new(
                    <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(2024),
                )))),
        )
        .nest(
            "/16",
            Router::new()
                .route("/wrap", post(day16::wrap))
                .route("/unwrap", get(day16::unwrap))
                .route("/decode", post(day16::decode)),
        )
        .nest(
            "/19",
            Router::new()
                .route("/reset", post(day19::reset))
                .route("/cite/:id", get(day19::cite))
                .route("/remove/:id", delete(day19::remove))
                .route("/undo/:id", put(day19::undo))
                .route("/draft", post(day19::draft))
                .route("/list", get(day19::list)),
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
