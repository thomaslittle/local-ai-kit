import { useState } from "react";
import { Activity, Bell, Gauge, Terminal } from "../components/Icon";
import mark from "../assets/brand/originlm-mark-small-white.svg";

const SURFACES = [
  { icon: Activity, title: "SDK badge", body: "ai.status() returns mode, providers and connected apps." },
  { icon: Gauge, title: "Tray or menu bar", body: "Connected apps and models, Pause, Open Control Center. No popups." },
  { icon: Bell, title: "Notifications", body: "Only decisions the user must make: a new app, a signed-out provider." },
  { icon: Terminal, title: "CLI", body: "The same status in the terminal with originlm status." },
];

export function StatusDemo() {
  const [connected, setConnected] = useState(true);

  return (
    <section className="section" aria-labelledby="status-title">
      <div className="container">
        <div className="section-head reveal">
          <span className="eyebrow">Status, unobtrusive</span>
          <h2 id="status-title">One badge, four surfaces, one source of truth.</h2>
          <p className="lede">
            Apps can show whether local AI is available without a modal, a permission wall or a settings
            screen. The user decides what runs, and the tray is where they change their mind.
          </p>
        </div>

        <div className="status-layout">
          <div className="status-demo reveal">
            <div className="status-copy">
              <h3>Try the badge</h3>
              <p>
                In a real app this is rendered from <code>ai.status()</code>. The fallback state links to
                the installer instead of failing silently.
              </p>
              <div className="status-toggle" role="group" aria-label="Demo connection state">
                <button type="button" aria-pressed={connected} onClick={() => setConnected(true)}>Connected</button>
                <button type="button" aria-pressed={!connected} onClick={() => setConnected(false)}>Not detected</button>
              </div>
            </div>

            <div className="badge-stage">
              {connected ? (
                <p className="connected-badge" data-state="connected">
                  <span className="pulse-dot" />
                  <strong>Local AI connected</strong>
                  <span>Claude, Codex, Ollama</span>
                </p>
              ) : (
                <p className="connected-badge" data-state="disconnected">
                  <span className="pulse-dot" />
                  <strong>Not detected</strong>
                  <a href="#install">Install OriginLM</a>
                </p>
              )}
            </div>

            <ul className="status-surface-list">
              {SURFACES.map(({ icon: Icon, title, body }) => (
                <li className="status-surface" key={title}>
                  <Icon size={17} />
                  <span><b>{title}</b> — {body}</span>
                </li>
              ))}
            </ul>
          </div>

          <div className="reveal" aria-label="Tray menu mockup">
            <div className="tray">
              <div className="tray-head">
                <img src={mark} alt="" width={16} height={16} />
                OriginLM
                <span className="tray-status">running</span>
              </div>
              <div className="tray-section">
                <h4>Connected apps</h4>
                <div className="tray-item"><span className="swatch" />Acme Studio<small>usage · sessions</small></div>
                <div className="tray-item"><span className="swatch" />Editor Bridge<small>models</small></div>
                <div className="tray-item" data-muted="true"><span className="swatch" />Old Prototype<small>revoked</small></div>
              </div>
              <div className="tray-section">
                <h4>Models available</h4>
                <div className="tray-item"><span className="swatch" />Claude Code<small>subscription</small></div>
                <div className="tray-item"><span className="swatch" />Codex<small>subscription</small></div>
                <div className="tray-item"><span className="swatch" />Ollama · qwen<small>local</small></div>
              </div>
              <div className="tray-section">
                <h4>Today</h4>
                <div className="tray-item"><span className="swatch" />128 turns<small>2 issues</small></div>
              </div>
              <div className="tray-actions">
                <button type="button">Pause</button>
                <button type="button">Open Control Center</button>
              </div>
            </div>
            <p className="compare-note" style={{ marginTop: "0.9rem" }}>
              <Gauge size={14} />
              <span>Mockup. The real tray is on by default and easy to disable.</span>
            </p>
          </div>
        </div>
      </div>
    </section>
  );
}
