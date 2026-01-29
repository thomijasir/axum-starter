use super::errors::HttpError;
use chrono::{Duration, Utc};
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn create_token(
    data: String,
    secret: &[u8],
) -> Result<String, Error> {
    // Validate input early
    if data.is_empty() {
        return Err(ErrorKind::InvalidSubject.into());
    }

    let now = Utc::now();
    let claims = TokenClaims {
        sub: data,
        iat: now.timestamp() as usize,
        exp: (now + Duration::hours(12)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn decode_token<T: AsRef<str>>(
    token: T,
    secret: &[u8],
) -> Result<(String, String), HttpError> {
    let token_ref = token.as_ref();

    match decode::<TokenClaims>(
        token_ref,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => {
            // Check if token has expired
            if token_data.claims.exp < token_data.claims.iat {
                return Err(HttpError::unauthorized("EXPIRED_SIGNATURE"));
            }
            // Parse token data
            let (user_id, email) = token_data
                .claims
                .sub
                .split_once("|")
                .ok_or_else(|| HttpError::unauthorized("INVALID_TOKEN"))?;

            Ok((user_id.to_string(), email.to_string()))
        }
        Err(err) => match err.kind() {
            ErrorKind::ExpiredSignature => Err(HttpError::unauthorized("EXPIRED_SIGNATURE")),
            ErrorKind::InvalidToken => Err(HttpError::unauthorized("INVALID_TOKEN")),
            ErrorKind::InvalidSignature => Err(HttpError::unauthorized("INVALID_SIGNATURE")),
            _ => Err(HttpError::unauthorized("UNAUTHORIZED")),
        },
    }
}
