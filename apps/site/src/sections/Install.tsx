import { useId, useRef, useState } from "react";
import { CopyButton } from "../components/CopyButton";
import { Plug, Terminal } from "../components/Icon";

type Track = "app" | "project";

type Command = {
  id: string;
  label: string;
  command: string;
  note?: string;
};

const TRACKS: Record<Track, { title: string; blurb: string; badge: string; commands: Command[]; foot: string }> = {
  app: {
    title: "Get the app",
    blurb: "The control center, the tray, and the service every SDK finds.",
    badge: "user service",
    commands: [
      {
        id: "unix",
        label: "macOS / Linux",
        command: "curl -fsSL https://originlm.dev/install.sh | sh",
        note: "launchd LaunchAgent on macOS · systemd --user unit on Linux",
      },
      {
        id: "windows",
        label: "Windows",
        command: "irm https://originlm.dev/install.ps1 | iex",
        note: "Task Scheduler logon task · no admin rights",
      },
    ],
    foot: "Installs one binary into a user-level location and registers a user service. No admin rights, no system-wide paths. Uninstall stops and removes the unit, binary and state descriptor.",
  },
  project: {
    title: "Add to your project",
    blurb: "The SDK speaks to the service, or bundles the helper itself.",
    badge: "pre-release",
    commands: [
      { id: "npm", label: "npm", command: "npm install @local-ai-kit/sdk" },
      { id: "pnpm", label: "pnpm", command: "pnpm add @local-ai-kit/sdk" },
      { id: "bun", label: "bun", command: "bun add @local-ai-kit/sdk" },
      { id: "yarn", label: "yarn", command: "yarn add @local-ai-kit/sdk" },
      { id: "cargo", label: "cargo", command: "cargo add local-ai-kit" },
      { id: "pip", label: "pip", command: "pip install local-ai-kit-preview" },
    ],
    foot: "Packages are pre-release: install the SDK tarball from a release bundle or build from source until the first public release. Cargo links the runtime; npm and Python clients speak the local protocol.",
  },
};

export function Install() {
  const [track, setTrack] = useState<Track>("app");
  const [tab, setTab] = useState("unix");
  const trackId = useId();
  const tabRefs = useRef<Record<string, HTMLButtonElement | null>>({});
  const data = TRACKS[track];
  const current = data.commands.find((command) => command.id === tab) ?? data.commands[0];

  function chooseTrack(next: Track) {
    setTrack(next);
    setTab(next === "app" ? "unix" : "npm");
  }

  function onTabKey(event: React.KeyboardEvent, index: number) {
    const keys = ["ArrowRight", "ArrowLeft", "Home", "End"];
    if (!keys.includes(event.key)) return;
    event.preventDefault();
    const length = data.commands.length;
    const next =
      event.key === "Home" ? 0
      : event.key === "End" ? length - 1
      : event.key === "ArrowRight" ? (index + 1) % length
      : (index - 1 + length) % length;
    setTab(data.commands[next].id);
    tabRefs.current[data.commands[next].id]?.focus();
  }

  return (
    <section className="section" id="install" aria-labelledby="install-title">
      <div className="container">
        <div className="section-head reveal">
          <span className="eyebrow">Install</span>
          <h2 id="install-title">Two tracks. One runtime.</h2>
          <p className="lede">
            Install it for the dashboard and every app built on it just works. Add it to a project and
            the user's existing AI clients become your backend.
          </p>
        </div>

        <div className="install-shell">
          <div className="install-tracks reveal" role="tablist" aria-label="Install track" aria-orientation="vertical">
            {(Object.keys(TRACKS) as Track[]).map((key) => {
              const item = TRACKS[key];
              return (
                <button
                  key={key}
                  type="button"
                  role="tab"
                  id={`${trackId}-${key}`}
                  aria-selected={track === key}
                  aria-controls={`${trackId}-panel`}
                  tabIndex={track === key ? 0 : -1}
                  className="track-card"
                  onClick={() => chooseTrack(key)}
                  onKeyDown={(event) => {
                    if (event.key === "ArrowDown" || event.key === "ArrowUp") {
                      event.preventDefault();
                      chooseTrack(key === "app" ? "project" : "app");
                      document.getElementById(`${trackId}-${key === "app" ? "project" : "app"}`)?.focus();
                    }
                  }}
                >
                  <span className="track-card-top">
                    <strong>{item.title}</strong>
                    {key === "app" ? <Plug size={17} /> : <Terminal size={17} />}
                  </span>
                  <p>{item.blurb}</p>
                  <code>{key === "app" ? "curl · powershell" : "npm · pnpm · bun · yarn · cargo · pip"}</code>
                </button>
              );
            })}
          </div>

          <div
            className="install-panel reveal"
            role="tabpanel"
            id={`${trackId}-panel`}
            aria-labelledby={`${trackId}-${track}`}
          >
            <div className="install-panel-head">
              <div>
                <h3>{data.title}</h3>
                <p>{data.blurb}</p>
              </div>
              <span className="install-badge">{data.badge}</span>
            </div>

            <div className="install-tabs" role="tablist" aria-label={`${data.title} packages`}>
              {data.commands.map((command, index) => (
                <button
                  key={command.id}
                  ref={(node) => { tabRefs.current[command.id] = node; }}
                  type="button"
                  role="tab"
                  className="install-tab"
                  aria-selected={current.id === command.id}
                  tabIndex={current.id === command.id ? 0 : -1}
                  onClick={() => setTab(command.id)}
                  onKeyDown={(event) => onTabKey(event, index)}
                >
                  {command.label}
                </button>
              ))}
            </div>

            <div className="install-cmd">
              <span className="prompt" aria-hidden="true">$</span>
              <code>{current.command}</code>
              <CopyButton value={current.command} label="Copy" className="copy-btn--lg" />
            </div>

            <p className="install-foot">
              {current.note ? <><strong>{current.note}.</strong> </> : null}
              {data.foot}
            </p>
          </div>
        </div>
      </div>
    </section>
  );
}
