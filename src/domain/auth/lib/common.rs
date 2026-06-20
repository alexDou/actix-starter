use std::env;
use actix_session::Session;
use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, error::ErrorUnauthorized};
use futures_util::future::{Ready, ready};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error as JWTError,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").expect("Insufficient API configuration")
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

pub fn create_jwt(user_id: &Uuid) -> Result<String, JWTError> {
    let secret = get_jwt_secret();
    let expiration = (OffsetDateTime::now_utc() + Duration::hours(24)).unix_timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub struct AuthenticatedUser {
    pub user_id: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req_jwt_token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .unwrap();
        let jwt_secret = get_jwt_secret();

        let session_user_id = Session::from_request(req, payload)
            .into_inner()
            .unwrap()
            .get::<String>("user_id")
            .unwrap()
            .unwrap();

        match decode::<Claims>(
            req_jwt_token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(data) => {
                if session_user_id != data.claims.sub {
                    return ready(Err(ErrorUnauthorized("Failed to approve user authentication")));
                }
                return ready(Ok(AuthenticatedUser {
                    user_id: data.claims.sub,
                }));
            }
            Err(_) => ready(Err(ErrorUnauthorized("Invalid token"))),
        }
    }
}
