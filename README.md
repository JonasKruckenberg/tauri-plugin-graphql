# Tauri Plugin graphql

A plugin for Tauri that enables type-safe IPC through GraphQL.

## Install

### Rust

This plugin is not yet published on crates.io, so for the time being you can install it from GitHub:

```toml
[dependencies]
tauri-plugin-positioner = { git = "https://github.com/JonasKruckenberg/tauri-plugin-graphql" }
```

### JavaScript

The only client-side adapter currently is `tauri-plugin-graphql-urql`, a custom exchange for [`urql`]. If you need adapters for other GraphQL clients, open a PR!

#### Urql

```console
$ pnpm add tauri-plugin-positioner
# or
$ npm install tauri-plugin-positioner
# or
$ yarn add tauri-plugin-positioner
```

## Usage

You need to register the plugin giving it a [`juniper::RootNode`] schema. This schema will be used to fulfill requests.

```rust
use juniper::{graphql_object, EmptySubscription, EmptyMutation, FieldResult, GraphQLObject, RootNode};
use tauri_plugin_graphql::Context as GraphQLContext;

#[derive(GraphQLObject, Debug, Clone)]
struct ListItem {
    id: i32,
    text: String
}

impl ListItem {
    pub fn new(text: String) -> Self {
        Self {
            id: rand::random::<i32>(),
            text
        }
    }
}

struct Query;

#[graphql_object(context = GraphQLContext)]
impl Query {
    fn list() -> FieldResult<Vec<ListItem>> {
        let item = vec![
            ListItem::new("foo".to_string()),
            ListItem::new("bar".to_string())
        ];

        Ok(item)
    }
}

// Consumers of this schema can only read data, so we must specifcy `EmptyMutation` and `EmptySubscription`
type Schema = RootNode<'static, Query, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;

fn main() {
    let schema = Schema::new(
        Query,
        EmptyMutation::<GraphQLContext>::new(),
        EmptySubscription::<GraphQLContext>::new(),
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_graphql::init(schema))
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
```

Now you can connect to the GraphQL endpoint using `urql`. A custom exchange is needed because this plugin transports all messages through Tauri's IPC system, not via an external HTTP server.

```javascript
import { tauriExchange } from "tauri-plugin-graphql-urql";

const client = createClient({
  url: "graphql", // this value is important, don't touch
  exchanges: [tauriExchange],
});
```

## Contributing

If you want to help out, there are a few areas that need improvement:

- **Subscriptions** - Currently, subscriptions are not supported, as the one-off nature of commands makes this difficult, but subscriptions can be implemented using events.
- **Client Adapters** - Currently, only a urql adapter exists; having adapters for more client libraries would be very nice.

PRs are welcome!

## License

[MIT © Jonas Kruckenberg](./LICENSE)

[`urql`]: https://formidable.com/open-source/urql/
[`juniper::rootnode`]: https://docs.rs/juniper/latest/juniper/struct.RootNode.html
