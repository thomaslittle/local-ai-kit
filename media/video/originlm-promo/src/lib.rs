//! OriginLM product film. The story is split in half, like the product:
//! know your AI usage, then build on the same runtime.
//!
//! Everything is drawn procedurally on a 1920x1080 grid: no stock footage, no
//! stock imagery, no music. See `README.md` for the render commands.

use fframes::{
    AudioMap, Color, Duration, FFramesContext, Frame, Overlap, Scene, Scenes, Svgr, Video,
    include_media_dir,
};

include_media_dir!(pub struct OriginlmPromoMedia, "media");

pub const WIDTH: usize = 1920;
pub const HEIGHT: usize = 1080;
pub const TOTAL_SECONDS: f32 = 52.0;

const BG: &str = "#171314";
const PANEL: &str = "#221d1e";
const PANEL_2: &str = "#2b2526";
const TEXT: &str = "#faf9f5";
const MUTED: &str = "#b0a8a1";
const FAINT: &str = "#8a817b";
const AMBER: &str = "#eba31a";
const AMBER_BRIGHT: &str = "#f7bd4b";
const SAGE: &str = "#a9b29f";
const GREEN: &str = "#7cc08f";
const DANGER: &str = "#e0776b";

const SANS: &str = "Red Hat Display";
const MONO: &str = "Noto Sans Mono";

/// 0..1 ease-out for entrances.
fn ease_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

/// 0..1 ease-in-out for structural moves.
fn ease_in_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 { 4.0 * t * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }
}

/// Slight overshoot, settles by t=1.
fn spring(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t >= 1.0 {
        1.0
    } else {
        1.0 - (-5.5 * t).exp() * ((9.0 * t).cos() + 0.55 * (9.0 * t).sin())
    }
}

/// Progress of an entrance that starts at `start` and lasts `duration`.
fn enter(seconds: f32, start: f32, duration: f32) -> f32 {
    ease_out((seconds - start) / duration)
}

fn spring_at(seconds: f32, start: f32, duration: f32) -> f32 {
    spring((seconds - start) / duration)
}

fn clamp01(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

/// Shared static background: graphite, a faint grid, one warm glow.
fn backdrop() -> Svgr<'static> {
    fframes::svgr!(
        <g>
            <defs>
                <radialGradient id="lm-glow" cx="0.72" cy="0.3" r="0.62">
                    <stop offset="0" stop-color={AMBER} stop-opacity="0.16" />
                    <stop offset="1" stop-color={AMBER} stop-opacity="0" />
                </radialGradient>
                <radialGradient id="lm-glow2" cx="0.1" cy="0.9" r="0.55">
                    <stop offset="0" stop-color={SAGE} stop-opacity="0.09" />
                    <stop offset="1" stop-color={SAGE} stop-opacity="0" />
                </radialGradient>
                <pattern id="lm-grid" width="64" height="64" patternUnits="userSpaceOnUse">
                    <path d="M64 0H0V64" fill="none" stroke={TEXT} stroke-opacity="0.045" />
                </pattern>
                <linearGradient id="lm-panel" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0" stop-color="#faf9f5" stop-opacity="0.09" />
                    <stop offset="1" stop-color="#faf9f5" stop-opacity="0.025" />
                </linearGradient>
            </defs>
            <rect width="1920" height="1080" fill={BG} />
            <rect width="1920" height="1080" fill="url(#lm-grid)" />
            <rect width="1920" height="1080" fill="url(#lm-glow)" />
            <rect width="1920" height="1080" fill="url(#lm-glow2)" />
        </g>
    )
}

fn scene_label<'a>(number: &'a str, label: &'a str) -> Svgr<'a> {
    fframes::svgr!(
        <g font-family={SANS} font-weight="600">
            <rect x="120" y="204" width="26" height="3" rx="1.5" fill={AMBER} />
            <text x="162" y="212" font-size="26" letter-spacing="4" fill={AMBER}>{label}</text>
            <text x="1800" y="212" text-anchor="end" font-size="24" letter-spacing="2" fill={FAINT}>{number}</text>
        </g>
    )
}

// ---------------------------------------------------------------------------
// Scene 1 — the problem
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct ProblemScene;

impl Scene for ProblemScene {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(6.4)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let exit = 1.0 - ease_in_out((t - 6.0) / 0.4);
        let fade = enter(t, 0.0, 0.3).min(exit);
        let cards = [
            ("Claude Code", "subscription", 1040.0, 268.0, AMBER),
            ("Codex", "subscription", 1392.0, 330.0, AMBER_BRIGHT),
            ("Cursor", "signed in", 1108.0, 430.0, SAGE),
            ("Grok", "local profile", 1446.0, 492.0, "#c97e5a"),
            ("Ollama", "7 local models", 1052.0, 592.0, SAGE),
            ("Gemini CLI", "metadata only", 1388.0, 654.0, AMBER),
        ];

        let card_svgs: Vec<Svgr> = cards
            .iter()
            .enumerate()
            .map(|(index, (name, status, x, y, tone))| {
                let start = 0.55 + index as f32 * 0.14;
                let p = spring_at(t, start, 0.9);
                let drift = ((t * 0.9 + index as f32) .sin()) * 4.0;
                let gather = enter(t, 3.9, 1.4);
                let target_x = 1010.0 + (index % 2) as f32 * 54.0;
                let target_y = 300.0 + index as f32 * 62.0;
                let cx = x + (target_x - x) * gather * 0.22;
                let cy = y + (target_y - y) * gather * 0.22 + drift;
                let opacity = clamp01(p) * exit;
                fframes::svgr!(
                    <g opacity={opacity} transform={format!("translate({:.1} {:.1}) scale({:.3})", cx, cy, 0.92 + p * 0.08)}>
                        <rect width="330" height="94" rx="18" fill={PANEL_2} fill-opacity="0.92" stroke="#faf9f5" stroke-opacity="0.1" />
                        <rect x="22" y="24" width="46" height="46" rx="12" fill={tone} fill-opacity="0.16" stroke={tone} stroke-opacity="0.45" />
                        <circle cx="45" cy="47" r="8" fill={tone} />
                        <text x="86" y="42" font-family={SANS} font-weight="600" font-size="28" fill={TEXT}>{*name}</text>
                        <text x="86" y="72" font-family={SANS} font-weight="500" font-size="21" fill={MUTED}>{*status}</text>
                    </g>
                )
            })
            .collect();

        let target = enter(t, 2.6, 1.4);

