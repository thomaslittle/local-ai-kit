# Browser accessibility and plain HTTP consumer qualification

Executed on the Windows development host against the real compiled service and repository packages. No provider inference or personal account metadata was requested by these tests.

## Commands and results

- `node --test scripts/test_accessibility.mjs`: 14 checks passed across all eight showroom variants and six additional pages.
- `node --test scripts/test_component_accessibility.mjs`: eight checks passed against the reusable React selectors in Chromium.
- `python scripts/test_demo.py`: 13 browser interaction tests passed, including the reproduced work-account palette regression.
- `python scripts/test_dotnet.py`: .NET SDK 8.0.425 built the C# example without warnings/errors; the example paired with an isolated native HTTP host, explicitly scanned an empty home, read the normalized live snapshot, and closed its client. A remote HTTPS endpoint was rejected before pairing.

Axe checks use WCAG 2 A/AA and WCAG 2.1 A/AA tags, without rule exclusions. The initial scan failed on selector-tab number contrast; the source style was corrected and every check rerun. These checks do not replace manual screen-reader testing or certify complete WCAG compliance.

The new selector regression failed before exact-phrase ranking was added. Broad partial matches remain searchable, but `Work workspace` ranks the exact account phrase before incidental matches of `work` inside `workspace`. Billing preference and route/account identity remain intact.

The .NET SDK was installed in ignored project verification storage, not added to the user's permanent PATH. Its first invocation emitted the standard development-certificate setup message; subsequent checks suppress SDK certificate generation. No provider configuration or browser profile was changed.

Reference: https://playwright.dev/docs/accessibility-testing
