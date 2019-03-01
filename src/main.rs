use actix_web::{server, App, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;
use chrono::prelude::*;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Ping {
    id: String,
    message: String,
    time: String,
}

fn pong(req: &HttpRequest) -> impl Responder {
    let p: Ping = Ping {
        id: req.,
        message: "empire.service".to_owned(),
        time: Utc::now().to_string()
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(p)
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(pong))
            .resource("/ping", |r| r.f(pong))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}