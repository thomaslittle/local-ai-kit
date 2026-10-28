import { useState } from "react";
import { Activity, Bell, CheckCircle, Gauge, Terminal } from "../components/Icon";
import mark from "../assets/brand/originlm-mark-small-white.svg";

const SURFACES = [
  { icon: Activity, title: "SDK badge", body: "The Rust facade exposes mode, providers and model counts." },
  { icon: Gauge, title: "Tray or menu bar", body: "Planned: connected apps, models and a pause control." },
  { icon: Bell, title: "Notifications", body: "The runtime records issues; OS notification delivery is not qualified." },
  { icon: Terminal, title: "CLI", body: "Inspect the runtime through the local-ai-kit-control terminal app." },
];

export function StatusDemo() {
  const [connected, setConnected] = useState(true);

  return (
    <div className="container status" aria-labelledby="status-title" role="group">
      <div className="head-split">
        <h2 id="status-title">One badge, four surfaces, one source of truth.</h2>
        <p className="lead">
          Apps can show whether local AI is available without a modal, a permission wall or a settings
          screen. The user decides what runs through explicit grants. Tray integration is planned.
        </p>
      </div>

      <div className="status-layout">
        <div className="status-demo">
          <h3>Try the badge</h3>
          <p>
            This interactive example illustrates a consumer status badge. The fallback state links to
            the installer instead of failing silently.
          </p>
          <div className="segmented" role="group" aria-label="Demo connection state">
            <button type="button" aria-pressed={connected} onClick={() => setConnected(true)}>Connected</button>
            <button type="button" aria-pressed={!connected} onClick={() => setConnected(false)}>Not detected</button>
          </div>

          <div className="badge-stage">
            {connected ? (
              <p className="connected-badge" data-state="connected" role="status">
                <CheckCircle size={16} />
                <strong>Local AI connected</strong>
                <span>Claude, Codex, Ollama</span>
              </p>
            ) : (
              <p className="connected-badge" data-state="disconnected" role="status">
                <strong>Not detected</strong>
                <a href="#install">Install OriginLM</a>
              </p>
            )}
          </div>

          <dl className="surfaces">
            {SURFACES.map(({ icon: Icon, title, body }) => (
              <div className="surface" key={title}>
                <dt><Icon size={17} /> {title}</dt>
                <dd>{body}</dd>
              </div>
            ))}
          </dl>
        </div>

        <figure className="tray-figure" aria-label="Tray menu mockup" data-slot="tray">
          <div className="tray">
            <div className="tray-head">
              <img src={mark} alt="" width={16} height={16} />
              OriginLM
              <span className="tray-status">running</span>
            </div>
            <div className="tray-section">
              <h4>Connected apps</h4>
              <div className="tray-item">Example Studio<small>usage, sessions</small></div>
              <div className="tray-item">Editor Bridge<small>models</small></div>
              <div className="tray-item" data-muted="true">Old Prototype<small>revoked</small></div>
            </div>
            <div className="tray-section">
              <h4>Models available</h4>
              <div className="tray-item">Claude Code<small>subscription</small></div>
              <div className="tray-item">Codex<small>subscription</small></div>
              <div className="tray-item">Ollama qwen<small>local</small></div>
            </div>
            <div className="tray-section">
              <h4>Today</h4>
              <div className="tray-item">128 turns<small>2 issues</small></div>
            </div>
            <div className="tray-actions">
              <button type="button" disabled>Pause (preview)</button>
              <button type="button" disabled>Open Control Center (preview)</button>
            </div>
          </div>
          <figcaption>Planned tray interface with example values. This is not a shipped system tray.</figcaption>
        </figure>
      </div>
    </div>
  );
}
