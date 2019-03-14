use juniper::{FieldResult, RootNode, graphql_object};
use juniper::{GraphQLObject, GraphQLInputObject};

// use uuid::Uuid;
use cuid;
use chrono::prelude::*;

#[derive(GraphQLObject)]
/// A server response to a ping"
struct Pong {
    id: String,
    message: String,
    time: String
}

#[derive(GraphQLInputObject)]
/// A server ping"
struct Ping {
    message: String,
}

/// Query root type
pub struct Query;

// query root fields definitions
graphql_object!(Query: () |&self| {

  // Arguments to resolvers can either be simple types or input objects
  field pong(&executor) -> FieldResult<Pong> {
     let p = Pong{
            id: cuid::cuid().to_string(),
            message: "empire app".to_owned(),
            time: Utc::now().to_string(),
        };
     Ok(p)
  }
});

/// Mutation root type
pub struct Mutation;

// mutation root fields definition
graphql_object!(Mutation: () |&self| {
  field ping(&executor, p: Ping) -> FieldResult<Pong> {
    Ok(Pong{
      id: cuid::cuid().to_string(),
      message: p.message,
      time: Utc::now().to_string(),
    })
  }
});

/// schema type alias
pub type Schema = RootNode<'static, Query, Mutation>;

/// Schema root - queries and mutations
pub fn create_schema() -> Schema {
  Schema::new(Query {}, Mutation {})
}
