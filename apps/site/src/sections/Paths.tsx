import { ArrowRight, CheckCircle } from "../components/Icon";
import { CodeBlock } from "../components/CodeBlock";
import { DashboardMock } from "../components/DashboardMock";

const BUILD_SNIPPET = `import { createLocalAI } from "@local-ai-kit/sdk";

const ai = await createLocalAI({ transport });
const snapshot = await ai.snapshot.get();

// The same state the dashboard reads.
const { providers, models, quotaPools } = snapshot;

// Nothing runs until the host asks, with a grant.
const session = await ai.sessions.create({
  selection: { routeId, modelId, revision },
  workspaceId,
});`;

export function Paths() {
  return (
    <section className="section paths" aria-labelledby="paths-title">
      <div className="container">
        <div className="paths-head reveal">
          <div>
            <span className="eyebrow">Two halves, equal weight</span>
            <h2 id="paths-title">One runtime. A dashboard for you, an SDK for your app.</h2>
          </div>
          <p>
            Most tools pick a side: a usage view with no API, or an AI SDK with no idea what the user
            already pays for. OriginLM is both, because both read the same local state.
          </p>
        </div>

        <div className="paths-grid">
          <article className="path-card path-card--see reveal">
            <div className="path-card-body">
              <span className="path-kicker">For everyone who uses AI</span>
              <h3>See your AI</h3>
              <p>
                A local control center for every client, model, account and subscription on your machine.
                No more guessing which plan you are on or how close you are to a limit.
              </p>
              <ul className="path-points">
                <li><CheckCircle size={15} /> Usage and quotas with reset times and pace</li>
                <li><CheckCircle size={15} /> Models per route — subscription or metered</li>
                <li><CheckCircle size={15} /> Cost and usage trends, plus issues that need attention</li>
              </ul>
            </div>
            <div className="path-visual">
              <DashboardMock />
            </div>
          </article>

          <article className="path-card reveal" id="developers">
            <div className="path-card-body">
              <span className="path-kicker">For developers</span>
              <h3>Build with it</h3>
              <p>
                Put the user's own AI in your app with one dependency. No provider SDKs, no API keys to
                collect, no setup wizard to rebuild for every client.
              </p>
              <ul className="path-points">
                <li><CheckCircle size={15} /> Typed SDKs for Rust, npm and Python</li>
                <li><CheckCircle size={15} /> Imports never scan, launch or read credentials</li>
                <li><CheckCircle size={15} /> Scoped, revocable grants for every app</li>
              </ul>
            </div>
            <div className="path-visual path-code">
              <CodeBlock code={BUILD_SNIPPET} title="your app · one dependency" />
            </div>
            <a className="path-code-note" href="#integrations">
              Framework-by-framework guides <ArrowRight size={13} />
            </a>
          </article>
        </div>
      </div>
    </section>
  );
}
