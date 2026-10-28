import { Play } from "../components/Icon";
import { PROMO } from "../data/video";

const CHAPTERS = [
  ["00:00", "Know your AI", "Every client, quota and reset time in one local view."],
  ["00:24", "Build on it", "The exploded stack, from providers up to your app."],
  ["00:38", "Connect", "Discovery, a revocable grant and the connected badge."],
];

export function Video() {
  return (
    <section className="section" id="video" aria-labelledby="video-title">
      <div className="container">
        <div className="section-head section-head--wide reveal">
          <span className="eyebrow">Product film</span>
          <h2 id="video-title">One runtime, two stories — in under a minute.</h2>
          <p className="lede">
            The same film in the hero and in the app: first what you can see, then what you can build.
            Rendered with fframes from the same palette and geometry as this page.
          </p>
        </div>

        <div className="video-shell reveal">
          <div className="video-frame">
            {PROMO.hasVideo ? (
              <video
                controls
                preload="metadata"
                playsInline
                poster={PROMO.poster}
                aria-label="OriginLM product film: know your AI usage, then build on it."
              >
                <source src={PROMO.webm} type="video/webm" />
                <source src={PROMO.mp4} type="video/mp4" />
                Your browser cannot play this video. The poster image and the sections above cover the
                same material.
              </video>
            ) : (
              <>
                <img className="video-poster" src="/hero-poster.svg" alt="" width={1200} height={760} loading="lazy" />
                <div className="video-placeholder">
                  <span className="video-play" aria-hidden="true"><Play size={26} /></span>
                  <div>
                    <span className="chip chip--brand">in post-production</span>
                    <h3>The 52-second tour</h3>
                    <p>
                      Rendering now in fframes. The final cut ships as H.264 and WebM with a poster
                      frame — silent by design, with no unlicensed music or voiceover.
                    </p>
                  </div>
                </div>
              </>
            )}
          </div>
          <div className="video-meta">
            <span>1920×1080 · H.264 / WebM</span>
            <span>52 seconds · silent</span>
            <span>fframes · source in media/video</span>
          </div>
        </div>

        <div className="video-chapters">
          {CHAPTERS.map(([time, title, body]) => (
            <div className="security-card" key={title}>
              <span className="chip chip--brand">{time}</span>
              <h3 style={{ marginTop: "0.8rem", fontSize: "0.98rem" }}>{title}</h3>
              <p>{body}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
