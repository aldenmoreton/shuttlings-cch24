use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use jsonwebtoken::{
    decode as jwt_decode, decode_header, encode, errors::ErrorKind, DecodingKey, EncodingKey,
    Header, Validation,
};

pub async fn wrap(jar: CookieJar, Json(body): Json<serde_json::Value>) -> impl IntoResponse {
    let token = encode(
        &Header::default(),
        &body,
        &EncodingKey::from_secret("secret".as_bytes()),
    )
    .unwrap();
    jar.add(Cookie::new("gift", token))
}

pub async fn unwrap(jar: CookieJar) -> impl IntoResponse {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.required_spec_claims = Default::default();
    validation.validate_exp = false;

    if let Some(cookie) = jar.get("gift") {
        let token = jwt_decode::<serde_json::Value>(
            &cookie.value(),
            &DecodingKey::from_secret("secret".as_bytes()),
            &validation,
        )
        .unwrap();
        return Json(token.claims).into_response();
    }

    StatusCode::BAD_REQUEST.into_response()
}

pub async fn decode(body: String) -> impl IntoResponse {
    let Ok(header) = decode_header(&body) else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let mut validation = Validation::new(header.alg);
    validation.required_spec_claims = Default::default();
    validation.validate_exp = false;

    let decoded: Result<jsonwebtoken::TokenData<serde_json::Value>, jsonwebtoken::errors::Error> =
        jwt_decode::<serde_json::Value>(
            &body,
            &DecodingKey::from_rsa_pem(include_bytes!("../../day16_santa_public_key.pem")).unwrap(),
            &validation,
        );

    match decoded.map_err(|e| e.into_kind()) {
        Ok(key) => Json(key.claims).into_response(),
        Err(ErrorKind::InvalidSignature) => StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}
