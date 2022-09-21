import React from 'react'
import ReactDOM from 'react-dom/client'
import { GraphiQL } from 'graphiql'
import { createGraphiQLFetcher } from '@graphiql/create-fetcher'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'

async function invokeFetch(_url: globalThis.RequestInfo | URL, options: globalThis.RequestInit) {
  // the JSON.parse here is a bit annoying, but there is currently no good way around it
  // @ts-ignore
  const [str, isOk] = await invoke('plugin:graphql|graphql', options.body && JSON.parse(options.body.toString()))
  return new Response(str, isOk)
}

class Response {
  #str: string
  readonly ok: boolean

  constructor(str: string, ok: boolean) {
    this.#str = str
    this.ok = ok
  }

  json() {
    return JSON.parse(this.#str)
  }
}

class InvokeSubscriptionClient {
  // @ts-ignore
  subscribe(payload, sink) {
    let unlisten = () => {}

    const id = Math.floor(Math.random() * 10000000)

    console.log(payload);

    Promise.resolve()
      .then(async () =>
        listen(`graphql://${id}`, (event) => {
          if (event.payload === null) return sink.complete()
          // @ts-ignore
          sink.next(JSON.parse(event.payload))
        })
      )
      .then(_unlisten => (unlisten = _unlisten))
      .then(() =>
        invoke('plugin:graphql|subscriptions', {...payload, id})
      )
      .catch(err => console.error(err))

    return unlisten
  }
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    {/* @ts-ignore */}
    <GraphiQL fetcher={createGraphiQLFetcher({
      // @ts-ignore
      wsClient: new InvokeSubscriptionClient(),
      // @ts-ignore
      fetch: invokeFetch
    })}/>
  </React.StrictMode>
)
