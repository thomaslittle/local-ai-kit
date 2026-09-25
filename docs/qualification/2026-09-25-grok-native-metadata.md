# Grok native metadata qualification — Windows x64

Date: September 25, 2026. This qualification intentionally performed **no model turn/inference**.

- Local AI Kit path: public TypeScript SDK -> native helper -> explicitly registered Grok native profile -> shared ACP client.
- Native client: Grok CLI 1.0.41 (4220f3b224a6), stable channel, Windows x64.
- Authentication: the existing native login was accepted by the Grok ACP session. No token/API key was read or exported.
- Catalog: Grok 4.7 was returned with a provider-native `reasoning_effort` option.
- Plan: not reported and therefore remains unknown.
- Usage/quota: not reported by this ACP path and therefore remains unknown; zero quota pools were invented.
- Billing: the test profile deliberately declared `subscription`; evidence is tagged `user_supplied`, not provider-verified.
- Execution: the same connection-scoped ACP execution driver is fixture-tested for streaming, permissions, cancellation, model/options changes, and cleanup. Live Grok generation was not run because repository policy requires separate authorization to spend inference allowance.

Passive Grok detection remains inert. Native ACP metadata/text support appears only for a host-registered profile and separate metadata/execution grants. Another Grok installation/account is not made executable by this profile.
