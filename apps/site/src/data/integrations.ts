export type Integration = {
  id: string;
  name: string;
  tag: string;
  title: string;
  description: string;
  language: string;
  code: string;
  meta: string[];
  note: string;
};

export const INTEGRATIONS: Integration[] = [
  {
    id: "tauri",
    name: "Tauri",
    tag: "rust",
    title: "Link the crate, keep the renderer thin",
    description: "The embedded runtime runs in-process. The webview only asks your own commands for data; it never constructs a policy.",
    language: "Rust · src-tauri",
    meta: ["in-process", "no sidecar required", "policy in Rust"],
    note: "Alternatively ship the helper as an externalBin sidecar. Never expose ClientPolicy construction to the renderer.",
    code: `use lai_kit::{ClientPolicy, Kit, Snapshot};

// Build once at startup; a passive policy scans nothing by itself.
let kit = Kit::builder().build()?;

#[tauri::command]
async fn ai_snapshot() -> Result<Snapshot, String> {
    let app = kit
        .connect("com.acme.app", ClientPolicy::passive())
        .map_err(|error| error.to_string())?;
    let snapshot = app.scan().await.map_err(|error| error.to_string())?;
    app.close(); // Releases this app's permissions, not the user's service.
    Ok(snapshot)
}`,
  },
  {
    id: "electron",
    name: "Electron",
    tag: "node",
    title: "One stdio helper, unpacked from asar",
    description: "The SDK launches only the binary you pass it, with an explicit argument array and no shell.",
    language: "TypeScript · main process",
    meta: ["asarUnpack", "stdio", "main process only"],
    note: "Add the helper glob to asarUnpack. The SDK detects an asar path without an unpacked sibling and fails with a readable error.",
    code: `import { app } from "electron";
import path from "node:path";
import { createLocalAI } from "@local-ai-kit/sdk";
import { openStdioTransport } from "@local-ai-kit/sdk/node";

const executable = path.join(
  process.resourcesPath,
  "app.asar.unpacked",
  "bin",
  "originlm",
);

const transport = await openStdioTransport({
  executable,
  args: ["serve", "--stdio"],
});

const ai = await createLocalAI({ transport });
const snapshot = await ai.snapshot.get();
app.on("before-quit", () => void ai.close());`,
  },
  {
    id: "gpui",
    name: "GPUI",
    tag: "rust",
    title: "A native view over the same runtime",
    description: "No webview, no extra service: scan on demand and render the evidence the runtime already normalizes.",
    language: "Rust · GPUI",
    meta: ["native", "in-process", "owned cancellation"],
    note: "Scanning is explicit. Reading an empty snapshot does not inspect the machine.",
    code: `use gpui::Context;
use lai_kit::{ClientPolicy, Kit};

struct UsageView {
    kit: Kit,
    snapshot: Option<lai_kit::Snapshot>,
}

impl UsageView {
    fn refresh(&mut self, cx: &mut Context<Self>) {
        let app = self.kit.connect("dev.originlm.usage", ClientPolicy::passive());
        cx.spawn(async move |this, cx| {
            let Ok(app) = app else { return };
            if let Ok(snapshot) = app.scan().await {
                this.update(cx, |view, cx| {
                    view.snapshot = Some(snapshot);
                    cx.notify();
                }).ok();
            }
            app.close();
        }).detach();
    }
}`,
  },
  {
    id: "node",
    name: "Node",
    tag: "node",
    title: "Server-side or CLI, same client",
    description: "Use the shared service when it is running; otherwise bundle the helper. Grants are requested explicitly.",
    language: "TypeScript · Node",
    meta: ["shared broker", "bundled fallback", "typed"],
    note: "A protocol mismatch falls back to the bundled copy and is recorded as FALLBACK_PROTOCOL_MISMATCH; it is never fatal.",
    code: `import { createLocalAI } from "@local-ai-kit/sdk";
import { openSharedBrokerTransport } from "@local-ai-kit/sdk/node";

const transport = await openSharedBrokerTransport({
  address: "127.0.0.1:4499",
  tokenFile: "/home/me/.local-ai-kit/token",
  executable: "/opt/originlm/bin/originlm",
  startArgs: ["serve", "--stdio", "--allow-passive-discovery"],
});

const ai = await createLocalAI({ transport });
await ai.grants.request({ scopes: ["snapshot.read"] });

const snapshot = await ai.snapshot.get();
console.log(snapshot.providers.map((provider) => provider.name));
await ai.close();`,
  },
  {
    id: "python",
    name: "Python",
    tag: "python",
    title: "Metadata and text turns",
    description: "The Python client speaks the same protocol. Discovery stays passive until you ask for more.",
    language: "Python",
    meta: ["stdio", "context manager", "no scanning on import"],
    note: "Nothing is imported at install time, and no provider client is launched until you call refresh.",
    code: `from local_ai_kit import Client

with Client.from_stdio(
    executable="/opt/originlm/bin/originlm",
    args=["serve", "--stdio", "--allow-passive-discovery"],
    timeout=30,
) as ai:
    ai.discovery.refresh({"level": "passive"})
    snapshot = ai.snapshot.get()

    for connection in snapshot["connections"]:
        print(connection["displayName"], connection["status"])`,
  },
  {
    id: "web",
    name: "Web",
    tag: "web",
    title: "Extension first, pairing as the fallback",
    description: "In a browser, the extension plus Native Messaging host is the primary bridge. No port opens for it.",
    language: "TypeScript · extension page",
    meta: ["native messaging", "no port", "origin approval"],
    note: "Each origin needs a one-time, revocable approval. The page receives results only, never credentials. Loopback pairing is the fallback.",
    code: `import { createLocalAI, createNativeMessagingTransport } from "@local-ai-kit/sdk";

// Only after the user's explicit Connect action, in the extension page.
const transport = createNativeMessagingTransport({
  connect: () => chrome.runtime.connectNative("dev.originlm.bridge"),
});

const ai = await createLocalAI({ transport });
const snapshot = await ai.snapshot.get();

document.querySelector("[data-originlm-status]")!.textContent =
  snapshot.connections.length
    ? \`Local AI connected · \${snapshot.providers.map((p) => p.name).join(", ")}\`
    : "Not detected";

await ai.close();`,
  },
];
