use actix_web::{server, App, HttpRequest, HttpResponse, Responder};
use actix_web::middleware::Logger;
use env_logger;
use uuid::Uuid;
use chrono::prelude::*;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Pong {
    id: String,
    message: String,
    time: String
}

fn pong(_req: &HttpRequest) -> impl Responder {
    let p: Pong = Pong {
        id: Uuid::new_v4().to_string(),
        message: "empire.service".to_owned(),
        time: Utc::now().to_string()
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(p)
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    server::new(|| {
        App::new()
            .middleware(Logger::new("%a %r %{User-Agent}i %Dms"))
            .resource("/", |r| r.f(pong))
            .resource("/ping", |r| r.f(pong))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}