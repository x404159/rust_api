use actix_web::{HttpResponse, Responder};

pub async fn handle_404() -> impl Responder {
    HttpResponse::NotFound().body("<h1 style='color: red; position: absolute; top: 50%; left: 30%; background-color: #000000; border-radius: 5px; padding: 1rem;'>You are not allowed to enter the forbidden site!!!</h1>")
}
