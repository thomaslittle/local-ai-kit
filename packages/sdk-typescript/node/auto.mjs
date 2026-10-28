/** Dependency-first transport. Nothing spawns or connects until this is called. */
import {spawn} from 'node:child_process';
import {createRequire} from 'node:module';
import {accessSync, constants, closeSync, fstatSync, lstatSync, openSync, readFileSync, readSync, statSync} from 'node:fs';
import {homedir} from 'node:os';
import {isAbsolute, join} from 'node:path';
import {tmpdir} from 'node:os';
import {LocalAIError} from '../dist/errors.js';
import {openBrokerTransport} from './index.mjs';

const fail = code => new LocalAIError(code);
const require = createRequire(import.meta.url);
const PLATFORM_PACKAGES = {
  'linux-x64': '@local-ai-kit/cli-linux-x64',
  'linux-arm64': '@local-ai-kit/cli-linux-arm64',
  'darwin-arm64': '@local-ai-kit/cli-darwin-arm64',
  'darwin-x64': '@local-ai-kit/cli-darwin-x64',
  'win32-x64': '@local-ai-kit/cli-win32-x64',
};

/** The optional platform package for a target, or undefined when unsupported. */
export function platformPackage(platform = process.platform, arch = process.arch) {
  return PLATFORM_PACKAGES[`${platform}-${arch}`];
}

function usable(path) {
  if (typeof path !== 'string' || !isAbsolute(path) || path.includes('\0')) return false;
  if (/\.(cmd|bat|ps1)$/i.test(path)) return false;
  try {
    accessSync(path, constants.X_OK);
    return statSync(path).isFile();
  } catch {
    return false;
  }
}

function fromPlatformPackage(platform, arch) {
  const name = platformPackage(platform, arch);
  if (!name) return undefined;
  try {
    const manifestPath = require.resolve(`${name}/package.json`);
    const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'));
    const relative = typeof manifest?.bin === 'string' ? manifest.bin : manifest?.bin?.['local-ai-kit'];
    if (typeof relative !== 'string' || !relative || relative.includes('\0')) return undefined;
    const binary = join(manifestPath, '..', relative);
    return usable(binary) ? binary : undefined;
  } catch {
    return undefined;
  }
}

/**
 * The helper binary: explicit, then `LAI_HELPER_PATH`, then the packaged
 * platform binary. It never downloads anything.
 */
export function resolveHelper(options = {}) {
  const explicit = options.executable;
  if (explicit !== undefined && !usable(explicit)) throw fail('INVALID_EXECUTABLE');
  if (usable(explicit)) return explicit;
  const fromEnv = process.env.LAI_HELPER_PATH;
  if (fromEnv !== undefined && !usable(fromEnv)) throw fail('INVALID_EXECUTABLE');
  if (usable(fromEnv)) return fromEnv;
  const packaged = fromPlatformPackage();
  if (packaged) return packaged;
  throw fail('RUNTIME_MISSING');
}

/** The per-user shared broker address used by a dependency-first consumer. */
export function defaultAddress() {
  const configured = process.env.LAI_BROKER_ADDRESS;
  if (configured) return configured;
  if (process.platform === 'win32') return 'local-ai-kit-user';
  const runtime = process.env.XDG_RUNTIME_DIR;
  if (runtime && isAbsolute(runtime)) return join(runtime, 'local-ai-kit', 'user.sock');
  return join(homedir(), '.local-ai-kit', 'user.sock');
}

/** The grant descriptor this application was given for the shared broker. */
export function defaultTokenFile() {
  return process.env.LAI_BROKER_TOKEN_FILE ?? join(homedir(), '.local-ai-kit', 'broker-token.json');
}

function sharedToken(address) {
  try {
    const path = defaultTokenFile();
    const before = lstatSync(path);
    if (!before.isFile() || before.isSymbolicLink()) return null;
    const fd = openSync(path, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0) | (constants.O_NONBLOCK ?? 0));
    let descriptor;
    try {
      const info = fstatSync(fd);
      if (!info.isFile() || info.size > 16384 || info.dev !== before.dev || info.ino !== before.ino) return null;
      if (process.platform !== 'win32' && (info.mode & 0o077) !== 0) return null;
      const bytes = Buffer.alloc(16385);
      let length = 0;
      while (length < bytes.length) {
        const count = readSync(fd, bytes, length, bytes.length - length, null);
        if (!count) break;
        length += count;
      }
      if (length > 16384) return null;
      descriptor = JSON.parse(bytes.subarray(0, length).toString('utf8'));
    } finally { closeSync(fd); }
    if (descriptor?.address !== undefined && descriptor.address !== address) return null;
    if (typeof descriptor?.token !== 'string' || !/^[0-9a-f]{64}$/.test(descriptor.token)) return null;
    return descriptor.token;
  } catch {
    return null;
  }
}

