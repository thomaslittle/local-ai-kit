import { LocalAIError } from './errors.ts';
import type { CallOptions, Handshake, Transport } from './types.ts';
const families = {
  runtime: ['health'], discovery: ['refresh', 'cancel'], snapshot: ['get'],
  connections: ['list', 'setupInstructions', 'authenticate', 'probe'], models: ['list'], usage: ['read'], capabilities: ['read'],
  selection: ['resolve', 'validate'], grants: ['request', 'list', 'revoke'],
  sessions: ['create', 'get', 'updateOptions', 'close'], turns: ['start', 'cancel'], approvals: ['respond'],
  artifacts: ['read'], diagnostics: ['report'], adapters: ['list'],
} as const;
type Method = <T = unknown>(params?: unknown, options?: CallOptions) => Promise<T>;
type Families = { [F in keyof typeof families]: { [M in typeof families[F][number]]: Method } };
export type LocalAIClient = Families & { handshake: Readonly<Handshake>; close(): Promise<void>; events: { subscribe: NonNullable<Transport['subscribe']> } };
export async function createLocalAI(options: { transport: Transport; clientName?: string; signal?: AbortSignal }): Promise<LocalAIClient> {
  const transport = options.transport;
  let handshake: Handshake;
  try {
    handshake = await transport.request<Handshake>('runtime.handshake', { protocolVersion: '1.0', clientName: options.clientName ?? 'local-ai-kit-sdk' }, { signal: options.signal });
    if (!handshake || !/^1\.\d+$/.test(handshake.protocolVersion) || !Array.isArray(handshake.methods) || !handshake.methods.every(m => typeof m === 'string')) throw new LocalAIError('PROTOCOL_INCOMPATIBLE');
  } catch (error) { await transport.close(); throw error; }
  const methods = new Set(handshake.methods); let closed = false;
  const requireAvailable = (method: string) => {
    if (closed) throw new LocalAIError('CLOSED');
    if (!methods.has(method)) throw new LocalAIError('CAPABILITY_UNSUPPORTED');
  };
  const client: Record<string, unknown> = {};
  for (const [family, names] of Object.entries(families)) {
    const group: Record<string, Method> = {};
    for (const name of names) group[name] = async <T>(params: unknown = {}, call: CallOptions = {}) => {
      const method = `${family}.${name}`; requireAvailable(method);
      return transport.request<T>(method, params, call);
    };
    client[family] = Object.freeze(group);
  }
  return Object.assign(client, {
    handshake: Object.freeze({ ...handshake, methods: Object.freeze([...handshake.methods]) }),
    async close() { if (!closed) { closed = true; await transport.close(); } },
    events: Object.freeze({ async *subscribe<T>(query = {}, call = {}) {
      requireAvailable('events.subscribe');
      if (!transport.subscribe) throw new LocalAIError('CAPABILITY_UNSUPPORTED');
      yield* transport.subscribe<T>(query, call);
    } }),
  }) as unknown as LocalAIClient;
}
