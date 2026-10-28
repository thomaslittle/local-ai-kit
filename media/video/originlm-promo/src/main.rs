use fframes::cli::clap;
use fframes::{EncoderOptions, RenderOptions, StaticMediaProvider, cli};
use fframes_skia_renderer::{
    SkiaFFramesRenderer, SkiaPipelineConcurrencyPolicy, SkiaPipelineConfig,
    vulkan::SkiaVulkanCtx,
};
use originlm_promo::{
    HEIGHT, OriginlmLoopVideo, OriginlmPromoMedia, OriginlmPromoVideo, OriginlmSquareVideo,
    OriginlmVerticalVideo, WIDTH,
};
use std::process::ExitCode;

#[derive(Debug, clap::Args)]
struct VideoArgs {
    /// promo (default), loop
    #[arg(long, default_value = "promo", global = true)]
    format: String,
}

fn main() -> ExitCode {
    let args = cli::parse::<VideoArgs>();
    let media = OriginlmPromoMedia::prepare().expect("media");
    let gpu = SkiaVulkanCtx::new(WIDTH, HEIGHT).expect("GPU context");
    let render_options = || RenderOptions {
        media: Some(&media),
        video_encoder_options: EncoderOptions {
            preferred_encoder: Some("libx264"),
            codec_params: Some(&[("crf", "21"), ("preset", "slow"), ("tune", "animation")]),
            ..Default::default()
        },
        ..Default::default()
    };

    match args.app.format.as_str() {
        "loop" => {
            let video = OriginlmLoopVideo::new(&media);
            cli::new(&video, render_options())
                .args(args)
                .backend(
                    SkiaFFramesRenderer::new_vulkan(
                        &gpu,
                        SkiaPipelineConfig {
                            concurrency_policy: SkiaPipelineConcurrencyPolicy::MaxPerformance,
                            ..Default::default()
                        },
                    )
                    .expect("skia renderer"),
                )
                .preview(fframes_native_player::cli_preview)
                .run()
        }
        "vertical" => {
            let video = OriginlmVerticalVideo::new(&media);
            let gpu = SkiaVulkanCtx::new(1080, 1920).expect("GPU context");
            cli::new(&video, render_options())
                .args(args)
                .backend(
                    SkiaFFramesRenderer::new_vulkan(
                        &gpu,
                        SkiaPipelineConfig {
                            concurrency_policy: SkiaPipelineConcurrencyPolicy::MaxPerformance,
                            ..Default::default()
                        },
                    )
                    .expect("skia renderer"),
                )
                .preview(fframes_native_player::cli_preview)
                .run()
        }
        "square" => {
            let video = OriginlmSquareVideo::new(&media);
            let gpu = SkiaVulkanCtx::new(1080, 1080).expect("GPU context");
            cli::new(&video, render_options())
                .args(args)
                .backend(
                    SkiaFFramesRenderer::new_vulkan(
                        &gpu,
                        SkiaPipelineConfig {
                            concurrency_policy: SkiaPipelineConcurrencyPolicy::MaxPerformance,
                            ..Default::default()
                        },
                    )
                    .expect("skia renderer"),
                )
                .preview(fframes_native_player::cli_preview)
                .run()
        }
        _ => {
            let video = OriginlmPromoVideo::new(&media);
            cli::new(&video, render_options())
                .args(args)
                .backend(
                    SkiaFFramesRenderer::new_vulkan(
                        &gpu,
                        SkiaPipelineConfig {
                            concurrency_policy: SkiaPipelineConcurrencyPolicy::MaxPerformance,
                            ..Default::default()
                        },
                    )
                    .expect("skia renderer"),
                )
                .preview(fframes_native_player::cli_preview)
                .run()
        }
    }
}
