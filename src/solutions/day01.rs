use axum::{http::StatusCode, response::IntoResponse};

pub async fn p1() -> &'static str {
    "Hello, bird!"
}

pub async fn p2() -> impl IntoResponse {
    (
        StatusCode::FOUND,
        [("location", "https://www.youtube.com/watch?v=9Gc4QTqslN4")],
    )
}

#[cfg(test)]
mod tests {
    use crate::{router, test_utils::collect_body};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt as _;

    #[tokio::test]
    async fn part1_1() {
        let response = router()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = collect_body(response).await;

        assert_eq!(body, "Hello, bird!");
    }

    #[tokio::test]
    async fn part2_1() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/-1/seek")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FOUND);

        assert_eq!(
            response
                .headers()
                .get("location")
                .map(|l| l.to_str().unwrap()),
            Some("https://www.youtube.com/watch?v=9Gc4QTqslN4")
        );

        let body = collect_body(response).await;
        assert!(body.is_empty())
    }
}
