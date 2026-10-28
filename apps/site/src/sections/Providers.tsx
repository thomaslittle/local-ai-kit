import { useMemo, useState } from "react";
import { Compass } from "../components/Icon";
import { OP_LABELS, PROVIDERS, type ProviderEntry, type ProviderOps } from "../data/providers";

const FILTERS = [
  { id: "all", label: "All providers" },
  { id: "Native client", label: "Native clients" },
  { id: "Local runtime", label: "Local runtimes" },
  { id: "API route", label: "API routes" },
] as const;

const STATE_LABEL: Record<ProviderOps[keyof ProviderOps], string> = {
  yes: "supported",
  partial: "conditional",
  none: "not supported",
};

function ProviderCard({ provider }: { provider: ProviderEntry }) {
  return (
    <article className="provider-card">
      <div className="provider-top">
        <span className="provider-glyph" aria-hidden="true">{provider.glyph}</span>
        <div>
          <h3>{provider.name}</h3>
          <span className="provider-kind">{provider.kind}</span>
        </div>
      </div>
      <div className="provider-ops" role="list" aria-label={`${provider.name} support`}>
        {OP_LABELS.map((operation) => {
          const state = provider.ops[operation.key];
          return (
            <span
              key={operation.key}
              className="op"
              data-state={state}
              role="listitem"
              title={`${operation.label}: ${STATE_LABEL[state]}`}
            >
              {operation.label}
              <span className="visually-hidden">: {STATE_LABEL[state]}</span>
            </span>
          );
        })}
      </div>
      <details>
        <summary>Evidence and limits</summary>
        <p>{provider.note}</p>
      </details>
    </article>
  );
}

export function Providers() {
  const [filter, setFilter] = useState<(typeof FILTERS)[number]["id"]>("all");
  const visible = useMemo(
    () => (filter === "all" ? PROVIDERS : PROVIDERS.filter((provider) => provider.kind === filter)),
    [filter],
  );

  return (
    <section className="section" id="providers" aria-labelledby="providers-title">
      <div className="container">
        <div className="section-head section-head--wide reveal">
          <span className="eyebrow">Supported providers</span>
          <h2 id="providers-title">The AI already on the machine, with the truth about what each one exposes.</h2>
          <p className="lede">
            Eighteen documented integrations. “Supported” does not mean every operation works everywhere:
            plan data can be missing, a client can refuse text-only sessions, and an offline local
            runtime is still detected. We show the gaps instead of filling them in.
          </p>
        </div>

        <div className="provider-toolbar reveal">
          <div className="filter-group" role="group" aria-label="Filter providers">
            {FILTERS.map((item) => (
              <button
                key={item.id}
                type="button"
                className="filter-chip"
                aria-pressed={filter === item.id}
                onClick={() => setFilter(item.id)}
              >
                {item.label}
              </button>
            ))}
          </div>
          <span className="provider-count" aria-live="polite">
            {visible.length} of {PROVIDERS.length}
          </span>
        </div>

        <div className="provider-grid reveal">
          {visible.map((provider) => <ProviderCard key={provider.id} provider={provider} />)}
        </div>

        <div className="provider-legend">
          <span><i className="yes" /> supported</span>
          <span><i className="partial" /> conditional, fixture-tested, or gated by a reported capability</span>
          <span><i /> not supported or not reported</span>
        </div>

        <p className="compare-note">
          <Compass size={14} />
          <span>
            Condensed from the{" "}
            <a href="https://github.com/thomaslittle/local-ai-kit/blob/main/docs/adapters/SUPPORT-MATRIX.md">provider support matrix</a>.
            Evidence and verification follow each field; unknown is never rendered as zero, and passive
            detection is not execution.
          </span>
        </p>
      </div>
    </section>
  );
}
