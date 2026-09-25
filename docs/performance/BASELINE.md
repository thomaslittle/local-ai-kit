# Measured baseline (development build, not final release qualification)

A release-mode Windows x86-64 build from source commit `67a5b15b07677d83fb2847146222a1d031c109fb` plus explicitly marked working-tree changes was measured using ten synthetic local profiles. No provider or inference request was made. Native helper memory excludes the browser and provider processes.

- 20 cold process+scan measurements: p95 3.805 ms.
- 200 cached stdio snapshot reads: p95 0.704 ms.
- 60-second idle sample: peak resident set 7.223 MiB; sampled CPU delta 0% of one core.
- Browser SDK bundle: 5,169 gzip bytes at that source state.

These are measured development values, not guarantees for every host/catalog or a claim about later commits. `scripts/benchmark.py` records the exact binary SHA-256, commit, dirty state, platform, fixture size, sample distribution, and whether targets were met. Re-run it against each new release bundle; do not reuse this baseline as proof of a newer build.
