import {invoke} from '@tauri-apps/api'
import {listen, Event} from '@tauri-apps/api/event'
import {
  Exchange,
  makeErrorResult,
  makeResult,
  Operation,
  OperationResult
} from '@urql/core'
import {
  ObserverLike,
  SubscriptionOperation
} from '@urql/core/dist/types/exchanges/subscription'
import {ExecutionResult, print} from 'graphql'
import {
  filter,
  make,
  merge,
  mergeMap,
  onPush,
  pipe,
  share,
  Source,
  takeUntil
} from 'wonka'

/**
 * An exchange for resolving GraphQL requests over Tauri's IPC bridge.
 *
 * **Example**
 *
 * ```javascript
 * import { createClient } from 'urql'
 * import { tauriExchange } from 'tauri-plugin-graphql/urql'
 *
 * const client = createClient({
 *    url: 'graphql', // this endpoint is important, don't touch
 *    exchanges: [tauriExchange]
 * })
 * ```
 */
export const invokeExchange: Exchange = ({forward}) => {
  return ops$ => {
    const sharedOps$ = share(ops$)

    const fetchResults$ = pipe(
      sharedOps$,
      filter(op => op.kind === 'query' || op.kind === 'mutation'),
      mergeMap(operation => {
        const {key} = operation
        const teardown$ = pipe(
          sharedOps$,
          filter(op => op.kind === 'teardown' && op.key === key)
        )

        const args = {
          query: print(operation.query),
          variables: operation.variables || undefined
        }

        const command = `plugin:graphql|${operation.context.url}`

        console.debug({
          type: 'invokeRequest',
          message: 'An invoke request is being executed.',
          operation,
          data: {
            command,
            args
          }
        })

        return pipe(
          makeInvokeSource(operation, command, args),
          takeUntil(teardown$),
          onPush(result => {
            const error = !result.data ? result.error : undefined

            console.debug({
              type: error ? 'invokeError' : 'invokeSuccess',
              message: `A ${
                error ? 'failed' : 'successful'
              } invoke response has been returned.`,
              operation,
              data: {
                value: error || result
              }
            })
          })
        )
      })
    )

    const forward$ = pipe(
      sharedOps$,
      filter(op => op.kind !== 'query' && op.kind !== 'mutation'),
      forward
    )

    return merge([fetchResults$, forward$])
  }
}

/**
 * @deprecated Use `invokeExchange` instead
 */
export const tauriExchange = invokeExchange

type Response = [body: string, isOk: boolean]

function makeInvokeSource(
  operation: Operation,
  command: string,
  invokeArgs: Record<string, any>
): Source<OperationResult> {
  return make(({next, complete}) => {
    let ended = false

    Promise.resolve()
      .then(() => {
        if (ended) return

        return invoke<Response>(command, invokeArgs)
      })
      .then(response => {
        const [body, ok] = response!
        const payload = JSON.parse(body)

        console.debug(response)

        if (!ok) throw new Error(payload.error)

        next(makeResult(operation, payload))
      })
      .then(complete)
      .catch(err => {
        const result = makeErrorResult(operation, err, null)

        next(result)
        complete()
      })

    return () => {
      ended = true
    }
  })
}

/**
 * A GraphQL Subscription transport that uses the Tauri IPC system.
 *
 * ## Example
 *
 * ```javascript
 * import { createClient } from 'urql'
 * import { tauriExchange } from 'tauri-plugin-graphql/urql'
 *
 * const client = createClient({
 *  url: "graphql", // this endpoint is important, don't touch
 *  exchanges: [
 *    invokeExchange,
 *    subscriptionExchange({
 *      forwardSubscription: (operation) => ({
 *        subscribe: (sink) => ({
 *          unsubscribe: subscribe(operation, sink),
 *        }),
 *      }),
 *    }),
 *  ],
 * });
 * ```
 *
 * @param operation The GraphQL Operation generated by urql
 * @param sink The sink that will receive the stream of subscription results
 * @returns
 */
export function subscribe(
  operation: SubscriptionOperation,
  sink: ObserverLike<ExecutionResult>
) {
  const id = Math.floor(Math.random() * 10000000)

  let unlisten: () => void = () => {
    Promise.resolve().then(() => invoke('plugin:graphql|subscriptions_end', {...operation, id}))
       .catch(err => console.error(err))
  }

  Promise.resolve()
    .then(async () =>
      listen(`graphql://${id}`, (event: Event<string | null>) => {
        if (event.payload === null) return sink.complete()
        sink.next(JSON.parse(event.payload))
      })
    )
    .then(_unlisten => (unlisten = _unlisten))
    .then(() => invoke('plugin:graphql|subscriptions', {...operation, id}))
    .catch(err => console.error(err))

  return unlisten
}
