# OriginLM product film

Source for the Phase 7 promo, made with [fframes](https://fframes.studio) (Rust + SVG + Skia).
The project is self-contained; all graphics are drawn in code and all type is local. There is
**no audio**: no music or voiceover is shipped, so no licence is implied.

## What is here

| Path | What |
| --- | --- |
| `originlm-promo/` | fframes project (landscape film + loop + social cuts) |
| `originlm-promo/src/lib.rs` | scenes, motion, palette, stack drawing |
| `originlm-promo/media/` | Red Hat Display, Noto Sans Mono, OriginLM white lockup (PNG) |
| `social/` | rendered 9:16 and 1:1 cuts and the 16:9 loop (committed, each < 2 MB) |
| `originlm-promo/out/` | rendered outputs (gitignored) |

## Rendered outputs

| File | Format | Length | Size |
| --- | --- | --- | --- |
| `out/originlm-promo.mp4` | 1920×1080 H.264, silent | 52.6 s | ~5.9 MB |
| `out/originlm-promo.webm` | 1920×1080 VP9, silent | 52.6 s | ~2.7 MB |
| `out/originlm-promo-poster.jpg` | poster, frame at 27 s | — | ~150 KB |
| `out/originlm-loop.mp4` | 1920×1080 H.264, seamless 8 s | 8 s | ~0.8 MB |
| `out/originlm-vertical.mp4` | 1080×1920 H.264, 10.5 s | 10.5 s | ~1.2 MB |
| `out/originlm-square.mp4` | 1080×1080 H.264, 10.5 s | 10.5 s | ~0.85 MB |

The site copy lives in `apps/site/public/media/` (main film, WebM, poster, loop).

## Commands

```bash
cd media/video/originlm-promo
cargo build --release
R=./target/release/originlm-promo

$R timeline
$R inspect
$R strip 2s,16s,27s,38s,51s -n 5
$R frame 27s
$R preview 18s           # requires a working Vulkan/Metal GPU

# main film
$R render -o out/originlm-promo.mp4

# loop and social cuts
$R --format loop render -o out/originlm-loop.mp4
$R --format vertical render -o out/originlm-vertical.mp4
$R --format square render -o out/originlm-square.mp4

# poster and WebM
ffmpeg -ss 27 -i out/originlm-promo.mp4 -frames:v 1 -q:v 2 out/originlm-promo-poster.jpg
ffmpeg -i out/originlm-promo.mp4 -c:v libvpx-vp9 -crf 34 -b:v 0 -row-mt 1 -an out/originlm-promo.webm
```

## Build note (this machine)

The prebuilt ffmpeg that fframes downloads expects **x264 API 163**, while the distro has 165.
OpenSSL-style ABI mismatch breaks the final link. This build used a local x264 at API 163:

```bash
git clone https://github.com/mirror/x264.git /tmp/opencode/x264
cd /tmp/opencode/x264 && git checkout ae03d92b^   # X264_BUILD 163
./configure --prefix=/tmp/opencode/x264-163 --enable-shared --disable-cli --disable-asm
make -j16 && make install
```

Then link and run with the rpath:

```bash
LIBRARY_PATH=/tmp/opencode/x264-163/lib cargo rustc --release --bin originlm-promo -- \
  -L native=/tmp/opencode/x264-163/lib \
  -C link-arg=-Wl,-rpath,/tmp/opencode/x264-163/lib
```

On a machine whose ffmpeg prebuild matches the system x264, plain `cargo build --release`
is enough. The 1080p encode was also rendered in 8 s ranges and concatenated: a full 1578-frame
render exceeded the available memory on this box. See `concat.txt` in `out/`.

## Story order

1. **The problem** — your AI is scattered across six apps.
2. **See your AI** — one local dashboard: providers, quotas, reset times, pace, issues.
3. **Build with it** — the exploded stack and light packets, matching the site hero.
4. **Discover / grant / connect** — provider evidence, a scoped grant, the connected badge.
5. **Install** — the one-line service and the three SDK commands, then `originlm.dev`.

The loop is the exploded stack alone, with periods chosen so frame 0 and frame 240 match
(seamless at 30 fps). Social cuts reuse the same stack and copy in 9:16 and 1:1.

## Provenance

- Tool: `cargo install --locked cargo-fframes --git https://github.com/dmtrKovalenko/fframes`
  (v1.3.0), official skill installed with `npx skills add https://fframes.studio`.
- Fonts: Red Hat Display (OFL, from the desktop demo), Noto Sans Mono (OFL, system).
- Logo: OriginLM asset pack, same provenance note as `apps/demo/src/assets/brand`.
- No stock footage, images, music or voiceover. Everything is procedural SVG.
