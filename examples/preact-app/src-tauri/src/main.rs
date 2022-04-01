#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use juniper::{
  graphql_object, graphql_subscription, EmptyMutation, FieldResult, GraphQLObject,
  RootNode, FieldError,
  futures::Stream
};
use tauri_plugin_graphql::Context as GraphQLContext;
use std::pin::Pin;

#[derive(GraphQLObject, Debug, Clone)]
struct Human {
  name: String,
}

struct Query;

#[graphql_object(context = GraphQLContext)]
impl Query {
  fn hero() -> FieldResult<Human> {
    Ok(Human {
      name: "Luke Skywalker".to_string(),
    })
  }
}

pub struct Subscription;

type StringStream = Pin<Box<dyn Stream<Item = Result<String, FieldError>> + Send>>;

#[graphql_subscription(context = GraphQLContext)]
impl Subscription {
    async fn hello_world() -> StringStream {
        let stream = juniper::futures::stream::iter(vec![
            Ok(String::from("Hello")),
            Ok(String::from("World!"))
        ]);

        Box::pin(stream)
    }
}

type Schema = RootNode<
  'static,
  Query,
  EmptyMutation<GraphQLContext>,
  Subscription,
>;

fn main() {
  let schema = Schema::new(
    Query,
    EmptyMutation::<GraphQLContext>::new(),
    Subscription,
  );

  tauri::Builder::default()
    .plugin(tauri_plugin_graphql::init(schema))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
