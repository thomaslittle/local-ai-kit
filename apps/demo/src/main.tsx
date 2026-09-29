import './styles.css';
import { createRoot } from 'react-dom/client';
import { LocalAIError, type LocalAIClient } from '@local-ai-kit/sdk';
import { App } from '@/components/app';
import { Pairing } from '@/components/pairing';
import { connectRuntime } from '@/lib/runtime';
import { applyStoredTheme } from '@/lib/theme';
import { createAppRouter, warmRoutes } from '@/router';
import { pathFor } from '@/lib/pages';
import { createDemoStore } from '@/state/store';
import type { FixtureFlags } from '@/state/types';

function fixtureFlagsFromParams(params: URLSearchParams): FixtureFlags {
  const mode = params.get('mode');
  return {
    staleAuth: params.has('staleAuth'),
    extraProfiles: params.has('extraProfiles'),
    forceMode: mode === 'live' || mode === 'synthetic' ? mode : undefined,
    probeResults: params.has('probeResults'),
  };
}

async function fetchJson(path: string): Promise<Record<string, unknown> | null> {
  const response = await fetch(path, { credentials: 'omit', cache: 'no-store', redirect: 'error' });
  return response.ok ? ((await response.json()) as Record<string, unknown>) : null;
}

function showUnavailable(): void {
  const status = document.createElement('p');
  status.textContent =
    'The native runtime is unavailable. Start the Local AI Kit demo command and reload this page. No example data has been substituted.';
  status.setAttribute('role', 'alert');
  status.className = 'm-8 text-muted-foreground';
  document.querySelector('#app')?.replaceChildren(status);
}

type Root = ReturnType<typeof createRoot>;

/** Loopback synthetic fixture: every record comes from the fixture service, never from the browser. */
async function bootFixture(root: Root, params: URLSearchParams): Promise<void> {
  const bootstrap = await fetchJson('/fixture/bootstrap');
  if (!bootstrap) throw new Error('RUNTIME_UNAVAILABLE');
  const client = await connectRuntime(window.location.origin, String(bootstrap.sessionToken));
  const router = createAppRouter();
  const demo = createDemoStore({
    navigate: (page) => void router.navigate({ to: pathFor(page) }),
    onBulkProbe: params.has('bulkProbeHook') ? () => void (document.body.dataset.bulkProbe = 'started') : undefined,
    connectFixture: async () => client,
    connectLive: async () => {
      throw new LocalAIError('TRANSPORT_FAILED');
    },
    initialClient: client,
    initialMode: 'synthetic',
    // Scripted scenarios are opt-in through the hash fragment (`#demo-fixture&staleAuth=1`).
    fixtureFlags: params.has('demo-fixture') ? fixtureFlagsFromParams(params) : undefined,
  });
  root.render(<App demo={demo} router={router} />);
  warmRoutes(router);
}

function bootPairing(root: Root): void {
  root.render(
    <Pairing
      onPaired={(client: LocalAIClient) => {
        const router = createAppRouter();
        const demo = createDemoStore({
          navigate: (page) => void router.navigate({ to: pathFor(page) }),
          connectFixture: async () => client,
          connectLive: connectRuntime,
          initialClient: client,
          initialMode: 'live',
        });
        root.render(<App demo={demo} router={router} />);
        warmRoutes(router);
      }}
    />,
  );
}

async function boot(): Promise<void> {
  applyStoredTheme();
  const root = createRoot(document.querySelector('#app') ?? document.body);
  // Scripted flags live in the hash fragment so they never reach the server:
  // asset routes reject query strings on every path except /v1/events.
  const params = new URLSearchParams(window.location.hash.replace(/^#/, ''));

  // The host says what it is. The fixture server announces `synthetic`; the native helper announces `live`
  // and requires pairing. `#demo-fixture` still forces the fixture for scripted checks and `npm run dev`.
  const info = await fetchJson('/runtime/info');
  if (params.has('demo-fixture') || info?.mode === 'synthetic') return bootFixture(root, params);
  if (!info) throw new Error('RUNTIME_UNAVAILABLE');
  if (info.mode !== 'live' || info.pairingRequired !== true) throw new Error('INVALID_RUNTIME');
  bootPairing(root);
}

boot().catch(showUnavailable);
