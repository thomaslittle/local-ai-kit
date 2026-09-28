# Third-party notices and reuse ledger

## Bootstrap status

This repository currently contains original planning and project-guidance documents. No implementation code, provider logos, screenshots, model weights, or runtime binaries have been copied from reference repositories into this bootstrap.

The research references T3 Code, CodeNotch, CodeNotch cross-platform, and Prmptr. Reference is not a claim of affiliation, endorsement, inherited security guarantees, or blanket rights to use vendor services.

## Reference projects

| Project | Source | Intended reference |
| --- | --- | --- |
| T3 Code | https://github.com/pingdotgg/t3code | Native client integration, provider instances, dynamic model options, picker patterns. |
| shadcn-ui/chatbot-template | https://github.com/shadcn-ui/chatbot-template | Chat interaction and model-selector patterns adapted for the demo Playground (see reuse ledger below). |
| CodeNotch cross-platform | https://github.com/thomaslittle/codenotch-crossplatform | Rust/platform discovery and usage parsing. |
| CodeNotch | https://github.com/vinzdg/codenotch | Broader provider/platform discovery. |
| Prmptr | https://github.com/thomaslittle/prmptr | Vertical provider rail and grouped model selection. |

See [SOURCE-AUDIT.md](SOURCE-AUDIT.md) for the previously inspected revisions and exact paths. Before porting each module, verify the license at the exact source revision and preserve required copyright and license text.

## Required ledger for every future reuse

Record the source URL, exact commit/blob, original path, copied or adapted symbols, destination path, license/SPDX identifier, copyright holder, retained notices, modifications, and applicable asset/model/dependency conditions. Add the required license text under a dedicated licenses directory when applicable. Do not mark this ledger complete merely because the root repository is MIT-licensed.

## Reuse ledger

### shadcn-ui/chatbot-template (demo Playground chat interface)

- Source: https://github.com/shadcn-ui/chatbot-template
- Source revision inspected: default branch on 2026-09-28 (`lib/models.ts`, `components/chat.tsx`, `components/prompt-form.tsx`, `components/model-select.tsx`, `components/chat-message.tsx`, `components/suggestions.tsx`, `app/globals.css`).
- License: MIT (SPDX: MIT), copyright holder `shadcn` (Copyright (c) 2026 shadcn). Full license text is retained in `licenses/shadcn-chatbot-template-MIT.txt`.
- Destination: `apps/demo/src/` (original implementation, no verbatim file copies).
- Adapted patterns: the empty-state-plus-suggestions start (`Empty`/`Suggestions`), the prompt composer with an inline model control and send/stop actions (`PromptForm`/`InputGroup` shape), the provider-grouped model picker (`ModelSelect`/`SelectGroup` shape), the user/assistant message thread with a pending indicator (`Chat`/`ChatMessage` shape), and the oklch design-token convention from `app/globals.css`.
- Modifications: reimplemented as dependency-free React components styled with Tailwind CSS and local shadcn-style primitives; the model list is grouped by provider with per-account route identity instead of a flat gateway list; every turn keeps the toolkit's explicit route selection, consent checkbox, and Run-click gate; synthetic fixture data stays labeled and live execution stays permissioned. No template code, assets, or model identifiers were copied.

Provider names are used for identification. Logos and trademarks are not granted by this repository's license. Provider authentication, subscription access, and endpoint use remain separate integration-policy questions.
