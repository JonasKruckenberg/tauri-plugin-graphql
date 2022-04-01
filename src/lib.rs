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
//! We can employ the same techniques used in traditional web development and
//! use shared schema that governs which types, methods, etc. are
//! available. GraphQL is such a schema language.
//!
//! ## Examples
//!
//! For the following examples, it is assumed you are familiar with [`Tauri
//! Commands`][`Commands`], [`Events`] and [`GraphQL`].
//!
//! ### Queries
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
//! // Consumers of this schema can only read data,
//! // so we must specifcy `EmptyMutation` and `EmptySubscription`
//! type Schema = RootNode<
//!   'static,
//!   Query,
//!   EmptyMutation<GraphQLContext>,
//!   EmptySubscription<GraphQLContext>
//! >;
//!
//! let schema = Schema::new(
//!     Query,
//!     EmptyMutation::<GraphQLContext>::new(),
//!     EmptySubscription::<GraphQLContext>::new(),
//! );
//!
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_graphql::init(schema));
//! ```
//!
//! ### Mutations
//!
//! GraphQL mutations provide a way to update or create state in the Core.
//!
//! Similarly to queries, mutations have access to a context object and can
//! manipulate windows, menus or global state.
//!
//! ```rust
//! use juniper::{graphql_object, EmptySubscription, EmptyMutation, FieldResult, GraphQLObject, RootNode};
//! use tauri_plugin_graphql::Context as GraphQLContext;
//! use tauri::Manager;
//! use std::sync::Mutex;
//!
//! #[derive(Debug, Default)]
//! struct List(Mutex<Vec<ListItem>>);
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
//!     fn list(ctx: &GraphQLContext) -> FieldResult<Vec<ListItem>> {
//!       let list = ctx.app().state::<List>();
//!       let list = list.0.lock().unwrap();
//!         
//!       let items = list.iter().cloned().collect::<Vec<_>>();
//!
//!       Ok(items)
//!     }
//! }
//!
//! struct Mutation;
//!
//! #[graphql_object(context = GraphQLContext)]
//! impl Mutation {
//!   fn add_entry(ctx: &GraphQLContext, text: String) -> FieldResult<ListItem> {
//!     let list = ctx.app().state::<List>();
//!     let mut list = list.0.lock().unwrap();
//!
//!     let item = ListItem::new(text);
//!
//!     list.push(item.clone());
//!
//!     Ok(item)
//!   }
//! }
//!
//! // Consumers of this schema can read and write data,
//! // so set only Subscription as being empty
//! type Schema = RootNode<
//!   'static,
//!   Query,
//!   Mutation,
//!   EmptySubscription<GraphQLContext>
//! >;
//!
//! let schema = Schema::new(
//!     Query,
//!     Mutation,
//!     EmptySubscription::<GraphQLContext>::new(),
//! );
//!
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_graphql::init(schema))
//!     .setup(|app| {
//!       app.manage(List::default());
//!
//!       Ok(())
//!     });
//! ```
//!
//! ### Subscriptions
//!
//! > **Support for GraphQL Subscriptions requires the `subscriptions` feature
//! flag**
//!
//! GraphQL subscriptions are a way to push real-time data to the Frontend.
//! Similarly to queries, a client can request a set of fields, but instead of
//! immediately returning a single answer, a new result is sent to the Frontend
//! every time the Core sends one.
//!
//! Subscription resolvers should be async and must return a [`Stream`].
//!
//! ```rust
//! use juniper::{
//!  graphql_object, graphql_subscription, EmptyMutation, FieldResult, GraphQLObject,
//!  RootNode, FieldError,
//!  futures::Stream
//! };
//! use tauri_plugin_graphql::Context as GraphQLContext;
//! use std::pin::Pin;
//!
//! struct Query;
//!
//! #[graphql_object(context = GraphQLContext)]
//! impl Query {
//!   fn hello_world() -> FieldResult<String> {
//!     Ok("Hello World!".to_string())
//!   }
//! }
//!
//! struct Subscription;
//!
//! type StringStream = Pin<Box<dyn Stream<Item = Result<String, FieldError>> + Send>>;
//!
//! #[graphql_subscription(context = GraphQLContext)]
//! impl Subscription {
//!   async fn hello_world() -> StringStream {
//!     let stream = juniper::futures::stream::iter(vec![
//!       Ok("Hello".to_string()),
//!       Ok("World!".to_string())
//!     ]);
//!
//!     Box::pin(stream)
//!   }
//! }
//!
//! // This schema allows queries and subscriptions,
//! // so the mutations must be set to empty
//! type Schema = RootNode<
//!   'static,
//!   Query,
//!   EmptyMutation<GraphQLContext>,
//!   Subscription,
//! >;
//!
//! let schema = Schema::new(
//!   Query,
//!   EmptyMutation::<GraphQLContext>::new(),
//!   Subscription,
//! );
//!
//! tauri::Builder::default()
//!   .plugin(tauri_plugin_graphql::init(schema));
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
//! [`Stream`]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
//! [`Commands`]: https://tauri.studio/docs/guides/command
//! [`Events`]: https://tauri.studio/docs/guides/events
//! [`GraphQL`]: https://graphql.org

