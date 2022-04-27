# Tauri Plugin graphql

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/tauri-plugin-graphql.svg
[crates-url]: https://crates.io/crates/tauri-plugin-graphql
[docs-badge]: https://img.shields.io/docsrs/tauri-plugin-graphql.svg
[docs-url]: https://docs.rs/tauri-plugin-graphql
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

A plugin for Tauri that enables type-safe IPC through GraphQL.

## Install

### Rust

```toml
[dependencies]
tauri-plugin-graphql = "0.2"
```

### JavaScript

The only client-side adapter currently is `tauri-plugin-graphql-urql`, a custom exchange for [`urql`]. 
If you need adapters for other GraphQL clients, open a PR!

| Package                       | Version (click for changelogs) |
|-------------------------------|--------------------------------|
| [`tauri-plugin-graphql-urql`] | [![urql adapter version][urql-adapter-version-badge]][urql-adapter-changelog]

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

## Contributing

If you want to help out, there are a few areas that need improvement:

- **Client Adapters** - Currently, only a urql adapter exists; having adapters for more client libraries would be very nice.

PRs are welcome!

## License

[MIT © Jonas Kruckenberg](./LICENSE)

[`tauri-plugin-graphql-urql`]: packages/urql
[urql-adapter-version-badge]: https://img.shields.io/npm/v/tauri-plugin-graphql-urql?label=%20
[urql-adapter-changelog]: packages/urql/CHANGELOG.md
[`urql`]: https://formidable.com/open-source/urql/
[`juniper::rootnode`]: https://docs.rs/juniper/latest/juniper/struct.RootNode.html
