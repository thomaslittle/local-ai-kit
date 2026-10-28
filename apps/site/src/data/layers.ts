export type Layer = {
  id: string;
  label: string;
  title: string;
  detail: string;
  meta: string;
};

/** The story the 3D hero and the promo video share. Same order, same words. */
export const LAYERS: Layer[] = [
  {
    id: "app",
    label: "Your app + dashboard",
    title: "Your app",
    detail: "Tauri, Electron, GPUI or the web. One small dependency, no provider code of your own.",
    meta: "tauri · electron · gpui · web",
  },
  {
    id: "dashboard",
    label: "",
    title: "The same runtime, on screen",
    detail: "Usage, quotas, reset times, models, routes and issues, read from the same local state.",
    meta: "see your AI",
  },
  {
    id: "sdk",
    label: "SDK",
    title: "Thin, typed clients",
    detail: "Rust, npm and Python surfaces over one protocol. No scanning on import, no secrets in the API.",
    meta: "rust · npm · python",
  },
  {
    id: "transport",
    label: "Transport",
    title: "Local transport only",
    detail: "stdio for native helpers, IPC for embedded hosts, Native Messaging for browser extensions.",
    meta: "stdio · ipc · native messaging",
  },
  {
    id: "runtime",
    label: "Runtime",
    title: "Grants, discovery, sessions",
    detail: "Every app gets its own revocable grant. Discovery is staged; execution needs permission.",
    meta: "local · permissioned",
  },
  {
    id: "adapters",
    label: "Adapters",
    title: "One identity model",
    detail: "Provider, installation, profile, account, connection, route, model and billing stay distinct.",
    meta: "evidence-first",
  },
  {
    id: "providers",
    label: "Providers",
    title: "The AI already installed",
    detail: "Claude Code, Codex, Cursor, Grok, Gemini, OpenCode, Copilot, Ollama, LM Studio and more.",
    meta: "18 providers documented",
  },
];
