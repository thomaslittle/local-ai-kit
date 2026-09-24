import { LocalAIError, limit, requireMethod } from './errors.ts';
import { parseSSE } from './sse.ts';
import type { CallOptions, EventQuery, Transport } from './types.ts';
export interface HttpOptions {
  baseUrl: string; token: string; timeoutMs?: number; maxFrameBytes?: number;
  /** Dependency injection for hosts/tests; the default is the platform fetch. */
  fetch?: typeof globalThis.fetch;
}
function endpoint(value: string): string {
  let url: URL; try { url = new URL(value); } catch { throw new LocalAIError('LOCAL_ENDPOINT_REQUIRED'); }
  if (!['http:', 'https:'].includes(url.protocol) || !['localhost', '127.0.0.1', '[::1]'].includes(url.hostname)
    || url.username || url.password || url.search || url.hash || url.pathname !== '/') throw new LocalAIError('LOCAL_ENDPOINT_REQUIRED');
  return url.origin;
}
async function readJSON(response: Response, max: number): Promise<unknown> {
  if (!response.headers.get('content-type')?.toLowerCase().startsWith('application/json') || !response.body) {
    await response.body?.cancel().catch(() => {}); throw new LocalAIError('PROTOCOL_INVALID');
  }
  const reader = response.body.getReader(); const decoder = new TextDecoder('utf-8', { fatal: true });
  let size = 0, text = '';
  try {
    while (true) {
      const next = await reader.read(); if (next.done) break;
      size += next.value.byteLength; if (size > max) throw new LocalAIError('FRAME_TOO_LARGE');
      text += decoder.decode(next.value, { stream: true });
    }
    return JSON.parse(text + decoder.decode());
  } catch (e) { if (e instanceof LocalAIError) throw e; throw new LocalAIError('PROTOCOL_INVALID'); }
  finally { await reader.cancel().catch(() => {}); reader.releaseLock(); }
}
export function createHttpTransport(options: HttpOptions): Transport {
  const origin = endpoint(options.baseUrl);
  if (!/^[\x21-\x7e]{1,4096}$/.test(options.token)) throw new LocalAIError('INVALID_AUTH');
  const fetcher = options.fetch ?? globalThis.fetch;
  const max = limit(options.maxFrameBytes ?? 1024 * 1024);
  const defaultTimeout = limit(options.timeoutMs ?? 30000, 1, 3600000);
  const lifecycle = new AbortController(); let sequence = 0;
  function context(call: CallOptions, streaming: boolean) {
    if (lifecycle.signal.aborted) throw new LocalAIError('CLOSED');
    if (call.signal?.aborted) throw new LocalAIError('CANCELLED');
    const timeout = new AbortController();
    const duration = call.timeoutMs === undefined ? (streaming ? undefined : defaultTimeout) : limit(call.timeoutMs, 1, 3600000);
    const timer = duration === undefined ? undefined : setTimeout(() => timeout.abort(), duration);
    const signal = AbortSignal.any([lifecycle.signal, timeout.signal, ...(call.signal ? [call.signal] : [])]);
    return { signal, dispose: () => { if (timer !== undefined) clearTimeout(timer); }, translate(error: unknown): Error {
      if (lifecycle.signal.aborted) return new LocalAIError('CLOSED');
      if (call.signal?.aborted) return new LocalAIError('CANCELLED');
      if (timeout.signal.aborted) return new LocalAIError('TIMEOUT');
      return error instanceof LocalAIError ? error : new LocalAIError('TRANSPORT_FAILED');
    } };
  }
  async function checkStatus(response: Response): Promise<void> {
    if (!response.ok) { await response.body?.cancel().catch(() => {}); throw new LocalAIError(`HTTP_${response.status}`); }
  }
  return {
    async request<T>(method: string, params: unknown = {}, call: CallOptions = {}): Promise<T> {
      requireMethod(method); const ctx = context(call, false);
      try {
        if (sequence >= Number.MAX_SAFE_INTEGER) throw new LocalAIError('REQUEST_ID_EXHAUSTED');
        const id = ++sequence; let body: string;
        try { body = JSON.stringify({ jsonrpc: '2.0', id, method, params }); } catch { throw new LocalAIError('INVALID_REQUEST'); }
        if (new TextEncoder().encode(body).byteLength > max) throw new LocalAIError('FRAME_TOO_LARGE');
        const response = await fetcher(`${origin}/v1/rpc`, { method: 'POST', body, signal: ctx.signal,
          headers: { 'Content-Type': 'application/json', Accept: 'application/json', Authorization: `Bearer ${options.token}` }, credentials: 'omit', redirect: 'error' });
        await checkStatus(response);
        const envelope = await readJSON(response, max) as Record<string, unknown>;
        if (!envelope || typeof envelope !== 'object' || envelope.jsonrpc !== '2.0' || envelope.id !== id
          || Object.hasOwn(envelope, 'result') === Object.hasOwn(envelope, 'error')) throw new LocalAIError('PROTOCOL_INVALID');
        if (Object.hasOwn(envelope, 'error')) {
          const error = envelope.error as { code?: unknown; data?: { code?: unknown; retryable?: unknown } };
          if (!error || typeof error !== 'object' || typeof error.code !== 'number') throw new LocalAIError('PROTOCOL_INVALID');
          const code = typeof error.data?.code === 'string' ? error.data.code : 'REMOTE_ERROR';
          throw new LocalAIError(code, error.data?.retryable === true);
        }
        return envelope.result as T;
      } catch (error) { throw ctx.translate(error); } finally { ctx.dispose(); }
    },
    async *subscribe<T>(query: EventQuery = {}, call: CallOptions = {}): AsyncGenerator<T> {
      const ctx = context(call, true);
      try {
        const url = new URL(`${origin}/v1/events`);
        for (const key of ['sessionId', 'after'] as const) if (query[key] !== undefined) {
          if (typeof query[key] !== 'string' || query[key].length > 512 || /[\x00-\x1f]/.test(query[key])) throw new LocalAIError('INVALID_REQUEST');
          url.searchParams.set(key, query[key]);
        }
        const response = await fetcher(url.href, { method: 'GET', signal: ctx.signal,
          headers: { Accept: 'text/event-stream', Authorization: `Bearer ${options.token}` }, credentials: 'omit', redirect: 'error' });
        await checkStatus(response);
        if (!response.headers.get('content-type')?.toLowerCase().startsWith('text/event-stream') || !response.body) {
          await response.body?.cancel().catch(() => {}); throw new LocalAIError('PROTOCOL_INVALID');
        }
        for await (const frame of parseSSE(response.body, { maxEventBytes: max })) {
          if (frame.event === 'gap') throw new LocalAIError('EVENT_GAP');
          let value: unknown; try { value = JSON.parse(frame.data); } catch { throw new LocalAIError('PROTOCOL_INVALID'); }
          yield value as T;
        }
      } catch (error) { throw ctx.translate(error); } finally { ctx.dispose(); }
    },
    async close() { lifecycle.abort(); },
  };
}
