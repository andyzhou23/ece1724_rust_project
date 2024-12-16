use crate::AppConfig;
use actix_web::{web, HttpMessage, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::rngs::StdRng;
use rand::{distributions::Alphanumeric, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: usize, // user id
    pub exp: u64,       // Expiration unix timestamp
}

pub fn create_jwt(user_id: usize, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        user_id,
        exp: (chrono::Utc::now().timestamp() as u64) + 3600 * 24 * 30,
    };

    let header = Header::new(Algorithm::HS256);
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}

pub fn _generate_secret() -> String {
    let rng = StdRng::from_entropy();
    rng.sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
pub async fn http_validator(
    req: actix_web::dev::ServiceRequest,
    credentials: BearerAuth,
) -> Result<actix_web::dev::ServiceRequest, (actix_web::Error, actix_web::dev::ServiceRequest)> {
    let app_data = req
        .app_data::<web::Data<AppConfig>>()
        .expect("AppConfig not found");
    let secret = &app_data.jwt_secret;

    match validate_jwt(credentials.token(), secret) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => Err((
            actix_web::error::ErrorUnauthorized(json!({"error": "Invalid Token"})),
            req,
        )),
    }
}

pub fn get_user_id(req: &HttpRequest) -> usize {
    req.extensions()
        .get::<Claims>()
        .expect("Claims not found in request extensions")
        .user_id
}