        fframes::svgr!(
            <g opacity={fade}>
                {scene_label("01 / 05", "THE PROBLEM")}
                <text x="120" y="356" font-family={SANS} font-weight="600" font-size="92" letter-spacing="-3" fill={TEXT}>"Your AI is scattered"</text>
                <text x="120" y="462" font-family={SANS} font-weight="600" font-size="92" letter-spacing="-3" fill={AMBER}>"across six apps."</text>
                <g opacity={enter(t, 0.35, 0.7)}>
                    <text x="122" y="556" font-family={SANS} font-weight="500" font-size="34" fill={MUTED}>"Usage, quotas, models and routes"</text>
                    <text x="122" y="602" font-family={SANS} font-weight="500" font-size="34" fill={MUTED}>"all live in different places."</text>
                </g>
                <g opacity={target * 0.75}>
                    <rect x="960" y="222" width="740" height="610" rx="34" fill={AMBER} fill-opacity="0.035" stroke={AMBER} stroke-opacity="0.22" stroke-dasharray="7 9" />
                    <text x="1000" y="804" font-family={SANS} font-weight="600" font-size="26" letter-spacing="3" fill={AMBER}>"ONE PLACE"</text>
                </g>
                {card_svgs}
                <g opacity={enter(t, 3.2, 0.7)}>
                    <path d="M120 746 H740" stroke="#faf9f5" stroke-opacity="0.12" stroke-width="2" />
                    <text x="120" y="800" font-family={SANS} font-weight="500" font-size="30" fill={MUTED}>"Every client keeps its own small dashboard."</text>
                </g>
            </g>
        )
    }
}

// ---------------------------------------------------------------------------
// Scene 2 — the dashboard
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct DashboardScene;

fn quota_row<'a>(seconds: f32, index: usize, name: &'a str, percent: f32, reset: &'a str, warn: bool) -> Svgr<'a> {
    let start = 1.2 + index as f32 * 0.13;
    let p = enter(seconds, start, 0.8);
    let y = 450.0 + index as f32 * 44.0;
    let tone = if warn { "#e0a184" } else { AMBER };
    let value = percent * ease_in_out((seconds - start) / 1.3);
    fframes::svgr!(
        <g opacity={p} transform={format!("translate(0 {:.1})", (1.0 - p) * 16.0)}>
            <text x="452" y={y + 22.0} font-family={SANS} font-weight="600" font-size="27" fill={TEXT}>{name}</text>
            <rect x="686" y={y + 8.0} width="620" height="10" rx="5" fill="#faf9f5" fill-opacity="0.08" />
            <rect x="686" y={y + 8.0} width={(value / 100.0 * 620.0).max(0.5)} height="10" rx="5" fill={tone} />
            <text x="1348" y={y + 22.0} text-anchor="end" font-family={SANS} font-weight="600" font-size="26" fill={if warn { "#e0a184" } else { TEXT }}>{format!("{:.0}%", value)}</text>
            <text x="1420" y={y + 22.0} font-family={SANS} font-weight="500" font-size="23" fill={MUTED}>{reset}</text>
        </g>
    )
}

impl Scene for DashboardScene {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(12.0)
    }

    fn overlap(&self) -> Overlap {
        Overlap::Previous(0.35)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let exit = 1.0 - ease_in_out((t - 11.65) / 0.35);
        let fade = enter(t, 0.0, 0.3).min(exit);
        let window_in = spring_at(t, 0.15, 1.1);
        let metrics = enter(t, 1.1, 0.7);
        let highlight = enter(t, 5.6, 0.8) * (1.0 - enter(t, 9.4, 0.6) * 0.0);

        let bars = [0.38, 0.55, 0.42, 0.68, 0.61, 0.82, 0.74, 0.96, 0.63, 0.78, 0.88, 0.71];
        let bar_svgs: Vec<Svgr> = bars
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let p = enter(t, 1.5 + index as f32 * 0.045, 0.55);
                let height = (value * 120.0 * p).max(0.5);
                let tone = if index > 8 { AMBER } else { "#6b5a4c" };
                fframes::svgr!(
                    <rect x={452.0 + index as f32 * 64.0} y={410.0 - height} width="38" height={height} rx="6" fill={tone} />
                )
            })
            .collect();

        fframes::svgr!(
            <g opacity={fade}>
                {scene_label("02 / 05", "SEE YOUR AI")}
                <text x="120" y="300" font-family={SANS} font-weight="600" font-size="64" letter-spacing="-2" fill={TEXT}>"One local view. Every provider."</text>
                <g transform={format!("translate({:.1} {:.1})", (1.0 - window_in) * 40.0, 350.0 + (1.0 - window_in) * 60.0)} opacity={clamp01(window_in)}>
                    <rect x="120" y="0" width="1680" height="630" rx="26" fill={PANEL} stroke="#faf9f5" stroke-opacity="0.1" />
                    <rect x="120" y="0" width="1680" height="58" rx="26" fill="#2b2526" />
                    <rect x="120" y="30" width="1680" height="28" fill="#2b2526" />
                    <circle cx="160" cy="30" r="7" fill={AMBER} />
                    <circle cx="186" cy="30" r="7" fill="#6f625c" />
                    <circle cx="212" cy="30" r="7" fill="#6f625c" />
                    <text x="260" y="38" font-family={MONO} font-size="19" fill={MUTED}>"OriginLM · Control Center"</text>
                    <text x="1750" y="38" text-anchor="end" font-family={MONO} font-size="17" fill={GREEN}>"LOCAL ONLY"</text>

                    <rect x="120" y="58" width="250" height="572" fill="#1b1718" fill-opacity="0.6" />
                    <g font-family={SANS} font-weight="500" font-size="23">
                        <rect x="150" y="98" width="190" height="42" rx="9" fill={AMBER} fill-opacity="0.14" />
                        <text x="172" y="126" fill={AMBER}>"Usage"</text>
                        <text x="172" y="180" fill={MUTED}>"Models"</text>
                        <text x="172" y="228" fill={MUTED}>"Routes"</text>
                        <text x="172" y="276" fill={MUTED}>"Issues"</text>
                        <text x="172" y="324" fill={MUTED}>"Apps"</text>
                    </g>
                    <path d="M370 58 V630" stroke="#faf9f5" stroke-opacity="0.08" />

                    <text x="410" y="112" font-family={SANS} font-weight="600" font-size="38" fill={TEXT}>"Today"</text>
                    <text x="410" y="146" font-family={SANS} font-weight="500" font-size="21" fill={FAINT}>"3 providers · 41 models · observed just now"</text>

                    <g opacity={metrics}>
                        <rect x="410" y="174" width="330" height="118" rx="14" fill="#faf9f5" fill-opacity="0.035" stroke="#faf9f5" stroke-opacity="0.08" />
                        <text x="434" y="206" font-family={SANS} font-weight="500" font-size="19" letter-spacing="1" fill={FAINT}>"CONNECTED"</text>
                        <text x="434" y="258" font-family={SANS} font-weight="600" font-size="42" fill={TEXT}>"3"</text>
                        <text x="500" y="258" font-family={SANS} font-weight="500" font-size="20" fill={DANGER}>"1 signed out"</text>

                        <rect x="760" y="174" width="330" height="118" rx="14" fill="#faf9f5" fill-opacity="0.035" stroke="#faf9f5" stroke-opacity="0.08" />
                        <text x="784" y="206" font-family={SANS} font-weight="500" font-size="19" letter-spacing="1" fill={FAINT}>"TURNS TODAY"</text>
                        <text x="784" y="258" font-family={SANS} font-weight="600" font-size="42" fill={TEXT}>"128"</text>
                        <text x="884" y="258" font-family={SANS} font-weight="500" font-size="20" fill={GREEN}>"+14% vs avg"</text>

                        <rect x="1110" y="174" width="330" height="118" rx="14" fill="#faf9f5" fill-opacity="0.035" stroke="#faf9f5" stroke-opacity="0.08" />
                        <text x="1134" y="206" font-family={SANS} font-weight="500" font-size="19" letter-spacing="1" fill={FAINT}>"METERED"</text>
                        <text x="1134" y="258" font-family={SANS} font-weight="600" font-size="42" fill={TEXT}>"$0.00"</text>
                        <text x="1264" y="258" font-family={SANS} font-weight="500" font-size="20" fill={MUTED}>"subscription"</text>
                    </g>

                    <g>
                        <rect x="410" y="290" width="1030" height="140" rx="14" fill="#faf9f5" fill-opacity="0.02" stroke="#faf9f5" stroke-opacity="0.06" />
                        
                        {bar_svgs}
                    </g>

                    {quota_row(t, 0, "Claude Code", 62.0, "resets in 3h 12m", false)}
                    {quota_row(t, 1, "Codex", 84.0, "resets in 41m", true)}
                    {quota_row(t, 2, "Cursor", 41.0, "monthly window", false)}
                    {quota_row(t, 3, "Ollama", 18.0, "7 models · local", false)}

                    <g opacity={highlight * 0.9}>
                        <rect x="410" y="483" width="1030" height="44" rx="10" fill={AMBER} fill-opacity="0.06" stroke={AMBER} stroke-opacity="0.32" />
                    </g>
                </g>
                <g opacity={enter(t, 8.8, 0.7)}>
                    <text x="120" y="1010" font-family={SANS} font-weight="500" font-size="28" fill={MUTED}>"Read from the clients already on your machine. No relay, no account."</text>
                </g>
            </g>
        )
    }
}

