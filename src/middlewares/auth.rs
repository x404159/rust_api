use actix_identity::RequestIdentity;
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

use crate::models::user::SlimUser;

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

        if let Some(t) = req.get_identity() {
            let user = serde_json::from_str::<SlimUser>(t.as_str()).unwrap();
            req.headers_mut().insert(
                header::HeaderName::from_static("user_email"),
                header::HeaderValue::from_str(&user.email).unwrap(),
            );
            let user_type = if user.clearance { "admin" } else { "non_admin" };
            req.headers_mut().insert(
                header::HeaderName::from_static("user_clearance"),
                header::HeaderValue::from_str(&user_type).unwrap(),
            );

            token_verified = true;
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
