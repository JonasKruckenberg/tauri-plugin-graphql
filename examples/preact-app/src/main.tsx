import { createClient, Provider, subscriptionExchange } from "@urql/preact";
import { render } from "preact";
import { invokeExchange, subscribe } from "tauri-plugin-graphql-urql";
import { App } from "./app";
import "./index.css";

const client = createClient({
  url: "graphql",
  exchanges: [
    invokeExchange,
    subscriptionExchange({
      forwardSubscription: (operation) => ({
        subscribe: (sink) => ({
          unsubscribe: subscribe(operation, sink),
        }),
      }),
    }),
  ],
});

render(
  <Provider value={client}>
    <App />
  </Provider>,
  document.getElementById("app")!
);
