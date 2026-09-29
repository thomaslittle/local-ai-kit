/**
 * Demo dev server: Vite with hot module replacement in front of the synthetic
 * fixture service. No account, credential, keyring, provider client, or network
 * call is involved; the fixture is the same loopback service the tests use.
 *
 *   npm run dev          -> http://localhost:5173/#demo-fixture
 */
import { createServer } from 'vite';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createFixtureServer } from '../apps/demo/server.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const fixture = await createFixtureServer({ port: 0 });

// The fixture rejects unknown origins by design, so the proxy presents itself as the fixture.
const toFixture = {
  target: fixture.url,
  changeOrigin: true,
  configure: (proxy) => proxy.on('proxyReq', (request) => request.setHeader('origin', fixture.url)),
};

const portFlag = process.argv.indexOf('--port');
const vite = await createServer({
  configFile: resolve(root, 'apps/demo/vite.config.ts'),
  server: {
    port: portFlag < 0 ? 5173 : Number(process.argv[portFlag + 1]),
    host: '127.0.0.1',
    proxy: { '/fixture': toFixture, '/v1': toFixture },
  },
});
await vite.listen();
vite.printUrls();
console.log('Open the synthetic showcase at: ' + vite.resolvedUrls.local[0] + '#demo-fixture');

for (const signal of ['SIGINT', 'SIGTERM']) {
  process.once(signal, async () => {
    await vite.close();
    await fixture.close();
    process.exit(0);
  });
}
