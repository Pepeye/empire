use actix::prelude::*;
use actix_web::{
    server, App, AsyncResponder, 
    Error, HttpRequest, HttpResponse, 
    FutureResponse, Responder, Json, State
};
use actix_web::middleware::Logger;
use env_logger;
// use uuid::Uuid;
use cuid;
use chrono::prelude::*;
use serde_derive::{Serialize, Deserialize};

use futures::future::Future;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod schema;
use schema::{create_schema, Schema};

#[derive(Serialize, Deserialize)]
struct Pong {
    id: String,
    message: String,
    time: String
}

// App state
struct AppState {
    executor: Addr<GraphQLExecutor>,
}

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

pub struct GraphQLExecutor {
    schema: std::sync::Arc<Schema>,
}

impl GraphQLExecutor {
    fn new(schema: std::sync::Arc<Schema>) -> GraphQLExecutor {
        GraphQLExecutor { schema }
    }
}

impl Actor for GraphQLExecutor {
    // sync Actor execution context
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &());
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

fn graphiql(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let html = graphiql_source("http://127.0.0.1:3000/graphql");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
    )
}
// function accepting a tuple of (State, GraphQLData)
fn graphql((st, data): (State<AppState>, Json<GraphQLData>),) -> FutureResponse<HttpResponse> {
    st.executor
        .send(data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().content_type("application/json").body(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn ping(_req: &HttpRequest<AppState>) -> impl Responder {
    let p: Pong = Pong {
        id: cuid::cuid().to_string(),
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
    let sys = actix::System::new("empire");

    let schema = std::sync::Arc::new(create_schema());
    let addr = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));
    
    server::new(move || {
        App::with_state(AppState{ executor: addr.clone()})
            // enable logger middleware
            .middleware(Logger::new("%a %r %{User-Agent}i %Dms"))
            .resource("/", |r| r.get().f(ping))
            .resource("/ping", |r| r.get().f(ping))
            .resource("/graphql", |r| r.post().with(graphql))
            .resource("/graphiql", |r| r.get().h(graphiql))
    })
    .bind("127.0.0.1:3000")
    .unwrap()
    .start();

    let _ = sys.run();
}