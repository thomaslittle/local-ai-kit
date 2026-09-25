import { LocalAIError } from './errors.ts';
import type { CallOptions, Handshake, Transport } from './types.ts';
import type { MethodInputs, MethodOutputs, Snapshot, Event } from './generated.ts';
const families = {
  runtime: ['health', 'closeClient'], discovery: ['refresh', 'cancel'], snapshot: ['get'],
  connections: ['list', 'setupInstructions', 'authenticate', 'probe'], models: ['list', 'page'], usage: ['read'], capabilities: ['read'],
  selection: ['resolve', 'validate'], grants: ['request', 'list', 'revoke'],
  sessions: ['create', 'get', 'updateOptions', 'close'], turns: ['start', 'cancel'], approvals: ['respond'],
  artifacts: ['read'], diagnostics: ['report'], adapters: ['list'],
} as const;
type Method = <T = unknown>(params?: unknown, options?: CallOptions) => Promise<T>;
type Input<K extends string> = K extends keyof MethodInputs ? MethodInputs[K] : unknown;
type Output<K extends string> = K extends keyof MethodOutputs ? MethodOutputs[K] : unknown;
type TypedMethod<K extends string> = {} extends Input<K>
  ? <T = Output<K>>(params?: Input<K>, options?: CallOptions) => Promise<T>
  : <T = Output<K>>(params: Input<K>, options?: CallOptions) => Promise<T>;
type Families = { [F in keyof typeof families]: { [M in typeof families[F][number]]: TypedMethod<`${F}.${M}`> } };
interface SnapshotMethods {
  get<T = Snapshot>(params?: { fields?: undefined }, options?: CallOptions): Promise<T>;
  get<K extends keyof Snapshot>(params: { fields: K[] }, options?: CallOptions): Promise<Pick<Snapshot, K>>;
}
export type LocalAIClient = Omit<Families, 'snapshot'> & {
  snapshot: SnapshotMethods;
  handshake: Readonly<Handshake>;
  close(): Promise<void>;
  events: { subscribe<T = Event>(query?: { sessionId?: string; after?: string }, options?: CallOptions): AsyncIterable<T> };
};
export async function createLocalAI(options: { transport: Transport; clientName?: string; signal?: AbortSignal }): Promise<LocalAIClient> {
  const transport = options.transport;
  let handshake: Handshake;
  try {
    handshake = await transport.request<Handshake>('runtime.handshake', { protocolVersion: '1.0', clientName: options.clientName ?? 'local-ai-kit-sdk' }, { signal: options.signal });
    if (!handshake || !/^1\.\d+$/.test(handshake.protocolVersion) || !Array.isArray(handshake.methods) || !handshake.methods.every(m => typeof m === 'string')) throw new LocalAIError('PROTOCOL_INCOMPATIBLE');
  } catch (error) { await transport.close(); throw error; }
  const methods = new Set(handshake.methods); let closed = false; let closing: Promise<void> | undefined;
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
    async close() {
      if (closing) return closing;
      closed=true;
      closing=(async()=>{
        try { if(methods.has('runtime.closeClient')) await transport.request('runtime.closeClient',{}, {timeoutMs:1000}); }
        catch { /* A disconnected peer must not prevent local cleanup or leak native errors. */ }
        finally { await transport.close(); }
      })();
      return closing;
    },
    events: Object.freeze({ async *subscribe<T>(query = {}, call = {}) {
      requireAvailable('events.subscribe');
      if (!transport.subscribe) throw new LocalAIError('CAPABILITY_UNSUPPORTED');
      yield* transport.subscribe<T>(query, call);
    } }),
  }) as unknown as LocalAIClient;
}
