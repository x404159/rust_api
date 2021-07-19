use actix_identity::Identity;

use crate::{errors::ServiceError, models::user::SlimUser};

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

pub fn is_admin(id: &Identity) -> Result<bool, ServiceError> {
    if id.identity().is_none() {
        return Err(ServiceError::Unauthorized);
    }
    match serde_json::from_str::<SlimUser>(&id.identity().unwrap()) {
        Ok(u) => {
            //checking for admin previleges
            if !u.clearance {
                return Err(ServiceError::BadRequest(
                    "only admins can access this route".to_owned(),
                ));
            }
        }
        Err(_) => return Err(ServiceError::Unauthorized),
    };
    Ok(true)
}

pub fn get_logged_user(id: &Identity) -> Result<SlimUser, ServiceError> {
    if let Some(u) = id.identity() {
        if let Ok(v) = serde_json::from_str::<SlimUser>(&u) {
            Ok(v)
        } else {
            Err(ServiceError::InternalServerError)
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}
