import { Suspense, lazy, useEffect, useRef, useState } from "react";
import { ArrowRight, Sparkle, Stack } from "../components/Icon";
import { LAYERS, type Layer } from "../data/layers";

const HeroScene = lazy(() => import("../scene/HeroScene"));

function supportsWebGL() {
  try {
    const canvas = document.createElement("canvas");
    return Boolean(canvas.getContext("webgl2") ?? canvas.getContext("webgl"));
  } catch {
    return false;
  }
}

export function HeroStage() {
  const stageRef = useRef<HTMLDivElement>(null);
  const labelRefs = useRef<(HTMLDivElement | null)[]>([]);
  const [enabled, setEnabled] = useState(false);
  const [ready, setReady] = useState(false);
  const [active, setActive] = useState<Layer | null>(null);
  const [quality, setQuality] = useState<"full" | "light">("full");
  const [staticMode, setStaticMode] = useState(false);

  useEffect(() => {
    const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    if (reduced || !supportsWebGL()) {
      setStaticMode(true);
      return;
    }

    setQuality(
      window.innerWidth >= 900 && (navigator.hardwareConcurrency ?? 4) >= 4 && !new URLSearchParams(window.location.search).has("light")
        ? "full"
        : "light",
    );

    let cancelled = false;
    const start = () => {
      if (!cancelled) setEnabled(true);
    };
    // Let the hero copy paint before the 3D chunk is requested.
    const timer = window.setTimeout(start, 450);

    return () => {
      cancelled = true;
      window.clearTimeout(timer);
    };
  }, []);

  return (
    <div className="hero-stage" ref={stageRef} data-static={staticMode || undefined}>
      <img
        className="stage-fallback"
        src="/hero-poster.svg"
        alt="The OriginLM stack, exploded: your app and the usage dashboard on top, then the SDK, transport, local runtime, adapters, and the AI providers already on the machine."
        width={1200}
        height={760}
        fetchPriority="high"
        style={{ opacity: ready ? 0 : 1, transition: "opacity 500ms ease" }}
      />

      {enabled && !staticMode && (
        <Suspense fallback={null}>
          <HeroScene
            stageEl={stageRef.current}
            labels={labelRefs}
            quality={quality}
            onHover={setActive}
            onReady={() => setReady(true)}
          />
        </Suspense>
      )}

      {enabled && !ready && !staticMode && (
        <div className="stage-loading" aria-hidden="true">
          <span className="pulse" />
        </div>
      )}

      <div className="stage-hint" aria-hidden="true">
        <Stack size={13} />
        <span className="hint-desktop">{staticMode ? "Static diagram · reduced motion" : "Scroll to separate · hover a layer"}</span>
        <span className="hint-mobile">{staticMode ? "Static diagram · reduced motion" : "Tap a layer to inspect"}</span>
      </div>

      <div className="stage-labels" aria-hidden="true">
        {LAYERS.map((layer, index) => (
          <div
            key={layer.id}
            ref={(node) => { labelRefs.current[index] = node; }}
            className="layer-label"
            data-active={active?.id === layer.id}
            style={{ opacity: 0 }}
          >
            <i />
            {layer.label}
          </div>
        ))}
      </div>

      <div className="stage-legend" aria-hidden="true">
        <span><i /> same runtime</span>
        <span><i /> local only</span>
      </div>

      <div className="layer-tooltip" data-visible={active !== null} role="status" aria-live="polite">
        {active ? (
          <>
            <strong>{active.title}</strong> — {active.detail}
          </>
        ) : (
          <span>&nbsp;</span>
        )}
      </div>

      <ul className="visually-hidden">
        {LAYERS.map((layer) => (
          <li key={layer.id}>
            {layer.title}: {layer.detail}
          </li>
        ))}
      </ul>
    </div>
  );
}

export function Hero() {
  return (
    <header className="hero" id="top">
      <div className="grid-wash" aria-hidden="true" />
      <div className="container">
        <div className="hero-copy">
          <p className="hero-pill">
            <span className="dot"><Sparkle size={12} /></span>
            <span><strong>OriginLM</strong> · local-first, no telemetry</span>
          </p>

          <h1>
            <span className="line">See and use all your AI.</span>
            <span className="line accent">One dependency for every app.</span>
          </h1>

          <p className="hero-sub">
            A local control center for every AI client, model, account and subscription on your machine.
            The same runtime is the <strong>one dependency</strong> that puts the user's own AI in your app.
          </p>

          <div className="hero-actions">
            <a className="btn btn-primary" href="#install">
              Get OriginLM <ArrowRight size={17} />
            </a>
            <a className="btn btn-ghost" href="#developers">
              Build with it
            </a>
          </div>

          <p className="hero-note">
            <span>free &amp; MIT-licensed</span>
            <span>macOS · Windows · Linux</span>
            <span>no account required</span>
          </p>
        </div>

        <div className="hero-stage-wrap">
          <HeroStage />
        </div>
      </div>
    </header>
  );
}