// ---------------------------------------------------------------------------
// Scene 3 — the exploded stack
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct StackScene;

struct LayerSpec {
    name: &'static str,
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    scale_x: f32,
}

fn stack_layer<'a>(seconds: f32, spec: &LayerSpec, index: usize, explode: f32, labels: bool) -> Svgr<'a> {
    let base_y = 548.0;
    let y = base_y + (spec.y - base_y) * explode;
    let start_x = 760.0 - spec.width * 0.5;
    let final_x = spec.x - spec.width * 0.5;
    let x = start_x + (final_x - start_x) * explode;
    let breathe = (seconds * 1.1 + index as f32 * 0.7).sin() * 2.5 * explode;
    let opacity = clamp01(0.35 + explode * 0.65);
    let ui = enter(seconds, 1.4 + index as f32 * 0.12, 0.6);
    let label_opacity = if labels { clamp01((explode - 0.55) / 0.4) } else { 0.0 };
    let label_y = y + spec.height * 0.5;

    fframes::svgr!(
        <g>
            <g opacity={opacity} transform={format!("translate({:.1} {:.1}) scale({:.3} 1)", x, y + breathe, spec.scale_x)}>
                <rect width={spec.width} height={spec.height} rx="16" fill="url(#lm-panel)" stroke="#c9b6ab" stroke-opacity={0.22 + explode * 0.12} />
                <rect x="22" y={spec.height - 12.0} width={spec.width - 44.0} height="2" rx="1" fill={AMBER} fill-opacity={0.2 + explode * 0.18} />
                {if index == 0 {
                    fframes::svgr!(<g opacity={ui}>
                        <circle cx="26" cy="24" r="4.5" fill={AMBER} />
                        <circle cx="42" cy="24" r="4.5" fill="#6f625c" />
                        <circle cx="58" cy="24" r="4.5" fill="#6f625c" />
                        <rect x="26" y="52" width="190" height="8" rx="4" fill={TEXT} fill-opacity="0.4" />
                        <rect x="26" y="72" width="300" height="8" rx="4" fill={TEXT} fill-opacity="0.16" />
                        <text x="26" y={spec.height - 28.0} font-family={SANS} font-weight="600" font-size="25" fill={TEXT}>"Your app"</text>
                        <rect x="430" y="18" width="266" height="94" rx="10" fill="#faf9f5" fill-opacity="0.04" stroke="#faf9f5" stroke-opacity="0.09" />
                        <text x="448" y="42" font-family={SANS} font-weight="600" font-size="17" fill={TEXT}>"Usage"</text>
                        <rect x="448" y="54" width="96" height="5" rx="2.5" fill={AMBER} fill-opacity="0.6" />
                        <rect x="448" y="66" width="150" height="5" rx="2.5" fill={TEXT} fill-opacity="0.18" />
                        <rect x="448" y="82" width="34" height="18" rx="3" fill={AMBER} fill-opacity="0.55" />
                        <rect x="490" y="74" width="34" height="26" rx="3" fill={SAGE} fill-opacity="0.5" />
                        <rect x="532" y="68" width="34" height="32" rx="3" fill={AMBER} fill-opacity="0.7" />
                        <rect x="574" y="78" width="34" height="22" rx="3" fill={SAGE} fill-opacity="0.4" />
                    </g>)
                } else {
                    fframes::svgr!(<g opacity={ui}>
                        <rect x="26" y={spec.height * 0.5 - 22.0} width={spec.width * 0.24} height="6" rx="3" fill={TEXT} fill-opacity="0.35" />
                        <rect x="26" y={spec.height * 0.5 - 2.0} width={spec.width * 0.34} height="6" rx="3" fill={TEXT} fill-opacity="0.16" />
                        <rect x="26" y={spec.height * 0.5 + 18.0} width={spec.width * 0.18} height="6" rx="3" fill={TEXT} fill-opacity="0.12" />
                    </g>)
                }}
            </g>
            {if labels {
                fframes::svgr!(<g opacity={label_opacity}>
                    <path d={format!("M{:.1} {:.1} H1240", x + spec.width * spec.scale_x + 18.0, label_y)} stroke={AMBER} stroke-opacity="0.3" stroke-width="1.5" />
                    <circle cx={x + spec.width * spec.scale_x + 18.0} cy={label_y} r="5" fill={AMBER} fill-opacity="0.8" />
                    <text x="1262" y={label_y + 8.0} font-family={SANS} font-weight="500" font-size="25" fill={MUTED}>{spec.name}</text>
                </g>)
            } else { Svgr::empty() }}
        </g>
    )
}

