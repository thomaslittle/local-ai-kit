import { ArrowUpRight, EyeOff, KeyRound, Lock, Shield } from "../components/Icon";

const CARDS = [
  {
    icon: EyeOff,
    title: "No secrets exported",
    body: "Provider credentials stay with the native client that already owns them. The SDK reports facts, never tokens or cookies. Browser cookies are never read.",
  },
  {
    icon: Lock,
    title: "Loopback only",
    body: "The HTTP surface binds to 127.0.0.1 with strict origin checks and one-time pairing. The extension bridge opens no port at all.",
  },
  {
    icon: KeyRound,
    title: "Revocable per-app grants",
    body: "Every application gets its own grant, scoped to routes and workspaces. Revoke it and owned sessions stop; another app's work is untouched.",
  },
  {
    icon: Shield,
    title: "No telemetry, no relay",
    body: "Nothing leaves the machine. Usage history and grants live in a local state database. There is no account to create and no analytics endpoint to call.",
  },
];

export function Security() {
  return (
    <section className="section" id="security" aria-labelledby="security-title">
      <div className="container">
        <div className="section-head section-head--wide reveal">
          <span className="eyebrow">Security and privacy</span>
          <h2 id="security-title">A local runtime is only useful if it stays local.</h2>
          <p className="lede">
            OriginLM is not a relay and not a proxy. It is a boundary: one place that knows which app
            asked for what, and one place where the user can take it back.
          </p>
        </div>

        <div className="security-grid reveal">
          {CARDS.map(({ icon: Icon, title, body }) => (
            <article className="security-card" key={title}>
              <span className="icon-tile"><Icon size={20} /></span>
              <h3>{title}</h3>
              <p>{body}</p>
            </article>
          ))}
        </div>

        <div className="security-foot reveal">
          <p>
            <strong>Read the actual guarantees.</strong> The threat model names the assets, the trust
            boundaries and the attacks that are out of scope. The privacy statement is generated from one
            source so the install page, README and app cannot drift.
          </p>
          <div className="security-links">
            <a href="https://github.com/thomaslittle/local-ai-kit/blob/main/SECURITY.md">Security policy <ArrowUpRight size={12} /></a>
            <a href="https://github.com/thomaslittle/local-ai-kit/blob/main/docs/security/threat-model.md">Threat model <ArrowUpRight size={12} /></a>
          </div>
        </div>
      </div>
    </section>
  );
}
