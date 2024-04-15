use std::sync::Arc;

use anyhow::Context as _;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{
    decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

use sos24_use_case::context::Context;

use crate::{error::AppError, module::Modules};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    pub iss: String,
    pub sub: String,
    pub email_verified: bool,
}

const JWK_URL: &str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";

pub(crate) async fn jwt_auth(
    State(modules): State<Arc<Modules>>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("ユーザー認証を行います");

    let authorization_header = request.headers().get("Authorization").ok_or(AppError::new(
        StatusCode::UNAUTHORIZED,
        "auth/missing-authorization-header".to_string(),
        "Authorization header is missing.".to_string(),
    ))?;
    let authorization = authorization_header.to_str().map_err(|e| {
        AppError::new(
            StatusCode::UNAUTHORIZED,
            "auth/invalid-authorization-header".to_string(),
            e.to_string(),
        )
    })?;

    if !authorization.starts_with("Bearer ") {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "auth/invalid-authorization-header".to_string(),
            "Authorization header is invalid. It should start with 'Bearer'.".to_string(),
        ));
    }

    let jwt_token = authorization.trim_start_matches("Bearer ");

    let token = match verify_id_token(jwt_token, &modules.config().firebase_project_id).await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Failed to verify: {e}");
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "auth/invalid-token".to_string(),
                e.to_string(),
            ));
        }
    };

    // メールが認証されているか確認
    if modules.config().require_email_verification && !token.claims.email_verified {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "auth/email-not-verified".to_string(),
            "Email is not verified.".to_string(),
        ));
    }

    // もし user_id 以上のものを Extension に入れるなら、ここで渡す
    let ctx = Context::new(token.claims.sub.clone());
    request.extensions_mut().insert(ctx);

    tracing::info!("ユーザー認証が完了しました");
    Ok(next.run(request).await)
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