impl Scene for StackScene {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(11.6)
    }

    fn overlap(&self) -> Overlap {
        Overlap::Previous(0.35)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let exit = 1.0 - ease_in_out((t - 11.25) / 0.35);
        let fade = enter(t, 0.0, 0.3).min(exit);
        let explode = 0.18 + 0.82 * ease_in_out((t - 0.15) / 2.3);
        let toward = enter(t, 6.4, 0.8);

        let layers = [
            LayerSpec { name: "Your app + dashboard", width: 720.0, height: 130.0, x: 760.0, y: 330.0, scale_x: 1.0 },
            LayerSpec { name: "SDK · Rust, npm, Python", width: 660.0, height: 90.0, x: 760.0, y: 500.0, scale_x: 1.0 },
            LayerSpec { name: "Transport · stdio, IPC, native messaging", width: 700.0, height: 76.0, x: 760.0, y: 610.0, scale_x: 1.0 },
            LayerSpec { name: "Runtime · grants, discovery, sessions", width: 744.0, height: 96.0, x: 760.0, y: 715.0, scale_x: 1.0 },
            LayerSpec { name: "Adapters · one identity model", width: 788.0, height: 74.0, x: 760.0, y: 840.0, scale_x: 1.0 },
            LayerSpec { name: "Providers already installed", width: 832.0, height: 100.0, x: 760.0, y: 930.0, scale_x: 1.0 },
        ];

        let layer_svgs: Vec<Svgr> = layers
            .iter()
            .enumerate()
            .map(|(index, spec)| stack_layer(t, spec, index, explode, true))
            .collect();

        let packet_count = 12;
        let packets: Vec<Svgr> = (0..packet_count)
            .map(|index| {
                let phase = (t * 0.32 + index as f32 / packet_count as f32) % 1.0;
                let travel = 1.0 - (phase * 2.0 - 1.0).abs();
                let y = 300.0 + travel * 700.0;
                let opacity = clamp01((explode - 0.3) / 0.4) * (0.25 + 0.6 * (phase * std::f32::consts::PI).sin());
                let x = 760.0 + (index as f32 * 2.7).sin() * 110.0;
                fframes::svgr!(<circle cx={x} cy={y} r={3.0 + (index % 3) as f32} fill={if index % 3 == 0 { SAGE } else { AMBER_BRIGHT }} fill-opacity={opacity} />)
            })
            .collect();

        let providers = ["Claude", "Codex", "Ollama", "Cursor", "Grok", "…"];
        let provider_nodes: Vec<Svgr> = providers
            .iter()
            .enumerate()
            .map(|(index, name)| {
                let x = 392.0 + index as f32 * 138.0;
                let tone = if index % 2 == 0 { AMBER } else { SAGE };
                let pulse = 1.0 + (t * 2.0 + index as f32).sin() * 0.08;
                fframes::svgr!(<g>
                    <circle cx={x} cy="978" r={14.0 * pulse} fill={tone} fill-opacity="0.16" />
                    <circle cx={x} cy="978" r={5.5 * pulse} fill={tone} />
                    <text x={x} y="1014" text-anchor="middle" font-family={SANS} font-weight="500" font-size="19" fill={MUTED}>{*name}</text>
                </g>)
            })
            .collect();

        fframes::svgr!(
            <g opacity={fade}>
                {scene_label("03 / 05", "BUILD WITH IT")}
                <text x="120" y="300" font-family={SANS} font-weight="600" font-size="58" letter-spacing="-2" fill={TEXT}>"The same runtime you just saw."</text>
                <g opacity={enter(t, 0.35, 0.6)}>
                    <text x="1800" y="286" text-anchor="end" font-family={SANS} font-weight="500" font-size="24" fill={FAINT}>"same runtime"</text>
                    <text x="1800" y="318" text-anchor="end" font-family={SANS} font-weight="500" font-size="24" fill={FAINT}>"two surfaces"</text>
                </g>
                {layer_svgs}
                {provider_nodes}
                {packets}
                <g opacity={toward} transform={format!("translate(0 {:.1})", (1.0 - toward) * 20.0)}>
                    <text x="108" y="430" font-family={SANS} font-weight="600" font-size="34" letter-spacing="-1" fill={TEXT}>"One dependency."</text>
                    <text x="120" y="472" font-family={SANS} font-weight="500" font-size="25" fill={MUTED}>"Install it once."</text>
                    <text x="120" y="506" font-family={SANS} font-weight="500" font-size="25" fill={MUTED}>"Every app gets it."</text>
                </g>
                <g opacity={enter(t, 9.0, 0.7)}>
                    <path d="M120 1006 H1800" stroke="#faf9f5" stroke-opacity="0.1" />
                    <text x="120" y="1002" font-family={SANS} font-weight="500" font-size="26" fill={FAINT}>"Light packets: the same request path, down to the providers and back up."</text>
                </g>
            </g>
        )
    }
}

// ---------------------------------------------------------------------------
// Scene 4 — discover, grant, connect
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct ConnectScene;

