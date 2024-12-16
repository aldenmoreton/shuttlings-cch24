use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[derive(serde::Serialize, serde::Deserialize)]
struct Claim {
    exp: usize,
    body: serde_json::Value,
}

pub async fn wrap(jar: CookieJar, Json(body): Json<serde_json::Value>) -> impl IntoResponse {
    let token = encode(
        &Header::default(),
        &Claim {
            exp: usize::MAX,
            body,
        },
        &EncodingKey::from_secret("secret".as_bytes()),
    )
    .unwrap();
    jar.add(Cookie::new("gift", token))
}

pub async fn unwrap(jar: CookieJar) -> impl IntoResponse {
    if let Some(cookie) = jar.get("gift") {
        let token = decode::<Claim>(
            &cookie.value(),
            &DecodingKey::from_secret("secret".as_bytes()),
            &Validation::default(),
        )
        .unwrap();
        return Json(token.claims.body).into_response();
    }

    StatusCode::BAD_REQUEST.into_response()
}
