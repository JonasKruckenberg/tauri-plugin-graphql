import { invoke } from "@tauri-apps/api";
import {
  Exchange,
  makeErrorResult,
  makeResult,
  Operation,
  OperationResult,
} from "@urql/core";
import { print } from "graphql";
import {
  filter,
  make,
  merge,
  mergeMap,
  onPush,
  pipe,
  share,
  Source,
  takeUntil,
} from "wonka";

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
 *    url: 'graphql', // this endpoint is important
 *    exchanges: [tauriExchange]
 * })
 * ```
 */
export const tauriExchange: Exchange = ({ forward }) => {
  return (ops$) => {
    const sharedOps$ = share(ops$);

    const fetchResults$ = pipe(
      sharedOps$,
      filter((op) => op.kind === "query" || op.kind === "mutation"),
      mergeMap((operation) => {
        const { key } = operation;
        const teardown$ = pipe(
          sharedOps$,
          filter((op) => op.kind === "teardown" && op.key === key)
        );

        const args = {
          query: print(operation.query),
          variables: operation.variables || undefined,
        };

        const command = `plugin:graphql|${operation.context.url}`

        console.debug({
            type: "invokeRequest",
            message: "An invoke request is being executed.",
            operation,
            data: {
                command,
                args
            },
          })

        return pipe(
          makeInvokeSource(operation, command, args),
          takeUntil(teardown$),
          onPush((result) => {
            const error = !result.data ? result.error : undefined;

            console.debug({
              type: error ? "invokeError" : "invokeSuccess",
              message: `A ${
                error ? "failed" : "successful"
              } invoke response has been returned.`,
              operation,
              data: {
                value: error || result,
              },
            });
          })
        );
      })
    );

    const forward$ = pipe(
      sharedOps$,
      filter((op) => op.kind !== "query" && op.kind !== "mutation"),
      forward
    );

    return merge([fetchResults$, forward$]);
  };
};

type Response = [body: string, isOk: boolean];

function makeInvokeSource(
  operation: Operation,
  command: string,
  invokeArgs: Record<string, any>
): Source<OperationResult> {
  return make(({ next, complete }) => {
    let ended = false;

    Promise.resolve()
      .then(() => {
        if (ended) return;

        return invoke<Response>(command, invokeArgs);
      })
      .then((response) => {
        const [body, ok] = response!;
        const payload = JSON.parse(body);

        console.debug(response);

        if (!ok) throw new Error(payload.error);

        next(makeResult(operation, payload));
      })
      .then(complete)
      .catch((err) => {
        const result = makeErrorResult(operation, err, null);

        next(result);
        complete();
      });

    return () => {};
  });
}