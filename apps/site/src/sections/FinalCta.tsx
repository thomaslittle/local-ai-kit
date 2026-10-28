import { ArrowRight, Sparkle } from "../components/Icon";

export function FinalCta() {
  return (
    <section className="section" aria-labelledby="cta-title">
      <div className="container">
        <div className="final-cta reveal">
          <span className="eyebrow" style={{ justifyContent: "center" }}>Install once</span>
          <h2 id="cta-title">Your AI is already on the machine. Use it everywhere.</h2>
          <p>
            Install OriginLM for the dashboard, and every app you build on it already has a backend.
            No API keys, no relay, no surprises.
          </p>
          <div className="hero-actions">
            <a className="btn btn-primary" href="#install">Get OriginLM <ArrowRight size={17} /></a>
            <a className="btn btn-ghost" href="#integrations">Read the integration guides</a>
          </div>
          <p className="hero-note" style={{ marginTop: "1.25rem" }}>
            <span><Sparkle size={12} /> Free and MIT-licensed</span>
            <span>No account</span>
            <span>No telemetry</span>
          </p>
        </div>
      </div>
    </section>
  );
}
