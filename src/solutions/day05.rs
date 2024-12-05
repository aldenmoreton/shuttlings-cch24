use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use cargo_manifest::Manifest;
use itertools::Itertools;

#[derive(Debug, serde::Deserialize)]
struct Orders {
    orders: Vec<toml::Value>,
}

#[derive(Debug, serde::Deserialize)]
struct Order {
    item: String,
    quantity: u32,
}

pub async fn p1(headers: HeaderMap, body: String) -> impl IntoResponse {
    let Some(package) = match headers
        .get("Content-Type")
        .and_then(|header| header.to_str().ok())
    {
        Some("application/toml") => toml::from_str::<Manifest>(&body).ok(),
        Some("application/yaml") => serde_yaml::from_str::<Manifest>(&body).ok(),
        Some("application/json") => serde_json::from_str::<Manifest>(&body).ok(),
        _ => return StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response(),
    }
    .and_then(|man| man.package) else {
        return (StatusCode::BAD_REQUEST, "Invalid manifest").into_response();
    };

    let contains_magic = package
        .keywords
        .and_then(|keys| {
            keys.as_local()
                .map(|keys| keys.iter().any(|key| key == "Christmas 2024"))
        })
        .unwrap_or(false);
    if !contains_magic {
        return (StatusCode::BAD_REQUEST, "Magic keyword not provided").into_response();
    }

    let orders = match package
        .metadata
        .and_then(|meta| meta.try_into::<Orders>().ok())
    {
        Some(Orders { orders }) => orders
            .into_iter()
            .filter_map(|order| order.try_into::<Order>().ok())
            .collect::<Vec<_>>(),
        _ => return StatusCode::NO_CONTENT.into_response(),
    };

    if orders.is_empty() {
        return StatusCode::NO_CONTENT.into_response();
    }

    Itertools::intersperse(
        orders
            .into_iter()
            .map(|order| format!("{}: {}", order.item, order.quantity)),
        "\n".into(),
    )
    .collect::<String>()
    .into_response()
}

#[cfg(test)]
mod tests {
    use crate::{router, test_utils::collect_body};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt as _;

    #[tokio::test]
    async fn part1_1() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/5/manifest")
                    .method("POST")
                    .header("Content-Type", "application/toml")
                    .body(
                        r#"
[package]
name = "not-a-gift-order"
authors = ["Not Santa"]
keywords = ["Christmas 2024"]

[[package.metadata.orders]]
item = "Toy car"
quantity = 2

[[package.metadata.orders]]
item = "Lego brick"
quantity = 230
"#
                        .to_string(),
                    )
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = collect_body(response).await;

        assert_eq!(
            body,
            "Toy car: 2
Lego brick: 230"
        );
    }

    #[tokio::test]
    async fn part1_2() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/5/manifest")
                    .method("POST")
                    .header("Content-Type", "application/toml")
                    .body(
                        r#"
[package]
name = "coal-in-a-bowl"
authors = ["H4CK3R_13E7"]
keywords = ["Christmas 2024"]

[[package.metadata.orders]]
item = "Coal"
quantity = "Hahaha get rekt"
"#
                        .to_string(),
                    )
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn part2_1() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/5/manifest")
                    .method("POST")
                    .header("Content-Type", "application/toml")
                    .body(
                        r#"
[package]
name = false
authors = ["Not Santa"]
keywords = ["Christmas 2024"]
"#
                        .to_string(),
                    )
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = collect_body(response).await;

        assert_eq!(body, "Invalid manifest");
    }

    #[tokio::test]
    async fn part2_2() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/5/manifest")
                    .method("POST")
                    .header("Content-Type", "application/toml")
                    .body(
                        r#"
[package]
name = "not-a-gift-order"
authors = ["Not Santa"]
keywords = ["Christmas 2024"]

[profile.release]
incremental = "stonks"
"#
                        .to_string(),
                    )
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = collect_body(response).await;

        assert_eq!(body, "Invalid manifest");
    }

    #[tokio::test]
    async fn part3_1() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/5/manifest")
                    .method("POST")
                    .header("Content-Type", "application/toml")
                    .body(
                        r#"
[package]
name = "grass"
authors = ["A vegan cow"]
keywords = ["Moooooo"]
"#
                        .to_string(),
                    )
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = collect_body(response).await;

        assert_eq!(body, "Magic keyword not provided");
    }

    #[tokio::test]
    async fn part4_1() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/5/manifest")
                    .method("POST")
                    .header("Content-Type", "text/html")
                    .body(r#"<h1>Hello, bird!</h1>"#.to_string())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn part4_2() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/5/manifest")
                    .method("POST")
                    .header("Content-Type", "application/yaml")
                    .body(
                        r#"
package:
  name: big-chungus-sleigh
  version: "2.0.24"
  metadata:
    orders:
      - item: "Toy train"
        quantity: 5
  rust-version: "1.69"
  keywords:
    - "Christmas 2024"
"#
                        .to_string(),
                    )
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = collect_body(response).await;

        assert_eq!(body, "Toy train: 5");
    }
}
