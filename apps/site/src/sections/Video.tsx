import { Play } from "../components/Icon";
import { PROMO } from "../data/video";

const CHAPTERS = [
  ["00:00", "Know your AI", "Every client, quota and reset time in one local view."],
  ["00:24", "Build on it", "The exploded stack, from providers up to your app."],
  ["00:38", "Connect", "Discovery, a revocable grant and the connected badge."],
];

export function Video() {
  return (
    <section className="section video" id="video" aria-labelledby="video-title">
      <div className="container">
        <div className="head-split">
          <h2 id="video-title">One runtime, two stories, in under a minute.</h2>
          <p className="lead">
            The same film in the hero and in the app: first what you can see, then what you can build.
            Rendered with fframes from the same palette and geometry as this page.
          </p>
        </div>

        <div className="video-layout">
          <figure className="video-frame">
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
              <div className="video-poster" data-slot="film">
                <img src="/video-poster.svg" alt="" width={1600} height={900} loading="lazy" />
                <div className="video-placeholder" data-keep>
                  <span className="video-play" aria-hidden="true"><Play size={26} /></span>
                  <div>
                    <h3>The 52-second tour</h3>
                    <p>
                      Rendering now. The final cut ships as H.264 and WebM with a poster frame, silent by
                      design: no unlicensed music or voiceover.
                    </p>
                  </div>
                </div>
              </div>
            )}
            <figcaption>
              1920×1080, H.264 and WebM. 52 seconds, silent. Made with fframes; the source is in media/video.
            </figcaption>
          </figure>

          <ol className="chapters">
            {CHAPTERS.map(([time, title, body]) => (
              <li key={title}>
                <time>{time}</time>
                <h3>{title}</h3>
                <p>{body}</p>
              </li>
            ))}
          </ol>
        </div>
      </div>
    </section>
  );
}