impl Scene for ConnectScene {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(11.4)
    }

    fn overlap(&self) -> Overlap {
        Overlap::Previous(0.35)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let exit = 1.0 - ease_in_out((t - 11.05) / 0.35);
        let fade = enter(t, 0.0, 0.3).min(exit);
        let badge = spring_at(t, 4.4, 1.1);

        let rows = [
            ("Claude Code", "Authenticated · plan reported", "connected", GREEN),
            ("Codex", "Authenticated · native catalog", "connected", GREEN),
            ("Ollama", "7 models · local endpoint", "local", SAGE),
            ("Cursor", "Signed out · cached metadata", "attention", AMBER),
        ];

        let row_svgs: Vec<Svgr> = rows
            .iter()
            .enumerate()
            .map(|(index, (name, detail, state, tone))| {
                let p = enter(t, 0.5 + index as f32 * 0.24, 0.65);
                let y = 372.0 + index as f32 * 112.0;
                fframes::svgr!(
                    <g opacity={p} transform={format!("translate(0 {:.1})", (1.0 - p) * 18.0)}>
                        <rect x="150" y={y} width="700" height="92" rx="16" fill={PANEL_2} fill-opacity="0.85" stroke="#faf9f5" stroke-opacity="0.08" />
                        <rect x="176" y={y + 22.0} width="48" height="48" rx="12" fill={tone} fill-opacity="0.15" />
                        <circle cx="200" cy={y + 46.0} r="9" fill={tone} />
                        <text x="248" y={y + 40.0} font-family={SANS} font-weight="600" font-size="28" fill={TEXT}>{*name}</text>
                        <text x="248" y={y + 70.0} font-family={SANS} font-weight="500" font-size="21" fill={MUTED}>{*detail}</text>
                        <rect x="652" y={y + 30.0} width="168" height="34" rx="17" fill={tone} fill-opacity="0.1" stroke={tone} stroke-opacity="0.28" />
                        <text x="736" y={y + 53.0} text-anchor="middle" font-family={SANS} font-weight="600" font-size="18" fill={tone}>{*state}</text>
                    </g>
                )
            })
            .collect();

        let grants = ["Usage and quota snapshot", "Model catalog", "Run sessions"];
        let grant_rows: Vec<Svgr> = grants
            .iter()
            .enumerate()
            .map(|(index, label)| {
                let on = enter(t, 2.2 + index as f32 * 0.35, 0.55);
                let y = 470.0 + index as f32 * 56.0;
                fframes::svgr!(<g>
                    <text x="1010" y={y} font-family={SANS} font-weight="500" font-size="24" fill={MUTED}>{*label}</text>
                    <rect x="1360" y={y - 18.0} width="62" height="28" rx="14" fill="#faf9f5" fill-opacity={0.07 + on * 0.03} />
                    <circle cx={1372.0 + on * 36.0} cy={y - 4.0} r="10" fill={if on > 0.55 { AMBER } else { FAINT }} />
                </g>)
            })
            .collect();

        fframes::svgr!(
            <g opacity={fade}>
                {scene_label("04 / 05", "CONNECT")}
                <text x="120" y="290" font-family={SANS} font-weight="600" font-size="72" letter-spacing="-2" fill={TEXT}>"Discover. Grant. Connect."</text>

                <g opacity={enter(t, 0.3, 0.5)}>
                    <text x="150" y="340" font-family={SANS} font-weight="500" font-size="22" letter-spacing="2" fill={FAINT}>"ON THIS MACHINE"</text>
                </g>
                {row_svgs}

                <g opacity={enter(t, 1.1, 0.7)} transform={format!("translate(0 {:.1})", (1.0 - enter(t, 1.1, 0.7)) * 22.0)}>
                    <rect x="940" y="330" width="720" height="290" rx="22" fill={PANEL} fill-opacity="0.86" stroke="#faf9f5" stroke-opacity="0.1" />
                    <text x="1010" y="390" font-family={SANS} font-weight="600" font-size="30" fill={TEXT}>"Acme Studio"</text>
                    <text x="1010" y="414" font-family={SANS} font-weight="500" font-size="20" fill={FAINT}>"requests access · 127.0.0.1"</text>
                    {grant_rows}
                </g>

                <g opacity={clamp01(badge)} transform={format!("translate(0 {:.1}) scale({:.3})", (1.0 - badge) * 26.0, 0.94 + badge * 0.06)}>
                    <rect x="988" y="660" width="676" height="92" rx="46" fill={PANEL_2} stroke="#faf9f5" stroke-opacity="0.16" />
                    <circle cx="1030" cy="706" r="11" fill={GREEN} />
                    <circle cx="1030" cy="706" r={18.0 + (t * 2.4).sin().abs() * 10.0} fill="none" stroke={GREEN} stroke-opacity={0.25 - (t * 2.4).sin().abs() * 0.18} />
                    <text x="1062" y="698" font-family={SANS} font-weight="600" font-size="27" fill={TEXT}>"Local AI connected"</text>
                    <text x="1062" y="728" font-family={SANS} font-weight="500" font-size="21" fill={MUTED}>"Claude, Codex, Ollama"</text>
                    <rect x="1508" y="686" width="126" height="40" rx="20" fill={GREEN} fill-opacity="0.12" />
                    <text x="1571" y="712" text-anchor="middle" font-family={SANS} font-weight="600" font-size="17" fill={GREEN}>"CONNECTED"</text>
                </g>

                <g opacity={enter(t, 5.8, 0.7)}>
                    <path d="M120 1006 H1800" stroke="#faf9f5" stroke-opacity="0.1" />
                    <text x="120" y="1002" font-family={SANS} font-weight="500" font-size="27" fill={MUTED}>"Discovery is not permission. Every app gets its own scoped, revocable grant."</text>
                </g>
            </g>
        )
    }
}

// ---------------------------------------------------------------------------
// Scene 5 — install
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct InstallScene;

fn typed(text: &str, seconds: f32, start: f32, duration: f32) -> String {
    let count = ((seconds - start) / duration).clamp(0.0, 1.0) * text.chars().count() as f32;
    text.chars().take(count as usize).collect()
}

