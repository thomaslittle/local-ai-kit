# Third-party notices and reuse ledger

## Scope

This file records third-party references and notices relevant to this project. It is not a complete inventory of every dependency; release artifacts include a generated dependency inventory and license materials as described below.

The ChatGPT managed-auth integration is original Rust integration code in this repository. Its design used the OpenAI documentation linked from [the ChatGPT managed-auth guide](docs/integration/chatgpt-managed-auth.md). T3 Code's [pull request 14290](https://github.com/pingdotgg/t3code/pull/14290) was consulted as a reference only; no code from that pull request was copied.

Reference does not imply affiliation, endorsement, inherited security guarantees, or blanket rights to use vendor services. Provider names are used for identification. Logos and trademarks are not granted by this repository's license. Provider authentication, subscription access, and endpoint use remain separate integration-policy questions.

## Other reference projects

| Project | Source | Intended reference |
| --- | --- | --- |
| T3 Code | https://github.com/pingdotgg/t3code | Native client integration, provider instances, dynamic model options, picker patterns. |
| CodeNotch cross-platform | https://github.com/thomaslittle/codenotch-crossplatform | Rust/platform discovery and usage parsing. |
| CodeNotch | https://github.com/vinzdg/codenotch | Broader provider/platform discovery. |
| Prmptr | https://github.com/thomaslittle/prmptr | Vertical provider rail and grouped model selection. |

See [SOURCE-AUDIT.md](SOURCE-AUDIT.md) for previously inspected revisions and exact paths. For any future reuse, verify the license at the exact source revision and preserve required copyright and license text.

## New Cargo dependencies in the ChatGPT managed-auth change

The following registry packages and versions were added to `Cargo.lock` in the managed-auth change. The SPDX expressions below are taken from each package's Cargo manifest metadata.

| Package | Version | SPDX license expression |
| --- | --- | --- |
| `byteorder` | `1.5.0` | `Unlicense OR MIT` |
| `deranged` | `0.5.8` | `MIT OR Apache-2.0` |
| `fs4` | `0.13.1` | `MIT OR Apache-2.0` |
| `jsonwebtoken` | `9.3.1` | `MIT` |
| `keyring` | `3.6.3` | `MIT OR Apache-2.0` |
| `num-conv` | `0.2.2` | `MIT OR Apache-2.0` |
| `pem` | `3.0.6` | `MIT` |
| `powerfmt` | `0.2.0` | `MIT OR Apache-2.0` |
| `simple_asn1` | `0.6.4` | `ISC` |
| `time` | `0.3.55` | `MIT OR Apache-2.0` |
| `time-core` | `0.1.9` | `MIT OR Apache-2.0` |
| `time-macros` | `0.2.32` | `MIT OR Apache-2.0` |
| `uuid` | `1.26.1` | `Apache-2.0 OR MIT` |
| `windows-sys` | `0.59.0` | `MIT OR Apache-2.0` |

## Release artifact license materials

Standalone Windows helper releases include a generated Cargo dependency license appendix with the helper's `THIRD_PARTY_NOTICES.md`, and a CycloneDX SBOM. The full SDK bundle also carries per-crate license files under `licenses/`, alongside its SBOM. These materials inventory the dependencies represented by that build; they do not certify legal compliance or cover unrelated code provenance.

## Required ledger for future reuse

Record the source URL, exact commit/blob, original path, copied or adapted symbols, destination path, license/SPDX identifier, copyright holder, retained notices, modifications, and applicable asset/model/dependency conditions. Add required license text under the release's dependency license materials when applicable. Do not mark the ledger complete merely because the root repository is MIT-licensed.
