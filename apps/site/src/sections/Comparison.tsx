import { CheckCircle } from "../components/Icon";

const ROWS = [
  {
    label: "Providers and accounts",
    widget: "One provider, one account, whichever the widget was built for.",
    origin: "Every detected client, profile and account on the machine, kept as separate identities.",
  },
  {
    label: "Usage and quotas",
    widget: "A number, often scraped or estimated from the last request.",
    origin: "Provider-reported windows with freshness, reset times and pace. Unknown stays unknown.",
  },
  {
    label: "Models and routes",
    widget: "A static list, if there is one at all.",
    origin: "Models per route, with subscription vs metered and capability evidence attached.",
  },
  {
    label: "History and issues",
    widget: "Today only.",
    origin: "Cost and usage trends, plus issues like a signed-out provider or a nearly full quota.",
  },
  {
    label: "Where it runs",
    widget: "Usually a hosted service that receives metadata or an API key.",
    origin: "Loopback only. No telemetry, no relay, no secrets exported by the SDK.",
  },
  {
    label: "For developers",
    widget: "Usually nothing. The widget is the end of the line.",
    origin: "Rust, npm and Python SDKs over the same runtime the dashboard reads.",
  },
  {
    label: "Permissions",
    widget: "All-or-nothing access to an account token.",
    origin: "Per-app grants scoped to routes and workspaces, revocable at any time.",
  },
];

export function Comparison() {
  return (
    <div className="container compare-block">
      <div className="head-split head-split--tight">
        <h2 id="compare-title">Everyone builds one-off usage widgets. None are also a full dev platform.</h2>
        <p className="lead">
          Tracking AI usage is becoming a feature in every client. What is missing is the layer underneath:
          one local runtime that both reads the truth and lets applications act on it.
        </p>
      </div>

      <div className="compare-scroll" role="region" aria-label="Comparison table" tabIndex={0}>
        <table className="compare">
          <caption className="visually-hidden">
            Single-provider usage widgets compared with OriginLM across seven capabilities.
          </caption>
          <thead>
            <tr>
              <th scope="col">Capability</th>
              <th scope="col">A single-provider usage widget</th>
              <th scope="col" className="compare-us">OriginLM</th>
            </tr>
          </thead>
          <tbody>
            {ROWS.map((row) => (
              <tr key={row.label}>
                <th scope="row">{row.label}</th>
                <td>{row.widget}</td>
                <td className="compare-us">
                  <span className="yes"><CheckCircle size={16} />{row.origin}</span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <p className="fine-print">
        The left column describes the product category in general, not a named product. OriginLM claims
        follow the <a href="https://github.com/thomaslittle/local-ai-kit/blob/main/docs/adapters/SUPPORT-MATRIX.md">support matrix</a>,
        including where a provider reports nothing and we show unknown rather than a guess.
      </p>
    </div>
  );
}
