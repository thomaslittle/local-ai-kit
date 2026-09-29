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

- Source: https://github.com/shadcn-ui/chatbot-template (default branch, inspected 2026-09-28: `components/chat.tsx`, `chat-message.tsx`, `prompt-form.tsx`, `model-select.tsx`, `suggestions.tsx`, `components.json`, `app/globals.css`).
- License: MIT (SPDX: MIT), copyright holder `shadcn` (Copyright (c) 2026 shadcn). Full text is retained in `licenses/shadcn-chatbot-template-MIT.txt`.
- Destination: `apps/demo/src/pages/playground.tsx`. The chat primitives the template uses (`bubble`, `message`, `message-scroller`, `item`, plus `empty`, `input-group`, `alert`, `spinner`) are installed unmodified from the official shadcn registry with the shadcn CLI into `apps/demo/src/components/ui/`, not copied from the template repository.
- Adapted structure (reimplemented, not copied): the empty state with suggestion chips, the `MessageScroller` thread of `Message` + `Bubble` pairs with a "Thinking" shimmer, the destructive `Alert` for failures, and a `PromptForm` whose `InputGroup` carries the model control beside Send/Stop.
- Modifications: the model control is the provider-rail popover (`apps/demo/src/components/model-picker.tsx`) instead of a flat gateway `Select`; assistant text renders through `react-markdown` + `remark-gfm` inside Tailwind Typography styles instead of the template's `typeset.css`; every turn keeps the toolkit's explicit route selection, consent checkbox, and Run-click gate, and synthetic fixture data stays labeled. No API route, AI SDK, gateway, or tool code from the template is used.

### shadcn/ui components (demo UI primitives)

- Source: https://ui.shadcn.com (npm package `shadcn` 4.21.0, style `base-mira`, Base UI primitives, Phosphor icon library; see `apps/demo/components.json`).
- License: MIT (SPDX: MIT), copyright holder shadcn. The components are code generated into `apps/demo/src/components/ui/` by `npx shadcn@latest add`, which is the project's documented install method.
- Modifications: `Badge` gains `success`/`warning`/`info`/`brand` variants and `Alert` gains `warning`/`info` variants for evidence status; `Sidebar` no longer writes a state cookie; `Dialog` overlay opacity; `use-mobile` reads the media query through `useSyncExternalStore`. Everything else is as generated.

### Design tokens and typography (demo theme)

- Theme tokens (neutral surfaces, crimson accent hue 22.7, elevation ramp, radius) follow the crntly-desktop design system by the same author (https://github.com/thomaslittle/crntly-desktop, `packages/shared/src/App.css`); no license issue, recorded for provenance.
- Font: Outfit Variable via npm package `@fontsource-variable/outfit` 5.3.0, license SIL OFL 1.1, Copyright 2021 The Outfit Project Authors. License text retained in `licenses/outfit-font-OFL-1.1.txt`. The woff2 files are bundled by Vite and served from the same origin.
- Icons: Phosphor Icons (`@phosphor-icons/react`, MIT, Copyright (c) 2020 Phosphor Icons), used as an npm dependency; license text in `licenses/phosphor-icons-MIT.txt`.

### Provider brand marks (demo provider icons)

- Source: https://github.com/lobehub/lobe-icons, npm package `@lobehub/icons-static-svg@1.95.1` (`icons/openai.svg`, `codex.svg`, `claude.svg`, `claudecode.svg`, `anthropic.svg`, `grok.svg`, `xai.svg`, `antigravity.svg`, `gemini.svg`, `opencode.svg`, `lmstudio.svg`, `ollama.svg`; monochrome variants).
- License: MIT (SPDX: MIT), copyright holder Copyright (c) 2023 LobeHub. Full license text is retained in `licenses/provider-icons-lobehub-MIT.txt` (fetched from the repository's root `LICENSE`; the npm tarball ships none).
- Destination: `apps/demo/src/assets/providers/` (`claude-code.svg` is upstream `claudecode.svg`), with per-file provenance in `apps/demo/src/assets/providers/README.md`.
- Modifications: removed the `height="1em"`, `width="1em"` and inline `style` attributes; the 24x24 `viewBox` and all path data are unchanged.
- The marks are trademarks of their respective owners and are used only to identify the provider. This repository's MIT license does not grant trademark rights.

### Neutral "local runtime" glyph (demo provider icons)

- Source: https://github.com/lucide-icons/lucide, npm package `lucide-static@1.48.0` (`icons/cpu.svg`).
- License: ISC (SPDX: ISC), copyright holder Copyright (c) 2026 Lucide Icons and Contributors. Full license text is retained in `licenses/provider-icons-lucide-ISC.txt`.
- Destination: `apps/demo/src/assets/providers/local.svg`.
- Modifications: removed the `@license` comment, `class`, `width` and `height` attributes; `viewBox` and geometry unchanged.

Provider names are used for identification. Logos and trademarks are not granted by this repository's license. Provider authentication, subscription access, and endpoint use remain separate integration-policy questions.
