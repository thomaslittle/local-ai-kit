import './styles.css';
import { createRoot } from 'react-dom/client';
import { createHttpTransport, createLocalAI, LocalAIError, type LocalAIClient } from '@local-ai-kit/sdk';
import { App } from './components/app';
import { Pairing } from './components/pairing';
import type { DemoFactory } from './state/demo';

async function connect(endpoint: string, token: string): Promise<LocalAIClient> {
  return createLocalAI({ clientName: 'local-ai-kit-showcase', transport: createHttpTransport({ baseUrl: endpoint, token }) });
}

function fixtureFlagsFromParams(params: URLSearchParams): DemoFactory['fixtureFlags'] {
  if (!params.has('demo-fixture')) return undefined;
  const mode = params.get('mode');
  return {
    staleAuth: params.has('staleAuth'),
    extraProfiles: params.has('extraProfiles'),
    forceMode: mode === 'live' || mode === 'synthetic' ? mode : undefined,
    probeResults: params.has('probeResults'),
    bulkProbeHook: params.has('bulkProbeHook'),
  };
}

async function boot(): Promise<void> {
  const container = document.querySelector('#app') ?? document.body;
  const root = createRoot(container);
  // Scripted flags live in the hash fragment so they never reach the server:
  // asset routes reject query strings on every path except /v1/events.
  const params = new URLSearchParams(window.location.hash.replace(/^#/, ''));

  if (params.has('demo-fixture')) {
    // Scripted HTTP fixture for automated checks. The browser never fabricates
    // accounts: every record comes from the loopback fixture service.
    const response = await fetch('/fixture/bootstrap', { credentials: 'omit', cache: 'no-store', redirect: 'error' });
    if (!response.ok) throw new Error('RUNTIME_UNAVAILABLE');
    const bootstrap = await response.json();
    const client = await connect(window.location.origin, bootstrap.sessionToken);
    const flags = fixtureFlagsFromParams(params);
    root.render(
      <App
        connectFixture={async () => client}
        connectLive={async () => {
          throw new LocalAIError('TRANSPORT_FAILED');
        }}
        initialClient={client}
        initialMode="synthetic"
        fixtureFlags={flags}
      />,
    );
    return;
  }

  const response = await fetch('/runtime/info', { credentials: 'omit', cache: 'no-store', redirect: 'error' });
  if (response.ok) {
    const info = await response.json();
    if (info.mode !== 'live' || info.pairingRequired !== true) throw new Error('INVALID_RUNTIME');
    root.render(
      <Pairing
        onPaired={(client) =>
          root.render(
            <App connectFixture={async () => client} connectLive={connect} initialClient={client} initialMode="live" />,
          )
        }
      />,
    );
    return;
  }
  throw new Error('RUNTIME_UNAVAILABLE');
}

boot().catch(() => {
  const status = document.createElement('p');
  status.textContent =
    'The native runtime is unavailable. Start the Local AI Kit demo command and reload this page. No example data has been substituted.';
  status.setAttribute('role', 'alert');
  document.querySelector('#app')?.replaceChildren(status);
});
