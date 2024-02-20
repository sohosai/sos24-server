use anyhow::Context;
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{
    decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

use crate::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    pub iss: String,
    pub sub: String,
}

const JWK_URL: &str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";

pub(crate) async fn jwt_auth(
    mut request: Request<Body>,
    next: Next,
    State(config): State<Config>,
) -> Result<impl IntoResponse, StatusCode> {
    let authorization_header = request
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let authorization = authorization_header
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !authorization.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let jwt_token = authorization.trim_start_matches("Bearer ");

    match verify_id_token(jwt_token, &config.firebase_project_id).await {
        Ok(v) => {
            request.extensions_mut().insert(v);

            Ok(next.run(request).await)
        }
        Err(e) => {
            tracing::error!("Failed to verify: {e}");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub(crate) async fn verify_id_token(
    token: &str,
    firebase_project_id: &str,
) -> anyhow::Result<TokenData<Claims>> {
    let header = decode_header(token)?;
    let kid = header.kid.context("No key ID found in JWT header")?;
    let jwks: JwkSet = reqwest::get(JWK_URL).await?.json().await?;

    let jwk = jwks.find(&kid).context("Unknown key ID")?;
    let key = DecodingKey::from_jwk(jwk)?;

    let mut validation = Validation::new(Algorithm::RS256);

    validation.validate_exp = true;
    validation.validate_nbf = false;
    validation.set_audience(&[&firebase_project_id]);
    validation.set_issuer(&[format!(
        "https://securetoken.google.com/{}",
        &firebase_project_id
    )]);
    validation.sub = None;

    let data = decode(token, &key, &validation).context("Failed to validate JWT")?;

    Ok(data)
}
