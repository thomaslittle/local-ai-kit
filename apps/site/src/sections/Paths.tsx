import { LayeredScreens } from "../components/LayeredScreens";
import { CheckCircle } from "../components/Icon";
import { CodeBlock } from "../components/CodeBlock";

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
    <div className="container">
      <div className="head-split">
        <h2 id="paths-title">One runtime. A dashboard for you, an SDK for your app.</h2>
        <p className="lead">
          Most tools pick a side: a usage view with no API, or an AI SDK with no idea what the user
          already pays for. OriginLM is both, because both read the same local state.
        </p>
      </div>

      <div className="halves">
        <article className="half half--see" aria-labelledby="see-title">
          <div className="half-copy">
            <h3 id="see-title">See your AI</h3>
            <p>
              <strong>For everyone who uses AI.</strong> A local control center for every client, model,
              account and subscription on your machine. No more guessing which plan you are on or how close
              you are to a limit.
            </p>
            <ul className="ticks">
              <li><CheckCircle size={16} /> Usage and quotas with reset times and pace</li>
              <li><CheckCircle size={16} /> Models per route, subscription or metered</li>
              <li><CheckCircle size={16} /> Cost and usage trends, plus issues that need attention</li>
            </ul>
          </div>
          <LayeredScreens />
        </article>

        <article className="half half--build" id="developers" aria-labelledby="build-title">
          <div className="half-copy">
            <h3 id="build-title">Build with it</h3>
            <p>
              <strong>For developers.</strong> Put the user’s own AI in your app with one dependency. No
              provider SDKs, no API keys to collect, no setup wizard to rebuild for every client.
            </p>
            <ul className="ticks">
              <li><CheckCircle size={16} /> Typed SDKs for Rust, npm and Python</li>
              <li><CheckCircle size={16} /> Imports never scan, launch or read credentials</li>
              <li><CheckCircle size={16} /> Scoped, revocable grants for every app</li>
            </ul>
          </div>
          <CodeBlock code={BUILD_SNIPPET} title="your-app.ts" className="half-code" />
          <p className="shot-caption">
            <a className="text-link" href="#integrations">Framework-by-framework guides</a>
          </p>
        </article>
      </div>
    </div>
  );
}
