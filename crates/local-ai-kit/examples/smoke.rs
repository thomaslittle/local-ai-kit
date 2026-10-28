//! Manual smoke: `LAI_HELPER_PATH=target/debug/local-ai-kit cargo run -p local-ai-kit --example smoke`
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ai = local_ai_kit::LocalAi::start()?;
    let status = ai.status()?;
    println!(
        "mode={:?} providers={:?} models={} revision={}",
        status.mode, status.providers, status.models, status.revision
    );
    Ok(())
}
