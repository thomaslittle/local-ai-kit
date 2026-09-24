# Showcase UI qualification

Actual UI sources and public TypeScript SDK were tested in Chromium through Playwright in the assistant Linux container. The first valid red UI test failed because the provider/mode interface did not exist. After implementation, all 13 interaction tests passed.

Coverage: provider rail, native future effort, removal of absent options, eight-layout selection equivalence, route-scoped favorites, strict capability filtering, unknown quotas without progress gauges, shared pool deduplication, streaming, approval denial, cancellation, clearing synthetic accounts on failed live connection, keyboard palette/search focus, 390px mobile drill-down/overflow, untrusted labels rendered as text, and a 5,000-model bounded list/search scenario.

Visual inspection: 1440px desktop and 390px mobile screenshots rendered and inspected. No external font/image dependencies or generated marketing screenshots were used.

Environment restriction: managed Chromium navigation is blocked. Browser tests use offline set_content plus a test-only in-memory transport to the same synthetic service. Separate Node tests verify actual HTTP/SSE behavior. Browser-to-HTTP navigation/connection and Windows/macOS UI execution are **not** claimed tested. No browser policy was changed.

The showcase is a working fixture demonstration, not a live detector. React packages, full virtualization, production native runtime, real accounts, and per-provider execution remain pending. The remaining plan is unchanged.
