use std::net::{Ipv4Addr, Ipv6Addr};

use axum::extract::Query;
use itertools::Itertools;

#[derive(serde::Deserialize)]
pub struct P1Query {
    from: Ipv4Addr,
    key: Ipv4Addr,
}

pub async fn p1(Query(P1Query { from, key }): Query<P1Query>) -> String {
    from.octets()
        .into_iter()
        .zip(key.octets().into_iter())
        .map(|(from, key)| from.wrapping_add(key).to_string())
        .join(".")
}

#[derive(serde::Deserialize)]
pub struct P2Query {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

pub async fn p2(Query(P2Query { from, to }): Query<P2Query>) -> String {
    from.octets()
        .into_iter()
        .zip(to.octets().into_iter())
        .map(|(from, to)| to.wrapping_sub(from).to_string())
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

#[derive(serde::Deserialize)]
pub struct P3AQuery {
    from: Ipv6Addr,
    key: Ipv6Addr,
}

pub async fn p3a(Query(P3AQuery { from, key }): Query<P3AQuery>) -> String {
    p3_converter(from, key).to_string()
}

#[derive(serde::Deserialize)]
pub struct P3BQuery {
    from: Ipv6Addr,
    to: Ipv6Addr,
}

pub async fn p3b(Query(P3BQuery { from, to }): Query<P3BQuery>) -> String {
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
    async fn part1_2() {
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
    async fn part2_1() {
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
    async fn part2_2() {
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

    #[tokio::test]
    async fn part3_1() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/2/v6/dest?from=fe80::1&key=5:6:7::3333")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = collect_body(response).await;
        assert_eq!(body, "fe85:6:7::3332")
    }

    #[tokio::test]
    async fn part3_2() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/2/v6/key?from=aaaa::aaaa&to=5555:ffff:c:0:0:c:1234:5555")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = collect_body(response).await;
        assert_eq!(body, "ffff:ffff:c::c:1234:ffff")
    }
}
