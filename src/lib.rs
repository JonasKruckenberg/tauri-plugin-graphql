// Copyright 2022 Jonas Kruckenberg
// SPDX-License-Identifier: MIT

//! This crate contains a Tauri plugin used to expose a [`juniper`] GraphQL
//! endpoint through Tauri's IPC system. This plugin can be used as safer
//! alternative to Tauri's existing Command API since both the Rust and
//! JavaScript side of the interface can be generated from a common schema.
//!
//! ## Rationale
//!
//! Especially in bigger projects that have specialized teams for the Frontend
//! and Rust core the existing command API falls short of being an optimal
//! solution. The Frontend is tightly coupled through `invoke()` calls to
//! backend commands, but there is no type-safety to alert Frontend developers
//! to changes in command signatures. This results in a very brittle interface
//! where changes on the Rust side will inadvertently break code in the
//! Frontend. This problem is similar exiting REST APIs, where the absence of a
//! formal contract between the server and the frontend makes future changes
//! very difficult.
//!
//! We can employ the same tactics used in traditional web development and use
//! shared schema that governs which types, methods, etc. are
//! available. GraphQL is such a schema language.
//!
//! ## Examples
//!
//! An example app that implements a very simple read-only todo-app using
//! GraphQL:
//!
//! ```rust
//! use juniper::{graphql_object, EmptySubscription, EmptyMutation, FieldResult, GraphQLObject, RootNode};
//! use tauri_plugin_graphql::Context as GraphQLContext;
//!
//! #[derive(GraphQLObject, Debug, Clone)]
//! struct ListItem {
//!     id: i32,
//!     text: String
//! }
//!
//! impl ListItem {
//!     pub fn new(text: String) -> Self {
//!         Self {
//!             id: rand::random::<i32>(),
//!             text
//!         }
//!     }
//! }
//!
//! struct Query;
//!
//! #[graphql_object(context = GraphQLContext)]
//! impl Query {
//!     fn list() -> FieldResult<Vec<ListItem>> {
//!         let item = vec![
//!             ListItem::new("foo".to_string()),
//!             ListItem::new("bar".to_string())
//!         ];
//!
//!         Ok(item)
//!     }
//! }
//!
//! // Consumers of this schema can only read data, so we must specifcy `EmptyMutation` and `EmptySubscription`
//! type Schema = RootNode<'static, Query, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;
//!
//! fn main() {
//!     let schema = Schema::new(
//!         Query,
//!         EmptyMutation::<GraphQLContext>::new(),
//!         EmptySubscription::<GraphQLContext>::new(),
//!     );
//!
//!     tauri::Builder::default()
//!         .plugin(tauri_plugin_graphql::init(schema));
//! }
//! ```
//!
//! ## Stability
//!
//! To work around limitations with the current command system, this plugin
//! directly implements an invoke handler instead of reyling on the
//! [`tauri::generate_handler`] macro.
//! Since the invoke handler implementation is not considered stable and might
//! change between releases **this plugin has no backwards compatibility
//! guarantees**.
//!
//! ## Subscriptions
//!
//! Subscriptions are currently not supported.

use juniper::http::GraphQLBatchRequest;
use std::sync::Arc;
use tauri::{
  plugin::{self, TauriPlugin},
  AppHandle, Manager, Runtime, Window,
};

pub use juniper;

pub struct Context<R: Runtime = tauri::Wry> {
  app: AppHandle<R>,
  window: Window<R>,
}

impl<R: Runtime> Context<R> {
  pub fn app(&self) -> &AppHandle<R> {
    &self.app
  }
  pub fn window(&self) -> &Window<R> {
    &self.window
  }
}

impl<R: Runtime> juniper::Context for Context<R> {}

/// Initializes the GraphQL plugin
///
/// This plugin exposes a juniper GraphQL endpoint via Tauri's IPC system,
/// allowing the frontend to invoke backend functionality through GraphQL.
/// **This does not open a web server.**
///
/// The `schema` argument must be a valid [`juniper::RootNode`].
///
/// ## Example
///
/// ```rust
/// use juniper::{EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode, graphql_object};
/// use tauri_plugin_graphql::Context as GraphQLContext;
///
/// #[derive(GraphQLObject)]
/// struct User {
///     id: i32,
///     name: String
/// }
///
/// struct Query;
///
/// // Implement resolvers for all possible queries.
/// #[graphql_object(context = GraphQLContext)]
/// impl Query {
///     fn me() -> FieldResult<User> {
///         Ok(User {
///             id: 1,
///             name: "Luke Skywalker".to_string(),
///         })
///     }
/// }
///
/// // A shorter alias for our apps schema. Note that this schema has no mutations or subscriptions,
/// // so we specify `EmptyMutation` and `EmptySubscription` respectively.
/// type Schema =
///     RootNode<'static, Query, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;
///
/// fn main() {
///     let schema = Schema::new(
///         Query,
///         EmptyMutation::<GraphQLContext>::new(),
///         EmptySubscription::<GraphQLContext>::new(),
///     );
///
///     tauri::Builder::default()
///         .plugin(tauri_plugin_graphql::init(schema));
///  }
/// ```
pub fn init<R, Query, Mutation, Subscription, S>(
  schema: juniper::RootNode<'static, Query, Mutation, Subscription, S>,
) -> TauriPlugin<R>
where
  R: Runtime,
  Query: juniper::GraphQLTypeAsync<S, Context = Context<R>> + Send + 'static,
  Query::TypeInfo: Send + Sync,
  Mutation: juniper::GraphQLTypeAsync<S, Context = Context<R>> + Send + 'static,
  Mutation::TypeInfo: Send + Sync,
  Subscription:
    juniper::GraphQLSubscriptionType<S, Context = Context<R>> + Send + 'static,
  Subscription::TypeInfo: Send + Sync,
  S: juniper::ScalarValue + Send + Sync + 'static,
{
  let schema = Arc::new(schema);

  plugin::Builder::new("graphql")
    .invoke_handler(move |invoke| {
      if invoke.message.command() != "graphql" {
        return invoke.resolver.reject(
          "Invalid graphql endpoint. Valid endpoints are: \"graphql\".",
        );
      }

      let window = invoke.message.window();
      let ctx = Context {
        app: window.app_handle(),
        window,
      };

      let schema = schema.clone();

      invoke.resolver.respond_async(async move {
        let req: GraphQLBatchRequest<S> =
          serde_json::from_value(invoke.message.payload().clone()).unwrap();

        let ret = req
          .execute::<Query, Mutation, Subscription>(&schema, &ctx)
          .await;

        let str = serde_json::to_string(&ret).unwrap();

        Ok((str, ret.is_ok()))
      });
    })
    .build()
}
