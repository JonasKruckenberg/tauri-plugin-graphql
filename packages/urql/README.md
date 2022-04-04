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
import { invokeExchange } from "tauri-plugin-graphql-urql";

const client = createClient({
  url: "graphql", // this value is important, don't touch
  exchanges: [invokeExchange],
});

const heroQuery = `
query {
  hero {
    name
  }
}
`;

function Hero() {
  const [result, reexecuteQuery] = useQuery({
    query: heroQuery,
  });

  const { data, fetching, error } = result;

  if (fetching) return <p>Loading...</p>;
  if (error) return <p>Oh no... {error.message}</p>;

  return (
    <p>The hero is {data.hero.name}</p>
  );
}
```

### Subscriptions

This adapter also supports subscriptions.

```javascript
import { subscribe } from "tauri-plugin-graphql-urql";
import { subscriptionExchange } from "@urql/preact";

const client = createClient({
  url: "graphql",
  exchanges: [
    subscriptionExchange({
      forwardSubscription: (operation) => ({
        subscribe: (sink) => ({
          unsubscribe: subscribe(operation, sink),
        }),
      }),
    }),
  ],
});

const newMessages = `
  subscription MessageSub {
    helloWorld
  }
`;

function handleSubscription(messages = [], response) {
  return [response.helloWorld, ...messages];
};

function TestSubscription() {
  const [res] = useSubscription({ query: newMessages }, handleSubscription);

  if (!res.data) {
    return <p>No new messages</p>;
  }

  return (
        <p>
          {res.data}
        </p>
  ); 
}
```

## License

[MIT © Jonas Kruckenberg](./LICENSE)

[top-level Readme]: ../../README.md
[npm-url]: https://www.npmjs.com/package/tauri-plugin-graphql-urql
[npm-badge]: https://img.shields.io/npm/v/tauri-plugin-graphql-urql