impl Scene for InstallScene {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(11.2)
    }

    fn overlap(&self) -> Overlap {
        Overlap::Previous(0.35)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let exit = enter(t, 0.0, 0.3).min(1.0);
        let left = enter(t, 0.35, 0.8);
        let right = enter(t, 0.6, 0.8);
        let commands = [
            ("npm", "npm install @local-ai-kit/sdk", 2.0),
            ("cargo", "cargo add local-ai-kit", 3.2),
            ("pip", "pip install local-ai-kit-preview", 4.4),
        ];

        let cmd_svgs: Vec<Svgr> = commands
            .iter()
            .enumerate()
            .map(|(index, (tool, command, start))| {
                let y = 552.0 + index as f32 * 96.0;
                let text = typed(command, t, *start, 0.9);
                fframes::svgr!(<g opacity={enter(t, *start - 0.2, 0.4)}>
                    <rect x="1000" y={y - 42.0} width="740" height="76" rx="12" fill="#141112" fill-opacity="0.8" stroke="#faf9f5" stroke-opacity="0.07" />
                    <text x="1022" y={y + 6.0} font-family={SANS} font-weight="600" font-size="18" letter-spacing="1" fill={FAINT}>{format!("{tool}")}</text>
                    <text x="1104" y={y + 6.0} font-family={MONO} font-size="23" fill={TEXT}>{text}</text>
                </g>)
            })
            .collect();

        fframes::svgr!(
            <g opacity={exit}>
                {scene_label("05 / 05", "INSTALL")}
                <text x="120" y="298" font-family={SANS} font-weight="600" font-size="70" letter-spacing="-2" fill={TEXT}>"Install once. Every app gets your AI."</text>

                <g opacity={left} transform={format!("translate(0 {:.1})", (1.0 - left) * 24.0)}>
                    <rect x="120" y="390" width="760" height="430" rx="24" fill={PANEL} fill-opacity="0.85" stroke="#faf9f5" stroke-opacity="0.1" />
                    <text x="156" y="444" font-family={SANS} font-weight="600" font-size="32" fill={TEXT}>"Get the app"</text>
                    <text x="156" y="476" font-family={SANS} font-weight="500" font-size="21" fill={MUTED}>"The control center, the tray, and the service every SDK finds."</text>
                    <rect x="156" y="514" width="688" height="92" rx="14" fill="#141112" fill-opacity="0.85" stroke="#faf9f5" stroke-opacity="0.07" />
                    <text x="180" y="556" font-family={MONO} font-size="19" fill={AMBER}>"$"</text>
                    <text x="206" y="556" font-family={MONO} font-size="21" fill={TEXT}>{typed("curl -fsSL https://originlm.dev/install.sh | sh", t, 0.9, 1.2)}</text>
                    <text x="206" y="586" font-family={MONO} font-size="18" fill={MUTED}>{typed("irm https://originlm.dev/install.ps1 | iex   # Windows", t, 2.4, 1.2)}</text>
                    <text x="156" y="654" font-family={SANS} font-weight="500" font-size="22" fill={MUTED}>"A user service. No admin rights."</text>
                    <text x="156" y="686" font-family={SANS} font-weight="500" font-size="22" fill={MUTED}>"launchd · systemd --user · Task Scheduler"</text>
                    <text x="156" y="742" font-family={SANS} font-weight="600" font-size="20" letter-spacing="1" fill={GREEN}>"NO TELEMETRY · LOOPBACK ONLY"</text>
                    <text x="156" y="784" font-family={SANS} font-weight="500" font-size="20" fill={FAINT}>"Uninstall removes the unit, binary and state."</text>
                </g>

                <g opacity={right} transform={format!("translate(0 {:.1})", (1.0 - right) * 24.0)}>
                    <rect x="920" y="390" width="880" height="430" rx="24" fill={PANEL} fill-opacity="0.85" stroke="#faf9f5" stroke-opacity="0.1" />
                    <text x="956" y="444" font-family={SANS} font-weight="600" font-size="32" fill={TEXT}>"Add to your project"</text>
                    <text x="956" y="476" font-family={SANS} font-weight="500" font-size="21" fill={MUTED}>"The SDK speaks to the service, or bundles the helper itself."</text>
                    {cmd_svgs}
                    <text x="956" y="802" font-family={SANS} font-weight="500" font-size="20" fill={MUTED}>"Rust, npm and Python · pre-release packages"</text>
                </g>

                <g opacity={enter(t, 7.2, 0.8)}>
                    <path d="M120 906 H1800" stroke="#faf9f5" stroke-opacity="0.1" />
                    <text x="960" y="980" text-anchor="middle" font-family={SANS} font-weight="600" font-size="44" letter-spacing="-1" fill={TEXT}>"See and use all your AI. One dependency for every app."</text>
                    <text x="960" y="1028" text-anchor="middle" font-family={SANS} font-weight="500" font-size="27" fill={AMBER}>"originlm.dev"</text>
                </g>
            </g>
        )
    }
}

// ---------------------------------------------------------------------------
// The video
// ---------------------------------------------------------------------------

pub struct OriginlmPromoVideo<'a> {
    problem: ProblemScene,
    dashboard: DashboardScene,
    stack: StackScene,
    connect: ConnectScene,
    install: InstallScene,
    #[allow(dead_code)]
    media: &'a OriginlmPromoMedia,
}

impl<'a> OriginlmPromoVideo<'a> {
    pub fn new(media: &'a OriginlmPromoMedia) -> Self {
        Self {
            problem: ProblemScene,
            dashboard: DashboardScene,
            stack: StackScene,
            connect: ConnectScene,
            install: InstallScene,
            media,
        }
    }
}

impl std::fmt::Debug for OriginlmPromoVideo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OriginlmPromoVideo").finish()
    }
}

impl Video for OriginlmPromoVideo<'_> {
    const FPS: usize = 30;
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const BACKGROUND_COLOR: Color = Color::hex(BG);

    fn duration(&self) -> Duration<'_> {
        Duration::Auto
    }

    fn audio(&self) -> AudioMap<'_> {
        AudioMap::none()
    }

    fn define_scenes(&self) -> Scenes<'_> {
        Scenes::from(vec![
            &self.problem as &dyn Scene,
            &self.dashboard,
            &self.stack,
            &self.connect,
            &self.install,
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let progress = ((frame.seconds() / TOTAL_SECONDS).clamp(0.0, 1.0) * 1680.0).max(0.5);
        let logo = &self.media.originlm_white_png;
        fframes::svgr!(
            <svg xmlns="http://www.w3.org/2000/svg" width={WIDTH} height={HEIGHT} viewBox="0 0 1920 1080">
                <rect width="1920" height="1080" fill={BG} />
                {backdrop()}
                {ctx.render_scenes(&frame)}
                <image href={logo.href()} x="120" y="62" width="252" height="50" />
                <path d="M120 1046 H1800" stroke="#faf9f5" stroke-opacity="0.08" stroke-width="2" />
                <rect x="120" y="1045" width={progress} height="3" fill={AMBER} />
            </svg>
        )
    }
}

// ---------------------------------------------------------------------------
// Seamless 8 s loop (hero fallback)
// ---------------------------------------------------------------------------

pub struct OriginlmLoopVideo<'a> {
    stack: LoopScene,
    #[allow(dead_code)]
    media: &'a OriginlmPromoMedia,
}

#[derive(Debug)]
pub struct LoopScene;

impl Scene for LoopScene {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(8.0)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let layers = [
            ("Your app + dashboard", 720.0, 150.0, 250.0),
            ("SDK", 660.0, 102.0, 402.0),
            ("Transport", 700.0, 88.0, 512.0),
            ("Runtime", 744.0, 112.0, 632.0),
            ("Adapters", 788.0, 86.0, 748.0),
            ("Providers", 832.0, 118.0, 858.0),
        ];
        let panels: Vec<Svgr> = layers
            .iter()
            .enumerate()
            .map(|(index, (name, width, height, y))| {
                let x = 760.0 - width * 0.5;
                let breathe = (t / 8.0 * std::f32::consts::TAU + index as f32 * 0.7).sin() * 3.0;
                fframes::svgr!(<g>
                    <rect x={x} y={*y + breathe} width={*width} height={*height} rx="16" fill="url(#lm-panel)" stroke="#c9b6ab" stroke-opacity="0.26" />
                    <rect x={x + 22.0} y={*y + breathe + *height - 12.0} width={*width - 44.0} height="2" rx="1" fill={AMBER} fill-opacity="0.3" />
                    <text x="1262" y={*y + breathe + *height * 0.5 + 8.0} font-family={SANS} font-weight="500" font-size="25" fill={MUTED}>{*name}</text>
                </g>)
            })
            .collect();

        let packets: Vec<Svgr> = (0..12)
            .map(|index| {
                let phase = (t / 8.0 + index as f32 / 12.0) % 1.0;
                let travel = 1.0 - (phase * 2.0 - 1.0).abs();
                let y = 300.0 + travel * 700.0;
                let opacity = 0.2 + 0.6 * (phase * std::f32::consts::PI).sin();
                fframes::svgr!(<circle cx={760.0 + (index as f32 * 2.7).sin() * 110.0} cy={y} r={4.0 + (index % 3) as f32} fill={AMBER_BRIGHT} fill-opacity={opacity} />)
            })
            .collect();

