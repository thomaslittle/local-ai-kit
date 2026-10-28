mod stage;
use std::sync::Arc;
//! OriginLM: a silent, evidence-based product film. All UI footage is example data.
use fframes::{AudioMap, Color, Duration, FFramesContext, Frame, Scene, Scenes, Svgr, Transform, Video, animation::Easing, include_media_dir};
include_media_dir!(pub struct OriginlmFilmMedia, "media");
pub const WIDTH: usize = 1920;
pub const HEIGHT: usize = 1080;
const BG: &str = "#1e1a1b";
const FG: &str = "#f5f1ed";
const AMBER: &str = "#eba31a";
const MUTED: &str = "#b9afa7";
const FONT: &str = "Red Hat Display";

pub struct OriginlmFilmVideo<'a> {
    pub media: &'a OriginlmFilmMedia,
    shots: Vec<Shot>,
}
impl<'a> OriginlmFilmVideo<'a> {
    pub fn new(media: &'a OriginlmFilmMedia, _: &'a str) -> Self {
        let stage = Arc::new(stage::LayerStage::new());
        let shots = [
            ("Know", 6., "Know all your AI.", "Whatever AI you already have.", None),
            ("Overview", 6., "One place to look.", "Your accounts. Their reported allowances.", Some("overview.png")),
            ("Allowances", 5., "See what is left.", "Usage, shared pools and reset evidence.", Some("usage.png")),
            ("Models", 4., "Know your options.", "Models keep their provider and route.", Some("models.png")),
            ("Evidence", 5., "Unknown stays unknown.", "Evidence and problems stay visible.", Some("diagnostics.png")),
            ("Build", 4., "Build with it.", "One dependency. The AI your users have.", None),
            ("Layers", 6., "A local runtime.", "Your app stays above the provider details.", None),
            ("Discover", 5., "Start with discovery.", "Imports do nothing. Your app asks explicitly.", None),
            ("Grants", 5., "Each app gets a grant.", "Discovery is not permission to execute.", None),
            ("Origin", 6., "See your AI. Build with it.", "OriginLM · source preview", None),
        ].into_iter().enumerate().map(|(kind,(name,seconds,title,caption,image))| Shot {kind,name,seconds,title,caption,image,stage:stage.clone()}).collect();
        Self { media, shots }
    }
}
impl std::fmt::Debug for OriginlmFilmVideo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("OriginlmFilmVideo").finish() }
}
impl Video for OriginlmFilmVideo<'_> {
    const FPS: usize = 30;
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const BACKGROUND_COLOR: Color = Color::BLACK;
    fn duration(&self) -> Duration<'_> { Duration::Auto }
    fn audio(&self) -> AudioMap<'_> { AudioMap::none() }
    fn define_scenes(&self) -> Scenes<'_> { Scenes::from(self.shots.iter().map(|s|s as &dyn Scene).collect::<Vec<_>>()) }
    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        fframes::svgr!(<svg xmlns="http://www.w3.org/2000/svg" width={WIDTH} height={HEIGHT}>
            <rect width={WIDTH} height={HEIGHT} fill={BG}/>
            {ctx.render_scenes(&frame)}
        </svg>)
    }
}
#[derive(Debug)]
struct Shot { kind: usize, name: &'static str, seconds: f32, title: &'static str, caption: &'static str, image: Option<&'static str>, stage: Arc<stage::LayerStage> }
impl Scene for Shot {
    fn name(&self) -> &'static str {self.name}
    fn duration(&self) -> Duration<'_> {Duration::Seconds(self.seconds)}
    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t=frame.seconds();
        let content = if let Some(file)=self.image {
            ctx.get_image(file).map(|image| fframes::svgr!(
                <g transform={frame.animate(fframes::timeline!(at 0.0 => 0.65, animate Transform::translate(0, 50) => Transform::translate(0, 0), Easing::EaseOut,))}>
                    <image href={image.href()} x="320" y="290" width="1280" height="800"/>
                    <rect x="320" y="990" width="1280" height="90" fill={BG}/>
                </g>
            )).unwrap_or_default()
        } else { match self.kind {
            0 => fframes::svgr!(<g>
                <text x="160" y="590" fill={AMBER} font-family={FONT} font-size="70" font-weight="500">"Accounts. Allowances. Models."</text>
                <path d="M160 680H1760" stroke="#514338" stroke-width="2"/>
                <circle cy="680" r="8" fill={AMBER} cx={160. + (t / 6.) * 1600.}/>
            </g>),
            5 => fframes::svgr!(<g>
                <text x="160" y="580" fill={AMBER} font-family={FONT} font-size="110" font-weight="600">"Rust + TypeScript + Python"</text>
                <text x="160" y="710" fill={MUTED} font-family={FONT} font-size="46">"Tauri · Electron · GPUI · web"</text>
            </g>),
            6 => {
                let image = self.stage.image(t);
                fframes::svgr!(<image href={image.href()} x="220" y="250" width="1480" height="740"/>)
            },
            7 => fframes::svgr!(<g>
                <path d="M160 350V820H1760" fill="none" stroke={AMBER} stroke-width="3"/>
                <text x="210" y="470" font-family={FONT} font-size="46" fill={MUTED}>"import { createLocalAI } from '@local-ai-kit/sdk';"</text>
                <g opacity={frame.animate(fframes::timeline!(at 0.8 => 1.2, animate 0.0 => 1.0, Easing::EaseOut,))}>
                    <text x="210" y="600" font-family={FONT} font-size="54" fill={FG}>"const client = await createLocalAI();"</text>
                </g>
                <g opacity={frame.animate(fframes::timeline!(at 1.6 => 2.0, animate 0.0 => 1.0, Easing::EaseOut,))}>
                    <text x="210" y="730" font-family={FONT} font-size="54" fill={AMBER}>"const snapshot = await client.snapshot.get();"</text>
                </g>
            </g>),
            8 => fframes::svgr!(<g>
                <text x="160" y="460" font-family={FONT} font-size="64" fill={FG}>"Your app"</text>
                <path d="M160 510H1760" stroke="#514338" stroke-width="2"/>
                <text x="160" y="610" font-family={FONT} font-size="56" fill={AMBER}>"Granted: passive discovery"</text>
                <text x="160" y="710" font-family={FONT} font-size="48" fill={MUTED}>"Execution requires a separate permission."</text>
                <text x="160" y="830" font-family={FONT} font-size="32" fill={MUTED}>"Illustrated grant · no account is connected in this film"</text>
            </g>),
            _ => fframes::svgr!(<g>
                <text x="160" y="550" font-family={FONT} font-size="144" font-weight="700" fill={AMBER}>"OriginLM"</text>
                <text x="160" y="710" font-family={FONT} font-size="48" fill={FG}>"github.com/thomaslittle/local-ai-kit"</text>
                <text x="160" y="820" font-family={FONT} font-size="40" fill={MUTED}>"Build from source today. Registry release pending."</text>
            </g>),
        }};
        fframes::svgr!(<g>
            <text x="160" y="90" fill={AMBER} font-family={FONT} font-size="30" font-weight="600">{if self.kind<5 {"SEE YOUR AI"} else {"BUILD WITH IT"}}</text>
            <g transform={frame.animate(fframes::timeline!(at 0.0 => 0.5, animate Transform::translate(0, 24) => Transform::translate(0, 0), Easing::EaseOut,))}>
                <text x="160" y="205" fill={FG} font-family={FONT} font-size="92" font-weight="650">{self.title}</text>
            </g>
            {content}
            <rect x="0" y="990" width={WIDTH} height="90" fill={BG}/>
            <text x="160" y="1040" fill={MUTED} font-family={FONT} font-size="32">{self.caption}</text>
            <text x="1760" y="90" text-anchor="end" fill={MUTED} font-family={FONT} font-size="28">{if self.image.is_some() {"EXAMPLE DATA"} else {"OriginLM"}}</text>
        </g>)
    }
}
