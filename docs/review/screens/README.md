# V1 review captures

All captures contain example data. No personal account was inspected or used.

- `web/*-1440.png` and `web/*-390.png`: settled current route captures from the
  example app on 127.0.0.1:5176. `capture.json` records source and browser context.
- `site/1440-collapsed.png` and `1440-exploded.png`: headless software-WebGL behavior
  captures, including the pinned scroll composition. `*-reduced.png` uses the
  static poster; `390-layer-names.png` shows its readable HTML layer names.
- `desktop/overview-costs.png`, `settings.png`, `mini-view.png`: current GPUI app,
  isolated example state, Xvfb/X11. The native window has a desktop minimum size;
  no 390px native/mobile support is claimed.
- `baseline-web/`: early headless v1 captures retained for the visual comparison.
- `baseline-site/`: inherited landing evidence extracted from the already-integrated
  `dbcbb1c` commit, before the principal redesign. These are historical screenshots,
  not fresh execution or evidence for later landing commits.
- `desktop/example-overview*.png`: early v1 native captures, retained as historical
  visual evidence; their exact source commit was not recorded.

Reproduce current captures with `node scripts/capture_v1_review.mjs` after starting
the example app on 5176 and the production site preview on 5177. It waits for lazy
charts, decodes provider masks, and refuses a non-synthetic app banner. Native
captures use `bash scripts/capture_desktop_example.sh` after a desktop build.

The site’s longer composition captures in `apps/site/review/` precede the final
sticky controller; current hero behavior is recorded above and tested by
`scripts/test_site.mjs`. No video qualification is implied by these UI captures.
