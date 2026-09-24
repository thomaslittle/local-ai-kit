# Contributing

This repository is at the planning/bootstrap stage. Start with [AGENTS.md](AGENTS.md), the [specification](docs/superpowers/specs/2026-09-24-local-ai-kit-design.md), and the [18-task plan](docs/superpowers/plans/2026-09-24-local-ai-kit-implementation.md).

Keep contributions modular, test-backed, and limited to the task being implemented. Preserve the Rust-core/language-neutral architecture and the separation of discovery, execution, and optional UI. Do not introduce a hosted-service requirement.

Provider contributions must include synthetic fixtures, negative cases, exact source/version/license evidence, supported operations/platforms, and honest live-test status. Installed does not mean authenticated; fixture-tested does not mean live-qualified.

Use a feature branch and bounded commits. Update [RESUME.md](docs/handoffs/RESUME.md) and the support/provenance documents as work progresses. Do not force-push others' work or edit the upstream reference repositories. Preserve attribution and do not add secrets or real user session logs.

Task 1 introduces executable verification commands. Until then, no workspace build/test commands exist; do not advertise passing product checks. After implementation begins, run the targeted checks in the plan and test installable artifacts before claiming SDK compatibility.

Publication, signing, changing visibility, spending inference quota, and modifying unrelated client settings require separate authorization.
