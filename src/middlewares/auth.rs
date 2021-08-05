use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::header,
    Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::utils::decode_jwt;

pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = S::Response;
    type Request = S::Request;
    type Error = S::Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = S::Response;
    type Request = S::Request;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    type Error = S::Error;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, mut req: Self::Request) -> Self::Future {
        let mut token_verified = false;
        //skip for user regiter and login
        if req.uri().path() == "/users" && req.method() == "POST"
            || req.uri().path() == "/auth" && req.method() == "POST"
        {
            token_verified = true;
        }
        if let Some(t) = req.headers_mut().get("AUTHORIZATION") {
            if let Ok(token) = t.to_str() {
                if token.starts_with("bearer") || token.starts_with("Bearer") {
                    let token = token[6..].trim();
                    if let Ok(data) = decode_jwt(token.to_owned()) {
                        req.headers_mut().insert(
                            header::HeaderName::from_static("user_email"),
                            header::HeaderValue::from_str(&data.claims.email).unwrap(),
                        );
                        let user_type = if data.claims.clearance {
                            "admin"
                        } else {
                            "non_admin"
                        };
                        req.headers_mut().insert(
                            header::HeaderName::from_static("user_clearance"),
                            header::HeaderValue::from_str(&user_type).unwrap(),
                        );

                        token_verified = true;
                    }
                }
            }
        }

        if token_verified {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(serde_json::json!({ "error": "Unauthorized", "msg": "please login" }))
                        .into_body(),
                ))
            })
        }
    }
}
