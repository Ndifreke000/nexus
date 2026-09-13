use crate::models::user::Claims;
use crate::utils::errors::AppError;
use axum::http::HeaderMap;
use jsonwebtoken::{decode, DecodingKey, Validation};

/// Decode and validate a raw JWT string, regardless of where it came from.
pub fn decode_token(token: &str) -> Result<Claims, AppError> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_default();
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))
}

/// Extract and decode JWT claims from the `Authorization: Bearer <token>` header
pub fn extract_claims(headers: &HeaderMap) -> Result<Claims, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            AppError::Unauthorized("Missing or invalid Authorization header".to_string())
        })?;

    decode_token(token)
}

/// Same as `extract_claims`, but falls back to a `?token=` query param when
/// no Authorization header is present. Browsers' native EventSource API
/// can't send custom headers, so SSE endpoints need this fallback — everything
/// else should keep using `extract_claims` and the header.
pub fn extract_claims_with_query_fallback(
    headers: &HeaderMap,
    query_token: Option<&str>,
) -> Result<Claims, AppError> {
    if let Ok(claims) = extract_claims(headers) {
        return Ok(claims);
    }

    let token = query_token.ok_or_else(|| {
        AppError::Unauthorized("Missing or invalid Authorization header".to_string())
    })?;

    decode_token(token)
}
