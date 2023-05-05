use actix_web::HttpMessage;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use uuid::Uuid;

use crate::errors::ServiceError;


// TODO: Move them to .env file
const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret";
const JWT_TOKEN_DURATION_IN_HOURS: i64 = 24;
const ALGORIITHM: Algorithm = Algorithm::HS256;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
struct Claim {
    user_id: i64, // user id
    exp: usize,  // expiry time
    iat: usize, // issued at
}

pub fn create_jwt(user_id: i64) -> Result<String, ServiceError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(JWT_TOKEN_DURATION_IN_HOURS))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claim {
        user_id: user_id,
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
    };
    let header = Header::new(ALGORIITHM);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| ServiceError::JWTTokenCreationError)
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    println!("validator for path: {}", req.path());
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token()) {
        Ok(res) => {
            req.request().extensions_mut().insert(res.user_id);
            Ok(req)
        }
        Err(_) => Err((AuthenticationError::from(config).into(), req)),
    }
}

fn validate_token(token: &str) -> Result<Claim, jsonwebtoken::errors::Error> {
    let token = token.trim_start_matches(BEARER);
    let decoded = decode::<Claim>(token, &DecodingKey::from_secret(JWT_SECRET), & Validation::new(ALGORIITHM));
    match decoded {
        Ok(claim) => {
            let now = Utc::now().timestamp() as usize;
            if now > claim.claims.exp {
                return Err((jsonwebtoken::errors::ErrorKind::ExpiredSignature).into());
            }
            Ok(claim.claims)
        },
        Err(e) => {
            Err(e)
        }
    }
}

pub fn get_new_refresh_token() -> String {
    Uuid::new_v4().to_string()
}