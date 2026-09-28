// Test-only browser host; never used by the shipped live/HTTP entry point.
import '../src/styles.css';
import { createRoot } from 'react-dom/client';
import { createLocalAI, LocalAIError } from '@local-ai-kit/sdk';
import { App } from '../src/components/app';
import { fixtureSnapshot } from '../fixture/snapshot.mjs';
import { createFixtureService } from '../fixture/service.mjs';
import { fixtureTransport } from '../fixture/transport.mjs';

(globalThis as any).mountTestApp = async (options: any = {}) => {
  const snapshot: any = fixtureSnapshot();
  if (options.maliciousLabel) snapshot.models[0].name = '<img src=x onerror="globalThis.__unsafeExecuted=true">';
  if (options.largeCatalog) {
    const models = snapshot.models;
    snapshot.models = Array.from({ length: 5000 }, (_, i) => ({
      ...models[i % models.length],
      id: 'huge-' + i,
      name: 'Catalog Model ' + i,
      nativeId: 'fixture/huge-' + i,
    }));
  }
  const connectFixture = async () =>
    createLocalAI({ transport: fixtureTransport((createFixtureService as any)({ snapshot, turnError: options.turnError ?? null })) });
  let initialClient: Awaited<ReturnType<typeof createLocalAI>> | null = null;
  if (options.nativeResumption) {
    (globalThis as any).testNativeCalls = [];
    const service = (createFixtureService as any)({ snapshot });
    const transport: any = fixtureTransport(service);
    const request = transport.request.bind(transport);
    let previousSession: any = null;
    transport.request = async (method: string, params: any = {}, call: any = {}) => {
      if (['sessions.create', 'sessions.close', 'turns.start'].includes(method)) (globalThis as any).testNativeCalls.push(method);
      if (method === 'runtime.handshake') {
        const handshake = await request(method, params, call);
        return { ...handshake, methods: [...handshake.methods, 'sessions.suspend', 'sessions.resume'] };
      }
      if (method === 'sessions.create') {
        previousSession = { ...(await request(method, params, call)), nativeResumption: true };
        return previousSession;
      }
      if (method === 'sessions.suspend') return { resumeToken: 'test-only-resume-token-sentinel' };
      if (method === 'sessions.resume') return { ...previousSession, state: 'ready' };
      return request(method, params, call);
    };
    initialClient = await createLocalAI({ transport });
  }
  createRoot(document.querySelector('#app') as HTMLElement).render(
    <App
      connectFixture={connectFixture}
      connectLive={async () => {
        throw new LocalAIError('TRANSPORT_FAILED');
      }}
      initialClient={initialClient}
      initialMode="synthetic"
    />,
  );
};