        let providers = ["Claude", "Codex", "Ollama", "Cursor", "Grok", "…"];
        let nodes: Vec<Svgr> = providers
            .iter()
            .enumerate()
            .map(|(index, name)| {
                let x = 392.0 + index as f32 * 138.0;
                let tone = if index % 2 == 0 { AMBER } else { SAGE };
                let pulse = 1.0 + ((t / 8.0 * std::f32::consts::TAU) * 2.0 + index as f32).sin() * 0.08;
                fframes::svgr!(<g>
                    <circle cx={x} cy="978" r={14.0 * pulse} fill={tone} fill-opacity="0.16" />
                    <circle cx={x} cy="978" r={5.5 * pulse} fill={tone} />
                    <text x={x} y="1014" text-anchor="middle" font-family={SANS} font-weight="500" font-size="19" fill={MUTED}>{*name}</text>
                </g>)
            })
            .collect();

        fframes::svgr!(<g>
            {panels}
            {nodes}
            {packets}
        </g>)
    }
}

impl<'a> OriginlmLoopVideo<'a> {
    pub fn new(media: &'a OriginlmPromoMedia) -> Self {
        Self { stack: LoopScene, media }
    }
}

impl std::fmt::Debug for OriginlmLoopVideo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OriginlmLoopVideo").finish()
    }
}

impl Video for OriginlmLoopVideo<'_> {
    const FPS: usize = 30;
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const BACKGROUND_COLOR: Color = Color::hex(BG);

    fn duration(&self) -> Duration<'_> {
        Duration::Auto
    }

    fn audio(&self) -> AudioMap<'_> {
        AudioMap::none()
    }

    fn define_scenes(&self) -> Scenes<'_> {
        Scenes::from(vec![&self.stack as &dyn Scene])
    }

    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let logo = &self.media.originlm_white_png;
        fframes::svgr!(
            <svg xmlns="http://www.w3.org/2000/svg" width={WIDTH} height={HEIGHT} viewBox="0 0 1920 1080">
                <rect width="1920" height="1080" fill={BG} />
                {backdrop()}
                {ctx.render_scenes(&frame)}
                <image href={logo.href()} x="120" y="62" width="252" height="50" />
            </svg>
        )
    }
}

// ---------------------------------------------------------------------------
// Social cuts: 9:16 and 1:1
// ---------------------------------------------------------------------------

fn social_backdrop(width: usize, height: usize) -> Svgr<'static> {
    fframes::svgr!(
        <g>
            <defs>
                <radialGradient id="social-glow" cx="0.62" cy="0.26" r="0.72">
                    <stop offset="0" stop-color={AMBER} stop-opacity="0.17" />
                    <stop offset="1" stop-color={AMBER} stop-opacity="0" />
                </radialGradient>
                <pattern id="social-grid" width="56" height="56" patternUnits="userSpaceOnUse">
                    <path d="M56 0H0V56" fill="none" stroke={TEXT} stroke-opacity="0.04" />
                </pattern>
                <linearGradient id="social-panel" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0" stop-color="#faf9f5" stop-opacity="0.09" />
                    <stop offset="1" stop-color="#faf9f5" stop-opacity="0.025" />
                </linearGradient>
            </defs>
            <rect width={width} height={height} fill={BG} />
            <rect width={width} height={height} fill="url(#social-grid)" />
            <rect width={width} height={height} fill="url(#social-glow)" />
        </g>
    )
}

#[derive(Debug)]
pub struct SocialScene {
    pub portrait: bool,
}

impl Scene for SocialScene {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(10.5)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let portrait = self.portrait;
        let width = 1080.0;
        let (title_y, stack_top, stack_gap, panel_w, panel_h, badge_y) = if portrait {
            (420.0, 700.0, 100.0, 800.0, 78.0, 1370.0)
        } else {
            (232.0, 410.0, 58.0, 690.0, 50.0, 810.0)
        };
        let title_size = if portrait { 72.0 } else { 58.0 };
        let explode = 0.35 + 0.65 * ease_in_out((t - 0.2) / 2.0);
        let names = ["Your app + dashboard", "SDK", "Transport", "Runtime", "Adapters", "Providers"];
        let widths = [1.0, 0.94, 0.97, 1.0, 1.04, 1.08];
        let panels: Vec<Svgr> = names
            .iter()
            .enumerate()
            .map(|(index, name)| {
                let w = panel_w * widths[index];
                let x = (width - w) * 0.5;
                let y = stack_top + index as f32 * stack_gap * (0.35 + explode * 0.65);
                let opacity = 0.4 + explode * 0.6;
                fframes::svgr!(<g opacity={opacity}>
                    <rect x={x} y={y} width={w} height={panel_h} rx="14" fill="url(#social-panel)" stroke="#c9b6ab" stroke-opacity="0.24" />
                    <rect x={x + 22.0} y={y + panel_h - 10.0} width={w - 44.0} height="2" rx="1" fill={AMBER} fill-opacity="0.28" />
                    <text x={x + 24.0} y={y + panel_h * 0.5 + 7.0} font-family={SANS} font-weight="600" font-size={if portrait { 21.0 } else { 18.0 }} fill={TEXT} fill-opacity="0.78">{*name}</text>
                    {if index != 0 {
                        fframes::svgr!(<g opacity="0.5">
                            <rect x={x + 24.0} y={y + 14.0} width={w * 0.16} height="4" rx="2" fill={TEXT} fill-opacity="0.3" />
                            <rect x={x + 24.0} y={y + 24.0} width={w * 0.24} height="4" rx="2" fill={TEXT} fill-opacity="0.14" />
                        </g>)
                    } else { Svgr::empty() }}
                    {if index == 0 {
                        fframes::svgr!(<g>
                            <rect x={x + w - 190.0} y={y + 16.0} width="24" height="34" rx="3" fill={AMBER} fill-opacity="0.5" />
                            <rect x={x + w - 158.0} y={y + 24.0} width="24" height="26" rx="3" fill={SAGE} fill-opacity="0.45" />
                            <rect x={x + w - 126.0} y={y + 20.0} width="24" height="30" rx="3" fill={AMBER} fill-opacity="0.7" />
                            <rect x={x + w - 94.0} y={y + 28.0} width="24" height="22" rx="3" fill={SAGE} fill-opacity="0.35" />
                        </g>)
                    } else { Svgr::empty() }}
                    {if index == 5 {
                        fframes::svgr!(<g>
                            <circle cx={x + w - 200.0} cy={y + panel_h * 0.5} r="7" fill={AMBER} />
                            <circle cx={x + w - 166.0} cy={y + panel_h * 0.5} r="7" fill={SAGE} />
                            <circle cx={x + w - 132.0} cy={y + panel_h * 0.5} r="7" fill={AMBER} />
                            <circle cx={x + w - 98.0} cy={y + panel_h * 0.5} r="7" fill={SAGE} />
                            <circle cx={x + w - 64.0} cy={y + panel_h * 0.5} r="7" fill={AMBER} />
                        </g>)
                    } else { Svgr::empty() }}
                </g>)
            })
            .collect();

