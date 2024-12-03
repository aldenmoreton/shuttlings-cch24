use std::net::Ipv6Addr;

use axum::extract::Query;
use itertools::Itertools;

#[derive(serde::Deserialize)]
pub struct P1Query {
    from: String,
    key: String,
}

pub async fn p1(Query(params): Query<P1Query>) -> String {
    params
        .from
        .split(".")
        .zip(params.key.split("."))
        .map(|(num1, num2)| {
            num1.parse::<u8>()
                .unwrap()
                .wrapping_add(num2.parse().unwrap())
        })
        .join(".")
}

#[derive(serde::Deserialize)]
pub struct P2Query {
    from: String,
    to: String,
}

pub async fn p2(Query(params): Query<P2Query>) -> String {
    params
        .from
        .split(".")
        .zip(params.to.split("."))
        .map(|(from, to)| {
            to.parse::<u8>()
                .unwrap()
                .wrapping_sub(from.parse().unwrap())
        })
        .join(".")
}

fn p3_converter(left: Ipv6Addr, right: Ipv6Addr) -> Ipv6Addr {
    let left = left.octets();
    let right = right.octets();

    let mut combined = [0; 16];
    for i in 0..16 {
        combined[i] = left[i] ^ right[i]
    }

    Ipv6Addr::from(combined)
}

pub async fn p3a(Query(params): Query<P1Query>) -> String {
    let from = params.from.parse::<Ipv6Addr>().unwrap();
    let key = params.key.parse::<Ipv6Addr>().unwrap();

    p3_converter(from, key).to_string()
}

pub async fn p3b(Query(params): Query<P2Query>) -> String {
    let from = params.from.parse::<Ipv6Addr>().unwrap();
    let to = params.to.parse::<Ipv6Addr>().unwrap();

    p3_converter(from, to).to_string()
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
            .oneshot(
                Request::builder()
                    .uri("/2/dest?from=10.0.0.0&key=1.2.3.255")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = collect_body(response).await;

        assert_eq!(body, "11.2.3.255");
    }

    #[tokio::test]
    async fn part2_1() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/2/dest?from=128.128.33.0&key=255.0.255.33")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = collect_body(response).await;
        assert_eq!(body, "127.128.32.33")
    }

    #[tokio::test]
    async fn part3_1() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/2/key?from=10.0.0.0&to=11.2.3.255")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = collect_body(response).await;
        assert_eq!(body, "1.2.3.255")
    }

    #[tokio::test]
    async fn part3_2() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/2/key?from=128.128.33.0&to=127.128.32.33")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = collect_body(response).await;
        assert_eq!(body, "255.0.255.33")
    }
}