use juniper::http::GraphQLBatchRequest;
#[cfg(feature = "subscriptions")]
use juniper::{
  futures::{FutureExt, StreamExt, TryFutureExt},
  http::GraphQLRequest,
};
#[cfg(feature = "subscriptions")]
use juniper_subscriptions::Connection;
#[cfg(feature = "subscriptions")]
use serde::Deserialize;
use std::sync::Arc;
use tauri::{
  plugin::{self, TauriPlugin},
  AppHandle, InvokeError, Manager, Runtime, Window,
};

pub use juniper;

/// The context that is available to GraphQL resolvers.
pub struct Context<R: Runtime = tauri::Wry> {
  app: AppHandle<R>,
  window: Window<R>,
}

impl<R: Runtime> Context<R> {
  /// A handle to the Tauri application instance.
  pub fn app(&self) -> &AppHandle<R> {
    &self.app
  }
  /// A handle to the window that issued the GraphQL request.
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
/// type Schema = RootNode<
///   'static,
///   Query,
///   EmptyMutation<GraphQLContext>,
///   EmptySubscription<GraphQLContext>
/// >;
///
/// let schema = Schema::new(
///     Query,
///     EmptyMutation::<GraphQLContext>::new(),
///     EmptySubscription::<GraphQLContext>::new(),
/// );
///
/// tauri::Builder::default()
///     .plugin(tauri_plugin_graphql::init(schema));
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
  Subscription: juniper::GraphQLSubscriptionType<S, Context = Context<R>> + Send + 'static,
  Subscription::TypeInfo: Send + Sync,
  S: juniper::ScalarValue + Send + Sync + 'static,
{
  let schema = Arc::new(schema);

  plugin::Builder::new("graphql")
    .invoke_handler(move |invoke| {
      let window = invoke.message.window();
      #[cfg(feature = "subscriptions")]
      let subscription_window = window.clone();

      let ctx = Context {
        app: window.app_handle(),
        window,
      };

      let schema = schema.clone();

      match invoke.message.command() {
        "graphql" => invoke.resolver.respond_async(async move {
          let req: GraphQLBatchRequest<S> =
            serde_json::from_value(invoke.message.payload().clone())
              .map_err(InvokeError::from_serde_json)?;

          let ret = req
            .execute::<Query, Mutation, Subscription>(&schema, &ctx)
            .await;

          let str = serde_json::to_string(&ret).map_err(InvokeError::from_serde_json)?;

          Ok((str, ret.is_ok()))
        }),
        #[cfg(feature = "subscriptions")]
        "subscriptions" => invoke.resolver.respond_async(async move {
          let req: GraphQLSubscriptionRequest<S> =
            serde_json::from_value(invoke.message.payload().clone())
              .map_err(InvokeError::from_serde_json)?;

          let mut conn = juniper::http::resolve_into_stream(&req.inner, &schema, &ctx)
            .map_ok(|(stream, errors)| Connection::from_stream(stream, errors))
            .boxed()
            .await?;

          let event_id = &format!("graphql://{}", req.id);

          while let Some(result) = conn.next().await {
            let str = serde_json::to_string(&result).map_err(InvokeError::from_serde_json)?;

            subscription_window.emit(event_id, str)?;
          }
          subscription_window.emit(event_id, Option::<()>::None)?;

          Ok(())
        }),
        _ => invoke
          .resolver
          .reject("Invalid endpoint. Valid endpoints are: \"graphql\", \"subscriptions\"."),
      };
    })
    .build()
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Deserialize)]
pub struct GraphQLSubscriptionRequest<S: juniper::ScalarValue> {
  #[serde(
    flatten,
    bound(deserialize = "juniper::InputValue<S>: serde::Deserialize<'de>")
  )]
  inner: GraphQLRequest<S>,
  id: u32,
}
