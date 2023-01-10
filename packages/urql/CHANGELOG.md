# Changelog

## \[2.0.3]

- Bump `async-graphql` to v5.
  - Bumped due to a bump in tauri-plugin-graphql.
  - [9551574](https://www.github.com/your-org/tauri-plugin-graphql/commit/95515743ca143d68743ae9817865fe31dffd7fd9) Create update-async-graphql.md on 2023-01-10

## \[2.0.2]

- Drop tauri >1.0.0
  - Bumped due to a bump in tauri-plugin-graphql.
  - [1d5a578](https://www.github.com/your-org/tauri-plugin-graphql/commit/1d5a578cb4595ee04b9705d448a5fd44f6c61659) Create adjust-dependency-version.md on 2022-09-22

## \[2.0.1]

- Added optional support for the GraphiQL IDE, to explore the Schema during Development. This is gated behind the `graphiql` feature flag.
  - Bumped due to a bump in tauri-plugin-graphql.
  - [209e0b4](https://www.github.com/your-org/tauri-plugin-graphql/commit/209e0b416ad6d845f7121e8bf8c270ced4d81be9) update changesets on 2022-09-21
- Re-export `async_graphql` from crate.
  - Bumped due to a bump in tauri-plugin-graphql.
  - [d587a33](https://www.github.com/your-org/tauri-plugin-graphql/commit/d587a33674a33a98edffec1b211dcf668905e7b5) feat: re-export async_graphql on 2022-09-22
- Switch from [`juniper`](https://github.com/async-graphql/async-graphql) to [`async-graphql`](https://github.com/graphql-rust/juniper). This adds integration with more crates, including the `log`, `tracing` and `opentelemetry` ecosystems. It also improves performance and simplifies the implementation.
  - Bumped due to a bump in tauri-plugin-graphql.
  - [f7ec823](https://www.github.com/your-org/tauri-plugin-graphql/commit/f7ec823ac12cbded1fcc2d27ae0aae7251fe4269) add changeset on 2022-09-20
  - [78649ef](https://www.github.com/your-org/tauri-plugin-graphql/commit/78649ef6e58298876ebcfc5e48977adf31a2f688) Update switch-to-async-graphql.md on 2022-09-20
  - [209e0b4](https://www.github.com/your-org/tauri-plugin-graphql/commit/209e0b416ad6d845f7121e8bf8c270ced4d81be9) update changesets on 2022-09-21

## \[0.2.6]

- Update Tauri to v1.0.0
  - [42d98bb](https://www.github.com/your-org/tauri-plugin-graphql/commit/42d98bb34ea8d0c350659b669128ad632590131c) add tauri changefile on 2022-04-27
  - [5f9cbdf](https://www.github.com/your-org/tauri-plugin-graphql/commit/5f9cbdf21e3f6d55c5ae41a19751b7d37fc56683) \[create-pull-request] automated change on 2022-04-27
  - [01838a3](https://www.github.com/your-org/tauri-plugin-graphql/commit/01838a3cfb2eba7208e979dad89050902c36e2e7) Create update-tauri.md on 2022-05-25
  - [fa12cf5](https://www.github.com/your-org/tauri-plugin-graphql/commit/fa12cf5756a9d7ccf282a465a0376b09b499cd01) \[create-pull-request] automated change on 2022-05-25
  - [9a9bde4](https://www.github.com/your-org/tauri-plugin-graphql/commit/9a9bde4e8a5bc0b6d792fcf25c3530943ac27fb5) add changefile on 2022-06-16

## \[0.2.5]

- Update Tauri to v1.0.0-rc.12
  - Bumped due to a bump in tauri-plugin-graphql.
  - [42d98bb](https://www.github.com/your-org/tauri-plugin-graphql/commit/42d98bb34ea8d0c350659b669128ad632590131c) add tauri changefile on 2022-04-27
  - [5f9cbdf](https://www.github.com/your-org/tauri-plugin-graphql/commit/5f9cbdf21e3f6d55c5ae41a19751b7d37fc56683) \[create-pull-request] automated change on 2022-04-27
  - [01838a3](https://www.github.com/your-org/tauri-plugin-graphql/commit/01838a3cfb2eba7208e979dad89050902c36e2e7) Create update-tauri.md on 2022-05-25

## \[0.2.4]

- Update dependencies.
  - Bumped due to a bump in tauri-plugin-graphql.
  - [f15f662](https://www.github.com/your-org/tauri-plugin-graphql/commit/f15f6628a4aee793691b13a9b41c7884abd9c5d0) Create chore-update-deps.md on 2022-05-18
  - [4f73836](https://www.github.com/your-org/tauri-plugin-graphql/commit/4f73836c1843b31009289c47d1951e11a1980a49) Update chore-update-deps.md on 2022-05-19

## \[0.2.3]

- Update dependencies
  - [7c8a65d](https://www.github.com/your-org/tauri-plugin-graphql/commit/7c8a65d2ccdf9ea8f0cced2fc8734ba9aec9d1c0) Create update-deps.md on 2022-05-11

## \[0.2.2]

- Update dependency `@tauri-apps/api` to `1.0.0-rc.4`
  - [5e68e7b](https://www.github.com/your-org/tauri-plugin-graphql/commit/5e68e7b7676f51bea1212a62f824708297e2df57) Create update-tauri-api.md on 2022-04-27

## \[0.2.1]

- Fix readme copy\&paste issue
  - [a9a3fed](https://www.github.com/your-org/tauri-plugin-graphql/commit/a9a3fedc958cd33b756d9613526cc555bff4e2f5) fix readme copy paste issue on 2022-04-02
- Update README.md
  - [0ebf8c6](https://www.github.com/your-org/tauri-plugin-graphql/commit/0ebf8c636834d4750b4b9c84eb575e140f7d6fd2) Create fix-update-readme.md on 2022-04-02

## \[0.2.0]

- Add support for GraphQL subscriptions. Subscriptions provide a type-safe way to model real-time state changes. This feature is currently gated by the `subscriptions` flag but will be enabled by default in a future release.
  - [d73c988](https://www.github.com/your-org/tauri-plugin-graphql/commit/d73c988230b5616dd3ce77c782a39cdfd2d10a8c) add changefiles on 2022-04-01
- **Breaking Change**: The `tauriExchange` has been renamed and deprecated and will be removed in a future release. Please use `invokeExchange` instead.
  - [d73c988](https://www.github.com/your-org/tauri-plugin-graphql/commit/d73c988230b5616dd3ce77c782a39cdfd2d10a8c) add changefiles on 2022-04-01

## \[0.1.1]

- Npm publish hotfix
  - [5ccffc3](https://www.github.com/your-org/tauri-plugin-graphql/commit/5ccffc37efa170db92a260ab6bd89e5fe40b625b) npm publish hotfix on 2022-03-30

## \[0.1.0]

- Initial release.
  - [691ebc1](https://www.github.com/your-org/tauri-plugin-graphql/commit/691ebc16f90aba3f0d33ca6b2dadb0552b098239) initial commit on 2022-03-30
  - [85e7726](https://www.github.com/your-org/tauri-plugin-graphql/commit/85e7726dd7e55b70c7bc739835d4ff08685fe220) update changefile on 2022-03-30
