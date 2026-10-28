mod app;
mod brand;
mod broker;
mod data;
mod history;
mod human;
mod icons;
mod pages;
mod theme;
mod ui;

use app::Desktop;
use gpui_kit::component::{ThemeMode, TitleBar};
use gpui_kit::*;
use std::{borrow::Cow, path::PathBuf, sync::Arc, time::Duration};

fn parse_args() -> (PathBuf, u64) {
    let mut token: Option<PathBuf> = None;
    let mut refresh = 30u64;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--token-file" => token = args.next().map(PathBuf::from),
            "--refresh" => {
                refresh = args
                    .next()
                    .and_then(|value| value.parse().ok())
                    .filter(|value: &u64| (5..=600).contains(value))
                    .unwrap_or(30);
            }
            "--help" | "-h" => {
                println!(
                    "local-ai-kit-desktop [--token-file PATH] [--refresh SECONDS]\n\
                     Reads the shared broker over its public authenticated API."
                );
                std::process::exit(0);
            }
            _ => {}
        }
    }
    let token = token
        .or_else(|| std::env::var_os("LAI_DESKTOP_TOKEN_FILE").map(PathBuf::from))
        .or_else(default_token_file)
        .unwrap_or_else(|| PathBuf::from("admin-token.json"));
    (token, refresh)
}

fn default_token_file() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let state = PathBuf::from(&home).join(".local/state/local-ai-kit/fedora-shared");
    [
        state.join("control-admin-token.json"),
        state.join("control-token.json"),
        PathBuf::from(&home).join(".local-ai-kit/broker-admin-token.json"),
    ]
    .into_iter()
    .find(|path| path.is_file())
}

fn main() {
    let (token_path, refresh) = parse_args();
    let application = gpui_kit::application().with_assets(gpui_kit::assets::Assets);
    application.run(move |cx| {
        let _ = cx
            .text_system()
            .add_fonts(vec![Cow::Borrowed(include_bytes!(
                "../assets/fonts/RedHatDisplay-Variable.ttf"
            ) as &[u8])]);
        gpui_kit::init(cx);
        theme::install(cx, true);
        let _ = ThemeMode::Dark;
        let token = token_path.clone();
        let (width, height) = std::env::var("LAI_DESKTOP_WINDOW")
            .ok()
            .and_then(|value| {
                let (w, h) = value.split_once('x')?;
                Some((w.parse::<f32>().ok()?, h.parse::<f32>().ok()?))
            })
            .unwrap_or((1440.0, 920.0));
        let light = std::env::var_os("LAI_DESKTOP_THEME").is_some_and(|value| value == "light");
        if light {
            theme::install(cx, false);
        }
        let icon = image::load_from_memory(include_bytes!("../assets/brand/originlm-app-256.png"))
            .ok()
            .map(|image| Arc::new(image.to_rgba8()));
        let mut options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(110.), px(70.)),
                size: size(px(width), px(height)),
            })),
            window_min_size: Some(size(px(1000.), px(660.))),
            ..TitleBar::window_options()
        };
        if let Some(titlebar) = options.titlebar.as_mut() {
            titlebar.title = Some("OriginLM".into());
        }
        options.icon = icon;
        gpui_kit::open_window(options, cx, move |window, cx| {
            cx.new(|cx| Desktop::new(window, cx, token, refresh))
        })
        .expect("desktop window");
        let _ = Duration::from_secs(1);
    });
}
