## Ship readiness

**In progress; not release-qualified.** Linux Rust service, adapter and GPUI builds, SDK builds, web/site builds and site prerendering have run. Example-mode headless app captures and intermediate film/scene renders have been inspected. Full tests, accessibility/performance qualification, local consumer installs, three-OS CI remain pending. The revised 35-second film, portrait/square cuts and loop are rendered on Linux. No publication, deployment, signing or live inference has occurred.

## Master changelog

- Integrated the distribution stack through status/tray and web bridge (PRs #51–#52). Integrated Hallmark and earlier landing work; v1 now owns the site and video. No landing-site commits after `dbcbb1c` are merged.
- Fixed private-helper cleanup and cancellation across Rust, TypeScript and Python. Incompatible shared protocols can fall back; denied authorization remains fatal.
- Hardened broker authentication, negotiation, response bounds and request deadlines. Consolidated bounded Rust grant reads and tightened opened-file checks in Python/TypeScript. Web badges distinguish runtime availability from authenticated access.
- Preserved workspace data on ordinary uninstall, corrected service escaping/socket defaults, and stopped the Windows task before unregistering it. Hardened checksum/extraction failure paths.
- Applied bounded dependency security updates, recorded official-source review, and removed terminal dependencies from nonterminal consumers.
- Added native cost evidence: provider-reported charges where supported, otherwise exact decimal estimates from token counts and dated, sourced local prices. Missing inputs remain unknown. Both apps expose cost trends and Settings price editing; history scope is explicitly runtime-lifetime, not an account-wide bill.
- Reworked web overview around quota rings, reset countdowns and cost evidence. Fixed GPUI mini-view unknown/stale values, platform storage resolution, repeated-history fabrication and unbounded history-file growth. The mini view is in-app; no OS tray is claimed.
- Established generated brand tokens and an OriginLM/package naming rule. Corrected blanket privacy claims: control state is local; authorized provider operations may contact providers.
- Replaced translucent 3D placeholders and packet dots with five opaque screens captured from example-mode UI. Site, offline film and poster share a perspective scene. Lazy scene JavaScript is approximately 145 KB gzip; mobile skips postprocessing. Added a lower CSS-perspective UI stack.
- Rendered the 35-second fframes film, 7-second portrait cut, 9-second square cut and 6-second loop in H.264/VP9. Inspected one-second stills and caption contrast. Large outputs stay ignored; the site stages them explicitly and degrades honestly when absent.
- Replaced unsafe publish-on-tag automation with five-target artifact preparation, matching installer archives, private preview packs and conservative wheel tags. Windows extraction now selects an exact bounded member and honors a configured signer pin. No signing/publication is claimed (ADR-007).

## Remaining work

Finish engineering review/remediation and product/docs polish. Public release preparation still needs a qualified portability baseline and maintainer signing/publication setup. Then run the full verification matrix and replace provisional statements with exact results and honest platform limits. Draft status is intentional.

## Evidence and ownership

Finding ledger: `docs/review/V1-PRINCIPAL-REVIEW.md`. Dependency review: `docs/dependency-audit-2026-10-28.md`. Cost and motion decisions: ADR-005 and ADR-006. New regression tests are authored but not yet executed under the requested verification order.

Thomas owns eventual registry publication credentials, signing identities, domain and hosting. Those are not represented as configured or complete.
