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
tauri-plugin-graphql = "2.0.0"
```

### JavaScript

The only client-side adapter currently is `tauri-plugin-graphql-urql`, a custom exchange for [`urql`]. 
If you need adapters for other GraphQL clients, open a PR!

| Package                       | Version (click for changelogs) |
|-------------------------------|--------------------------------|
| [`tauri-plugin-graphql-urql`] | [![urql adapter version][urql-adapter-version-badge]][urql-adapter-changelog]

## Usage

You need to register the plugin giving it a [`async_graphql::Schema`]. This schema will be used to fulfill requests.

```rust
use async_graphql::{Schema, Object, EmptySubscription, EmptyMutation, Result as GraphQLResult, SimpleObject};

#[derive(SimpleObject, Debug, Clone)]
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

#[Object]
impl Query {
    async fn list(&self) -> GraphQLResult<Vec<ListItem>> {
        let item = vec![
            ListItem::new("foo".to_string()),
            ListItem::new("bar".to_string())
        ];

        Ok(item)
    }
}

fn main() {
    let schema = Schema::new(
        Query,
        EmptyMutation,
        EmptySubscription,
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
[`async_graphql::Schema`]: https://docs.rs/async-graphql/latest/async_graphql/struct.Schema.html
