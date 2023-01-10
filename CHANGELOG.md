# Changelog

## \[2.1.0]

- Bump `async-graphql` to v5.
  - [9551574](https://www.github.com/your-org/tauri-plugin-graphql/commit/95515743ca143d68743ae9817865fe31dffd7fd9) Create update-async-graphql.md on 2023-01-10

## \[2.0.1]

- Drop tauri >1.0.0
  - [1d5a578](https://www.github.com/your-org/tauri-plugin-graphql/commit/1d5a578cb4595ee04b9705d448a5fd44f6c61659) Create adjust-dependency-version.md on 2022-09-22

## \[2.0.0]

- Added optional support for the GraphiQL IDE, to explore the Schema during Development. This is gated behind the `graphiql` feature flag.
  - [209e0b4](https://www.github.com/your-org/tauri-plugin-graphql/commit/209e0b416ad6d845f7121e8bf8c270ced4d81be9) update changesets on 2022-09-21
- Re-export `async_graphql` from crate.
  - [d587a33](https://www.github.com/your-org/tauri-plugin-graphql/commit/d587a33674a33a98edffec1b211dcf668905e7b5) feat: re-export async_graphql on 2022-09-22
- Switch from [`juniper`](https://github.com/async-graphql/async-graphql) to [`async-graphql`](https://github.com/graphql-rust/juniper). This adds integration with more crates, including the `log`, `tracing` and `opentelemetry` ecosystems. It also improves performance and simplifies the implementation.
  - [f7ec823](https://www.github.com/your-org/tauri-plugin-graphql/commit/f7ec823ac12cbded1fcc2d27ae0aae7251fe4269) add changeset on 2022-09-20
  - [78649ef](https://www.github.com/your-org/tauri-plugin-graphql/commit/78649ef6e58298876ebcfc5e48977adf31a2f688) Update switch-to-async-graphql.md on 2022-09-20
  - [209e0b4](https://www.github.com/your-org/tauri-plugin-graphql/commit/209e0b416ad6d845f7121e8bf8c270ced4d81be9) update changesets on 2022-09-21

## \[1.0.0]

- Mark as `stable`
  - [d6ba689](https://www.github.com/your-org/tauri-plugin-graphql/commit/d6ba6891a0132ea67fbf58b10b4d500c37592b35) Create mark-as-stable.md on 2022-06-16
- Update Tauri to v1.0.0
  - [42d98bb](https://www.github.com/your-org/tauri-plugin-graphql/commit/42d98bb34ea8d0c350659b669128ad632590131c) add tauri changefile on 2022-04-27
  - [5f9cbdf](https://www.github.com/your-org/tauri-plugin-graphql/commit/5f9cbdf21e3f6d55c5ae41a19751b7d37fc56683) \[create-pull-request] automated change on 2022-04-27
  - [01838a3](https://www.github.com/your-org/tauri-plugin-graphql/commit/01838a3cfb2eba7208e979dad89050902c36e2e7) Create update-tauri.md on 2022-05-25
  - [fa12cf5](https://www.github.com/your-org/tauri-plugin-graphql/commit/fa12cf5756a9d7ccf282a465a0376b09b499cd01) \[create-pull-request] automated change on 2022-05-25
  - [9a9bde4](https://www.github.com/your-org/tauri-plugin-graphql/commit/9a9bde4e8a5bc0b6d792fcf25c3530943ac27fb5) add changefile on 2022-06-16

## \[0.2.4]

- Update Tauri to v1.0.0-rc.12
  - [42d98bb](https://www.github.com/your-org/tauri-plugin-graphql/commit/42d98bb34ea8d0c350659b669128ad632590131c) add tauri changefile on 2022-04-27
  - [5f9cbdf](https://www.github.com/your-org/tauri-plugin-graphql/commit/5f9cbdf21e3f6d55c5ae41a19751b7d37fc56683) \[create-pull-request] automated change on 2022-04-27
  - [01838a3](https://www.github.com/your-org/tauri-plugin-graphql/commit/01838a3cfb2eba7208e979dad89050902c36e2e7) Create update-tauri.md on 2022-05-25

## \[0.2.3]

- Update dependencies.
  - [f15f662](https://www.github.com/your-org/tauri-plugin-graphql/commit/f15f6628a4aee793691b13a9b41c7884abd9c5d0) Create chore-update-deps.md on 2022-05-18
  - [4f73836](https://www.github.com/your-org/tauri-plugin-graphql/commit/4f73836c1843b31009289c47d1951e11a1980a49) Update chore-update-deps.md on 2022-05-19

## \[0.2.2]

- Update dependencies
  - [7c8a65d](https://www.github.com/your-org/tauri-plugin-graphql/commit/7c8a65d2ccdf9ea8f0cced2fc8734ba9aec9d1c0) Create update-deps.md on 2022-05-11
- Update documentation
  - [de4c1fa](https://www.github.com/your-org/tauri-plugin-graphql/commit/de4c1fa22fbaaa84f786f92568bc4a52201a3a2d) Create update-docs.md on 2022-05-03

## \[0.2.1]

- Update dependency `tauri` to `1.0.0-rc.8`
  - [42d98bb](https://www.github.com/your-org/tauri-plugin-graphql/commit/42d98bb34ea8d0c350659b669128ad632590131c) add tauri changefile on 2022-04-27

## \[0.2.0]

- Add support for GraphQL subscriptions. Subscriptions provide a type-safe way to model real-time state changes. This feature is currently gated by the `subscriptions` flag but will be enabled by default in a future release.
  - [d73c988](https://www.github.com/your-org/tauri-plugin-graphql/commit/d73c988230b5616dd3ce77c782a39cdfd2d10a8c) add changefiles on 2022-04-01
- Replace panicking behavior with proper error handling.
  - [d73c988](https://www.github.com/your-org/tauri-plugin-graphql/commit/d73c988230b5616dd3ce77c782a39cdfd2d10a8c) add changefiles on 2022-04-01

## \[0.1.0]

- Initial release.
  - [691ebc1](https://www.github.com/your-org/tauri-plugin-graphql/commit/691ebc16f90aba3f0d33ca6b2dadb0552b098239) initial commit on 2022-03-30
  - [85e7726](https://www.github.com/your-org/tauri-plugin-graphql/commit/85e7726dd7e55b70c7bc739835d4ff08685fe220) update changefile on 2022-03-30
