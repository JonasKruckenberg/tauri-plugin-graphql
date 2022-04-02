# tauri-plugin-graphql

[![Npm][npm-badge]][npm-url]

Custom `urql` exchange that uses Tauri's IPC system to resolve queries against a GraphQL endpoint.

## Install

```console
$ pnpm add tauri-plugin-graphql-urql
# or
$ npm install tauri-plugin-graphql-urql
# or
$ yarn add tauri-plugin-graphql-urql
```

## Usage

You need to register the plugin with Tauri first! Please see the [top-level Readme] for a full example.

Import and use the custom exchange to connect to the GraphQL endpoint exposed over IPC.

```javascript
import { tauriExchange } from "tauri-plugin-graphql-urql";

const client = createClient({
  url: "graphql", // this value is important, don't touch
  exchanges: [tauriExchange],
});
```

## License

[MIT © Jonas Kruckenberg](./LICENSE)

[top-level Readme]: ../../README.md
[npm-url]: https://www.npmjs.com/package/tauri-plugin-graphql-urql
[npm-badge]: https://img.shields.io/npm/v/tauri-plugin-graphql-urql