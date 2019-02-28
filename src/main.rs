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

fn ping(_req: &HttpRequest) -> impl Responder {
    // let to = req.match_info().get("name").unwrap_or("World");
    // format!("Hello {}!", to)
    let p: Ping = Ping {
        id: Uuid::new_v4().to_string(),
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
            .resource("/", |r| r.f(ping))
            .resource("/{name}", |r| r.f(ping))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}