# Generated contract and introspection verification

Executed on the Windows development worktree, 2026-09-24. This is code/fixture qualification, not a new live AI-account test.

- Rust workspace: 219 tests passed.
- Generator: 11 tests passed, including unknown/union fields, local references, Python forward types, method-map consistency, and OpenAPI security.
- TypeScript SDK: 33 tests passed; headless selector: 24 passed.
- Python SDK: 19 tests passed.
- Generated TypeScript consumer: typechecked; expected invalid requests and unrequested projection fields are rejected at compile time.
- Generated files: regeneration followed by check-only comparison passed.
- Strict Rust Clippy: passed. Native health/grants/adapter introspection regression tests pass.

Canonical Rust contracts now produce the domain JSON Schema, method manifest, OpenAPI, TypeScript declarations, Python TypedDict records and client stubs. Core selection and service execution reuse the canonical request records. Generated documentation is a compiled protocol superset; actual runtime methods and permissions are still negotiated. Dynamic diagnostic payloads are intentionally not given invented static shapes.

No live inference, credential export, package publication, or merge into main was performed.
