# Reusable React package verification

Windows, Node 22.20, React 19.3, 2026-09-24. The optional packages use the public SDK and headless selection package; provider logic stays in Rust.

- Snapshot store: six tests failed before implementation, then passed.
- React hooks: three meaningful red failures, then four passing SSR/DOM tests. Mounts and rerenders do not scan or request accounts.
- Styled selectors: six failures before implementation, then nine passing component tests, including all eight layouts, route identity, future native effort values, denied/late sensitive approvals, invalidated controlled selections, 5,000-model keyboard navigation, and escaped model labels.
- TypeScript SDK suite: 39 tests passed; selector core: 24 passed.
- TypeScript builds for SDK, selector, React bindings, and styled components passed.

The components accept snapshots; they do not authenticate, scan, or execute on render. Live account/stream verification belongs to the native runtime and is separate from these tests. Accessibility is covered by keyboard/semantic interaction tests, not a claim of a completed independent screen-reader audit. Real-browser checks and installable-package smoke checks are separate release gates.
