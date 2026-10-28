import { useEffect, useRef, useState } from "react";
import { CodeBlock } from "../components/CodeBlock";

const STEPS = [
  {
    id: "discover",
    label: "Discover",
    title: "Discover what is already there",
    body: "A passive inventory of installations, profiles and endpoints. It reads install locations and provider-reported metadata, never browser cookies, and never on import.",
    code: `// Passive inventory: locations, profiles, endpoints.
await ai.discovery.refresh({ level: "passive" });
const { connections, providers } = await ai.snapshot.get();`,
  },
  {
    id: "grant",
    label: "Grant",
    title: "Ask for exactly what you need",
    body: "Discovery is not permission. A host requests a scoped grant for routes, workspaces and whether metered billing is allowed. The user can revoke it per app.",
    code: `// Scoped, explicit, revocable by the user.
const grant = await ai.grants.request({
  scopes: ["snapshot.read", "sessions.run"],
  connectionIds: [selectedConnection],
});`,
  },
  {
    id: "session",
    label: "Session",
    title: "Run in a session you own",
    body: "A selection names the route, model and revision. The session streams events; cancellation only touches work this app started.",
    code: `const session = await ai.sessions.create({
  selection: { routeId, modelId, revision },
  workspaceId,
});
const turn = await ai.turns.start({
  sessionId: session.id,
  prompt,
  requestId: crypto.randomUUID(),
});`,
  },
];

function DiscoverVisual() {
  return (
    <figure className="shot shot--panel" data-slot="connections" data-kind="product">
      <img
        src="/screens/control-center-connections.webp"
        width={2880}
        height={1800}
        loading="lazy"
        alt="The control center's Connections screen with example data: authenticated profiles for Codex, Antigravity, Grok and OpenCode, each with plan, profile, account and installation."
      />
    </figure>
  );
}

function GrantVisual() {
  return (
    <div className="grant-card" aria-hidden="true">
      <div className="grant-head">
        <span className="app-icon" aria-hidden="true">E</span>
        <div>
          <strong>Example Studio</strong>
          <small>requesting access from 127.0.0.1</small>
        </div>
      </div>
      <div className="grant-list">
        {[
          ["Usage and quota snapshot", false],
          ["Model catalog", false],
          ["Run sessions", false],
          ["Workspace ~/Projects/example", true],
        ].map(([label, off]) => (
          <div className="grant-item" key={String(label)} data-off={off}>
            <span>{label}</span>
            <span className="toggle" />
          </div>
        ))}
      </div>
    </div>
  );
}

function SessionVisual() {
  return (
    <div className="session-card" aria-hidden="true">
      <div className="session-line"><span>route</span><code>claude-code/subscription</code></div>
      <div className="session-line"><span>model</span><code>native-id@7f2c</code></div>
      <div className="session-line"><span>workspace</span><code>~/Projects/example</code></div>
      <div className="session-stream">
        <span>session ready, grant verified</span>
        <span>turn started, owned cancellation</span>
        <span>native approval requested once</span>
      </div>
    </div>
  );
}

const VISUALS = [DiscoverVisual, GrantVisual, SessionVisual];

export function HowItWorks() {
  const [active, setActive] = useState(0);
  const sectionRef = useRef<HTMLElement>(null);
  const manualUntil = useRef(0);

  useEffect(() => {
    let frame = 0;
    const onScroll = () => {
      if (frame) return;
      frame = window.requestAnimationFrame(() => {
        frame = 0;
        if (performance.now() < manualUntil.current) return;
        const element = sectionRef.current;
        if (!element) return;
        const rect = element.getBoundingClientRect();
        const viewport = window.innerHeight;
        if (rect.bottom < viewport * 0.25 || rect.top > viewport * 0.75) return;
        const progress = Math.min(1, Math.max(0, (viewport * 0.62 - rect.top) / Math.max(1, rect.height - viewport * 0.35)));
        setActive(Math.min(2, Math.floor(progress * 3)));
      });
    };
    window.addEventListener("scroll", onScroll, { passive: true });
    onScroll();
    return () => {
      window.removeEventListener("scroll", onScroll);
      if (frame) window.cancelAnimationFrame(frame);
    };
  }, []);

  function select(index: number) {
    manualUntil.current = performance.now() + 4500;
    setActive(index);
  }

  return (
    <section className="section how" id="how" aria-labelledby="how-title" ref={sectionRef}>
      <div className="container">
        <div className="head-split">
          <h2 id="how-title">Discover, grant, session. The same three steps for every client.</h2>
          <p className="lead">
            Nothing is automatic except the safe part. The runtime stages discovery, apps ask for a
            scoped grant, and execution only starts inside a session the host owns.
          </p>
        </div>

        <div className="how-grid">
          <ol className="how-steps" role="tablist" aria-label="How OriginLM works" aria-orientation="vertical">
            {STEPS.map((step, index) => (
              <li key={step.id} role="presentation">
                <button
                  type="button"
                  role="tab"
                  className="how-step"
                  aria-selected={active === index}
                  aria-controls={`how-panel-${step.id}`}
                  id={`how-tab-${step.id}`}
                  data-active={active === index}
                  onClick={() => select(index)}
                >
                  <span className="how-step-text">
                    <h3>{step.title}</h3>
                    <p>{step.body}</p>
                  </span>
                </button>
              </li>
            ))}
          </ol>

          <div className="how-panels">
            {STEPS.map((step, index) => {
              const Visual = VISUALS[index];
              return (
                <div
                  className="how-panel"
                  key={step.id}
                  id={`how-panel-${step.id}`}
                  role="tabpanel"
                  aria-labelledby={`how-tab-${step.id}`}
                  data-active={active === index}
                  hidden={active !== index}
                >
                  <div className="how-visual">
                    <Visual />
                  </div>
                  <CodeBlock code={step.code} title={`${step.id}.ts`} />
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </section>
  );
}
