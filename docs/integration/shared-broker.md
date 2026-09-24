# Shared local broker

`local-ai-kit serve --ipc ADDRESS --allow-passive-discovery` runs one local service. Use a simple local pipe name on Windows and an absolute socket path inside your private application directory on Unix. Windows uses an owner-only pipe security descriptor and rejects remote pipe clients. Unix sockets use mode 0600. An existing listener is not unlinked or replaced.

The broker's stdin/stdout are a **trusted parent application control channel**, not a log: stdout returns an initial JSON descriptor with `address`, an expiring one-use `token`, and `protocolVersion`. The host may send `{"action":"authorize"}` on stdin to issue another independent client credential. Do not publish or log this control output. Closing the parent control input stops the broker; closing an application connection does not.

Applications connect using the native `connect --ipc ADDRESS` bridge. The token travels as the first JSON-RPC frame over stdin, never a command-line argument. The bridge does not start a broker automatically.

```ts
import {createLocalAI} from '@local-ai-kit/sdk';
import {openBrokerTransport} from '@local-ai-kit/sdk/node';
const ai = await createLocalAI({transport: await openBrokerTransport({
  executable: approvedBinaryPath,
  address: privateDescriptor.address,
  token: privateDescriptor.token,
})});
await ai.discovery.refresh({level:'passive'});
const snapshot = await ai.snapshot.get();
await ai.close(); // Other applications remain connected.
```

Python: `AsyncStdioTransport.open_broker(executable=..., address=..., token=...)` followed by `AsyncClient.connect(transport)`. Embedded Rust hosts can call `Broker::authorize(policy, execution_grant)` separately for each application. Each application has independently revocable scopes; no protocol request can grant itself broader access. Disconnect unregisters that client. Tokens expire after five minutes and are consumed once. The CLI parent determines its clients' common maximum policy; embedded hosts can use distinct policies.

Verified on Windows with two independent SDK helper processes, one shared scan, denied third-client access, one-use credentials, duplicate broker rejection, and client-specific revocation. Unix socket code must be qualified separately on Unix before claiming platform execution evidence. No implicit installation, telemetry, remote relay, or cross-user brokerage.