        let packets: Vec<Svgr> = (0..10)
            .map(|index| {
                let phase = (t * 0.4 + index as f32 / 10.0) % 1.0;
                let travel = 1.0 - (phase * 2.0 - 1.0).abs();
                let start = stack_top - 30.0;
                let end = stack_top + 5.0 * stack_gap + panel_h;
                let y = start + travel * (end - start);
                let opacity = 0.2 + 0.6 * (phase * std::f32::consts::PI).sin();
                fframes::svgr!(<circle cx={width * 0.5 + (index as f32 * 2.4).sin() * 90.0} cy={y} r={4.0 + (index % 3) as f32} fill={AMBER_BRIGHT} fill-opacity={opacity} />)
            })
            .collect();

        let badge = spring_at(t, 4.6, 1.0);
        let url_opacity = enter(t, 5.4, 0.6);

        fframes::svgr!(
            <g>
                <g opacity={enter(t, 0.05, 0.6)} transform={format!("translate(0 {:.1})", (1.0 - enter(t, 0.05, 0.6)) * 22.0)}>
                    <text x={if portrait { 80.0 } else { 64.0 }} y={title_y - 84.0} font-family={SANS} font-weight="600" font-size="22" letter-spacing="3" fill={AMBER}>"ORIGINLM · LOCAL-FIRST"</text>
                    <text x={if portrait { 80.0 } else { 64.0 }} y={title_y} font-family={SANS} font-weight="600" font-size={title_size} letter-spacing="-2" fill={TEXT}>"See and use"</text>
                    <text x={if portrait { 80.0 } else { 64.0 }} y={title_y + title_size * 1.08} font-family={SANS} font-weight="600" font-size={title_size} letter-spacing="-2" fill={TEXT}>"all your AI."</text>
                    <text x={if portrait { 80.0 } else { 64.0 }} y={title_y + title_size * 2.02} font-family={SANS} font-weight="600" font-size={if portrait { 38.0 } else { 30.0 }} letter-spacing="-0.5" fill={AMBER}>"One dependency for every app."</text>
                    {if portrait {
                        fframes::svgr!(<text x="80" y={title_y + title_size * 2.02 + 46.0} font-family={SANS} font-weight="500" font-size="27" fill={MUTED}>"Usage, quotas, models and routes in one place."</text>)
                    } else { Svgr::empty() }}
                </g>
                {panels}
                {packets}
                <g opacity={clamp01(badge)} transform={format!("translate({:.1} {:.1}) scale({:.3})", width * 0.5, badge_y, 0.94 + badge * 0.06)}>
                    <rect x={-260.0} y={-46.0} width="520" height="92" rx="46" fill={PANEL_2} stroke="#faf9f5" stroke-opacity="0.18" />
                    <circle cx={-218.0} cy="0" r="10" fill={GREEN} />
                    <text x={-190.0} y="-6" font-family={SANS} font-weight="600" font-size="25" fill={TEXT}>"Local AI connected"</text>
                    <text x={-190.0} y="24" font-family={SANS} font-weight="500" font-size="20" fill={MUTED}>"Claude, Codex, Ollama"</text>
                </g>
                <g opacity={url_opacity}>
                    <text x={width * 0.5} y={badge_y + 104.0} text-anchor="middle" font-family={SANS} font-weight="600" font-size="30" fill={AMBER}>"originlm.dev"</text>
                    <text x={width * 0.5} y={badge_y + 140.0} text-anchor="middle" font-family={SANS} font-weight="500" font-size="20" fill={FAINT}>"Free · MIT · no telemetry"</text>
                </g>
            </g>
        )
    }
}

pub struct OriginlmVerticalVideo<'a> {
    scene: SocialScene,
    #[allow(dead_code)]
    media: &'a OriginlmPromoMedia,
}

impl<'a> OriginlmVerticalVideo<'a> {
    pub fn new(media: &'a OriginlmPromoMedia) -> Self {
        Self { scene: SocialScene { portrait: true }, media }
    }
}

impl std::fmt::Debug for OriginlmVerticalVideo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OriginlmVerticalVideo").finish()
    }
}

impl Video for OriginlmVerticalVideo<'_> {
    const FPS: usize = 30;
    const WIDTH: usize = 1080;
    const HEIGHT: usize = 1920;
    const BACKGROUND_COLOR: Color = Color::hex(BG);

    fn duration(&self) -> Duration<'_> {
        Duration::Auto
    }

    fn audio(&self) -> AudioMap<'_> {
        AudioMap::none()
    }

    fn define_scenes(&self) -> Scenes<'_> {
        Scenes::from(vec![&self.scene as &dyn Scene])
    }

    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        fframes::svgr!(
            <svg xmlns="http://www.w3.org/2000/svg" width="1080" height="1920" viewBox="0 0 1080 1920">
                {social_backdrop(1080, 1920)}
                {ctx.render_scenes(&frame)}
                <image href={self.media.originlm_white_png.href()} x="80" y="120" width="260" height="51" />
            </svg>
        )
    }
}

pub struct OriginlmSquareVideo<'a> {
    scene: SocialScene,
    #[allow(dead_code)]
    media: &'a OriginlmPromoMedia,
}

impl<'a> OriginlmSquareVideo<'a> {
    pub fn new(media: &'a OriginlmPromoMedia) -> Self {
        Self { scene: SocialScene { portrait: false }, media }
    }
}

impl std::fmt::Debug for OriginlmSquareVideo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OriginlmSquareVideo").finish()
    }
}

impl Video for OriginlmSquareVideo<'_> {
    const FPS: usize = 30;
    const WIDTH: usize = 1080;
    const HEIGHT: usize = 1080;
    const BACKGROUND_COLOR: Color = Color::hex(BG);

    fn duration(&self) -> Duration<'_> {
        Duration::Auto
    }

    fn audio(&self) -> AudioMap<'_> {
        AudioMap::none()
    }

    fn define_scenes(&self) -> Scenes<'_> {
        Scenes::from(vec![&self.scene as &dyn Scene])
    }

    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        fframes::svgr!(
            <svg xmlns="http://www.w3.org/2000/svg" width="1080" height="1080" viewBox="0 0 1080 1080">
                {social_backdrop(1080, 1080)}
                {ctx.render_scenes(&frame)}
                <image href={self.media.originlm_white_png.href()} x="64" y="70" width="220" height="43" />
            </svg>
        )
    }
}
