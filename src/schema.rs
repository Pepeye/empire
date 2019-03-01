use juniper::{FieldResult, RootNode};
use uuid::Uuid;
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
pub struct Query;

graphql_object!(Query: () |&self| {

  // Arguments to resolvers can either be simple types or input objects
  field pong(&executor) -> FieldResult<Pong> {
     let p = Pong{
            id: Uuid::new_v4().to_string(),
            message: "empire app".to_owned(),
            time: Utc::now().to_string(),
        };
     Ok(p)
  }
});

pub struct Mutation;

graphql_object!(Mutation: () |&self| {
  field ping(&executor, p: Ping) -> FieldResult<Pong> {
    Ok(Pong{
      id: Uuid::new_v4().to_string(),
      message: p.message,
      time: Utc::now().to_string(),
    })
  }
});

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
  Schema::new(Query {}, Mutation {})
}
