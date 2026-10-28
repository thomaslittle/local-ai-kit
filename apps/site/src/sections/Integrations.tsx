import { useRef, useState } from "react";
import { CodeBlock } from "../components/CodeBlock";
import { ArrowUpRight } from "../components/Icon";
import { INTEGRATIONS } from "../data/integrations";

export function Integrations() {
  const [active, setActive] = useState(INTEGRATIONS[0].id);
  const tabRefs = useRef<Record<string, HTMLButtonElement | null>>({});
  const current = INTEGRATIONS.find((item) => item.id === active) ?? INTEGRATIONS[0];
  const index = INTEGRATIONS.indexOf(current);

  function onKey(event: React.KeyboardEvent, position: number) {
    if (!["ArrowDown", "ArrowUp", "Home", "End"].includes(event.key)) return;
    event.preventDefault();
    const next =
      event.key === "Home" ? 0
      : event.key === "End" ? INTEGRATIONS.length - 1
      : event.key === "ArrowDown" ? (position + 1) % INTEGRATIONS.length
      : (position - 1 + INTEGRATIONS.length) % INTEGRATIONS.length;
    setActive(INTEGRATIONS[next].id);
    tabRefs.current[INTEGRATIONS[next].id]?.focus();
  }

  return (
    <section className="section" id="integrations" aria-labelledby="integrations-title">
      <div className="container">
        <div className="section-head section-head--wide reveal">
          <span className="eyebrow">Integrations</span>
          <h2 id="integrations-title">Real code for the framework you actually ship.</h2>
          <p className="lede">
            Every example talks to the same protocol. Pick your host: in-process Rust, a stdio helper,
            or the browser extension bridge. None of them read credentials on import.
          </p>
        </div>

        <div className="integrations reveal">
          <div className="integration-tabs" role="tablist" aria-label="Framework integrations" aria-orientation="vertical">
            {INTEGRATIONS.map((item, position) => (
              <button
                key={item.id}
                ref={(node) => { tabRefs.current[item.id] = node; }}
                type="button"
                role="tab"
                id={`int-tab-${item.id}`}
                aria-selected={active === item.id}
                aria-controls={`int-panel-${item.id}`}
                tabIndex={active === item.id ? 0 : -1}
                className="integration-tab"
                onClick={() => setActive(item.id)}
                onKeyDown={(event) => onKey(event, position)}
              >
                {item.name}
                <small>{item.tag}</small>
              </button>
            ))}
          </div>

          <div
            className="integration-panel"
            role="tabpanel"
            id={`int-panel-${current.id}`}
            aria-labelledby={`int-tab-${current.id}`}
            tabIndex={0}
          >
            <div className="integration-intro">
              <span className="eyebrow" style={{ marginBottom: "0.4rem" }}>{String(index + 1).padStart(2, "0")} / {String(INTEGRATIONS.length).padStart(2, "0")}</span>
              <h3>{current.title}</h3>
              <p>{current.description}</p>
              <div className="integration-meta">
                {current.meta.map((item) => <span className="chip" key={item}>{item}</span>)}
              </div>
            </div>
            <CodeBlock code={current.code} title={current.language} />
            <div className="integration-foot">
              <span>{current.note}</span>
              <a className="btn-link" href="https://github.com/thomaslittle/local-ai-kit/tree/main/examples">
                Examples <ArrowUpRight size={13} />
              </a>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