function uniqueAddress() {
  const suffix = `${process.pid}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}`;
  if (process.platform === 'win32') return `local-ai-kit-private-${suffix}`;
  // The broker binds and owner-restricts a filesystem socket; the temp
  // directory always exists and the name is unique per process.
  return join(tmpdir(), `local-ai-kit-${suffix}.sock`);
}

function probeArgs(options) {
  const args = ['serve', '--ipc', options.address];
  if (options.passive !== false) args.push('--allow-passive-discovery');
  for (const provider of options.probeProviders ?? []) {
    if (typeof provider !== 'string' || !/^[a-z0-9_]{1,32}$/.test(provider)) throw fail('INVALID_REQUEST');
    args.push('--allow-metadata', provider);
  }
  return args;
}

/** Start a parent-owned broker and connect to it through the stdio bridge. */
async function openPrivateTransport(helper, options) {
  const address = uniqueAddress();
  const child = spawn(helper, probeArgs({...options, address}), {
    cwd: options.cwd ?? tmpdir(),
    env: options.env ? {...process.env, ...options.env} : process.env,
    shell: false,
    windowsHide: true,
    stdio: ['pipe', 'pipe', 'pipe'],
  });
  child.stderr.on('data', () => {});
  child.stdin.on('error', () => {});
  const exited = new Promise(resolve => {
    child.once('exit', resolve);
    child.once('error', resolve);
  });
  let stopping;
  const stop = () => stopping ??= (async () => {
    child.stdin.end();
    const killer = setTimeout(() => {
      if (child.exitCode === null && child.signalCode === null) child.kill('SIGKILL');
    }, 2000);
    try { await exited; } finally { clearTimeout(killer); }
  })();
  try {
    const descriptor = await new Promise((resolve, reject) => {
      let buffer = '';
      let settled = false;
      const timer = setTimeout(() => finish(fail('RUNTIME_MISSING')), options.waitMs ?? 8000);
      function finish(error, value) {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        child.stdout.off('data', onData);
        child.off('error', onError);
        child.off('exit', onError);
        if (error) reject(error); else resolve(value);
      }
      function onError() { finish(fail('RUNTIME_MISSING')); }
      function onData(chunk) {
        buffer += chunk.toString('utf8');
        if (buffer.length > 4096) return finish(fail('PROTOCOL_INVALID'));
        const end = buffer.indexOf('\n');
        if (end === -1) return;
        let value;
        try { value = JSON.parse(buffer.slice(0, end)); } catch { return finish(fail('PROTOCOL_INVALID')); }
        if (value?.protocolVersion !== '1.0' || value.address !== address || !/^[0-9a-f]{64}$/.test(value.token) || value.independent === true) {
          return finish(fail('PROTOCOL_INVALID'));
        }
        finish(null, value);
      }
      child.stdout.on('data', onData);
      child.once('error', onError);
      child.once('exit', onError);
    });

    const transport = await openBrokerTransport({...options, executable: helper, address: descriptor.address, token: descriptor.token});
    const close = transport.close.bind(transport);
    transport.close = async () => {
      try { await close(); } finally { await stop(); }
    };
    return Object.assign(transport, {mode: 'child'});
  } catch (error) {
    await stop();
    throw error;
  }
}

/**
 * Resolve a transport without the caller naming a binary: a granted shared
 * broker first, then a private parent-owned child.
 */
export async function openAutoTransport(options = {}) {
  const helper = resolveHelper(options);
  const address = options.address ?? defaultAddress();
  if (typeof address !== 'string' || !address || address.length > 200 || address.includes('\0')) throw fail('INVALID_REQUEST');
  const token = options.token ?? sharedToken(address);
  if (token) {
    try {
      const transport = await openBrokerTransport({...options, executable: helper, address, token});
      try {
        const handshake = await transport.request('runtime.handshake', {protocolVersion: '1.0', clientName: 'local-ai-kit-auto'});
        if (!handshake || !/^1\.\d+$/.test(handshake.protocolVersion) || !Array.isArray(handshake.methods) || !handshake.methods.every(method => typeof method === 'string')) {
          throw fail('PROTOCOL_INCOMPATIBLE');
        }
        return Object.assign(transport, {mode: 'shared'});
      } catch (error) {
        await transport.close();
        throw error;
      }
    } catch (error) {
      if (error instanceof LocalAIError && ['RUNTIME_MISSING', 'TRANSPORT_CLOSED', 'PROTOCOL_INCOMPATIBLE'].includes(error.code)) return openPrivateTransport(helper, options);
      throw error;
    }
  }
  return openPrivateTransport(helper, options);
}
