## Ship readiness

**In progress; not release-qualified.** Linux app and site builds have run, and synthetic headless captures have been taken. Full tests, visual qualification, consumer installs, three-OS CI, and video export are still pending. No publication, deployment, signing, or live inference has occurred.

## Master changelog

- Integrated the distribution stack through `feat/service-install` (`41af8dd`) and both landing branches. Preserved Hallmark structure and the original 3D scene interactions.
- Fixed private-helper startup cleanup in Rust, TypeScript and Python, including cancellation. Invalid Rust helper overrides now fail explicitly.
- Hardened the control client's broker authentication, version negotiation, response size and total request deadline. Failed streams cannot be reused.
- Preserved workspace data on ordinary uninstall, corrected service argument escaping and socket defaults, and stopped the Windows task before unregistering it.
- Applied bounded security dependency updates and recorded their rationale. Terminal dependencies now require Rust 1.88.
- Established shared core brand tokens, documented the OriginLM/package naming boundary, and corrected blanket privacy claims.
- Made the dashboard overview lead with provider allowances and fresh authentication evidence. Added isolated native and HMR web example modes for private-data-free captures.
- Added production site prerendering, a larger animated hero and reduced-motion fallback. Lazy 3D files currently total approximately 273 KB gzipped excluding shared React.
- Began the 52-second fframes film using synthetic app captures; export and motion refinement remain in progress.

## Remaining work

Complete the engineering audit and remediation, including consistent SDK fallback and release artifact/signing flow. Finish product/design/docs work, film and social exports, then run the full verification matrix and update this report with exact evidence and honest platform limits.

## Evidence and ownership

The detailed finding ledger is `docs/review/V1-PRINCIPAL-REVIEW.md`; dependency review is `docs/dependency-audit-2026-10-28.md`. Checks added for fixes have not yet run: the brief reserves qualification for the final phase. Screenshot success is not live-account qualification.

Thomas will own eventual registry credentials/publication, signing identities, domain and hosting. No claim is made that those steps are configured or complete.
