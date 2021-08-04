use actix_web::HttpRequest;

use crate::errors::ServiceError;

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "sct07".repeat(8));
}

const SALT: &str = "supersecretsalt";

pub fn hash_password(passwd: &str) -> Result<String, ServiceError> {
    let config = argon2::Config {
        secret: SECRET_KEY.as_bytes(),
        ..Default::default()
    };
    argon2::hash_encoded(passwd.as_bytes(), &SALT.as_bytes(), &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify_hash(hash: &str, passwd: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded_ext(hash, &passwd.as_bytes(), &SECRET_KEY.as_bytes(), &[]).map_err(
        |err| {
            dbg!(err);
            ServiceError::Unauthorized
        },
    )
}

pub fn parse_request(req: HttpRequest) -> (String, String) {
    let mut email = String::new();
    let mut clearance = String::new();
    if let Some(e) = req.headers().get("user_email") {
        email = e.to_str().unwrap().to_owned();
    }
    if let Some(c) = req.headers().get("clearance") {
        clearance = c.to_str().unwrap().to_owned();
    }
    (email, clearance)
}
