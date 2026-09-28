import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import {
  createCatalog,
  reconcileSelection,
  resolveCandidates,
  type Candidate,
  type Catalog,
  type Selection,
} from '@local-ai-kit/selector-core';
import {
  readArtifactBytes,
  readCatalogSnapshot,
  type LocalAIClient,
  type Snapshot,
} from '@local-ai-kit/sdk';
import { filterLiveCatalog } from '../../lib/catalog_visibility.mjs';
import { debugSessionError, userFacingError, userFacingSessionError } from '../../lib/errors.mjs';

export type Page =
  | 'Overview'
  | 'Connections'
  | 'Models'
  | 'Usage'
  | 'Selector lab'
  | 'Playground'
  | 'Diagnostics';

export type Mode = 'synthetic' | 'live';

export interface ProbeResult {
  ok: boolean;
  message: string;
}

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant';
  text: string;
  status: string;
}

export interface DebugError {
  code: string;
  message: string;
}

export interface ApprovalRequest {
  approvalId: string;
  action: string;
  detail: string;
}

export interface DemoState {
  page: Page;
  variant: string;
  mode: Mode;
  snapshot: Snapshot | null;
  selection: Selection | null;
  provider: string;
  query: string;
  required: string[];
  onlyEligible: boolean;
  allowPaid: boolean;
  favorites: string[];
  sensitiveConsent: boolean;
  mobileStep: number;
  mobileProvider: string;
  mobileConnection: string | null;
  playgroundProviderId: string | null;
  hasClient: boolean;
  events: any[];
  sessionId: string | null;
  nativeResumption: boolean;
  resumeToken: string | null;
  turnId: string | null;
  sessionStatus: string;
  output: string;
  chat: ChatMessage[];
  debugError: DebugError | null;
  approval: ApprovalRequest | null;
  grantConsent: boolean;
  simulateApproval: boolean;
  workspaceId: string;
  probing: string | null;
  probeResults: Record<string, ProbeResult>;
  metadataBatch: { completed: number; total: number } | null;
  prompt: string;
  notice: string;
  connectionError: string | null;
  diagnostics: unknown;
  handshake: unknown;
  detectedExpanded: boolean;
}

export interface DemoFactory {
  connectFixture: () => Promise<LocalAIClient>;
  connectLive: (endpoint: string, token: string) => Promise<LocalAIClient>;
  initialClient?: LocalAIClient | null;
  initialMode?: Mode;
  /** Test/a11y fixture flags from the URL (fixture bootstrap only). */
  fixtureFlags?: {
    staleAuth?: boolean;
    extraProfiles?: boolean;
    forceMode?: Mode;
    probeResults?: boolean;
    bulkProbeHook?: boolean;
  };
}

const initialState: DemoState = {
  page: 'Selector lab',
  variant: 'rail',
  mode: 'synthetic',
  snapshot: null,
  selection: null,
  provider: 'codex',
  query: '',
  required: [],
  onlyEligible: false,
  allowPaid: false,
  favorites: [],
  sensitiveConsent: false,
  mobileStep: 0,
  mobileProvider: 'codex',
  mobileConnection: null,
  playgroundProviderId: null,
  hasClient: false,
  events: [],
  sessionId: null,
  nativeResumption: false,
  resumeToken: null,
  turnId: null,
  sessionStatus: 'Ready',
  output: '',
  chat: [],
  debugError: null,
  approval: null,
  grantConsent: false,
  simulateApproval: false,
  workspaceId: 'default',
  probing: null,
  probeResults: {},
  metadataBatch: null,
  prompt: '',
  notice: '',
  connectionError: null,
  diagnostics: null,
  handshake: null,
  detectedExpanded: true,
};

export interface Demo extends DemoState {
  catalog: Catalog | null;
  all: Candidate[];
  candidates: Candidate[];
  providers: any[];
  connections: any[];
  patch: (update: Partial<DemoState>) => void;
  select: (candidate: Candidate) => void;
  startTurn: () => void;
  cancelTurn: () => void;
  suspendSession: () => void;
  resumeSession: () => void;
  answerApproval: (allow: boolean) => void;
  scan: () => void;
  probeConnection: (id: string) => Promise<boolean>;
  probeAllConnections: () => void;
  revokeConnection: () => void;
  refreshDiagnostics: () => void;
  downloadArtifact: (event: any) => void;
  connectLive: (endpoint: string, token: string) => void;
}

function applyFixtureFlags(snapshot: Snapshot, flags: NonNullable<DemoFactory['fixtureFlags']>): Snapshot {
  let next: any = snapshot;
  if (flags.staleAuth) {
    next = {
      ...next,
      connections: (next.connections ?? []).map((connection: any) =>
        connection?.auth?.value === 'authenticated'
          ? { ...connection, auth: { ...connection.auth, freshness: 'stale' } }
          : connection,
      ),
    };
  }
  if (flags.extraProfiles) {
    const now = new Date().toISOString();
    const source = { kind: 'native', adapterId: 'fixture', adapterVersion: '1', reference: 'fixture' };
    const localBilling = {
      state: 'known',
      value: 'local',
      source,
      observedAt: now,
      freshness: 'fresh',
      verification: 'observed',
    };
    next = {
      ...next,
      providers: [...(next.providers ?? []), { id: 'lmstudio', name: 'LM Studio', kind: 'local_runtime' }],
      connections: [
        ...(next.connections ?? []),
        {
          id: 'local-lm',
          providerId: 'lmstudio',
          displayName: 'LM Studio · Default profile',
          installationId: 'install-lm',
          profileId: 'profile-lm',
          accountId: null,
          auth: { state: 'unknown', reason: 'LOCAL_RUNTIME_HAS_NO_ACCOUNT_AUTH' },
          plan: { state: 'unknown', reason: 'NOT_REPORTED' },
          status: 'needs_auth',
          diagnostics: [],
        },
        {
          id: 'signed-out',
          providerId: 'codex',
          displayName: 'Codex · Additional profile',
          installationId: 'install-codex',
          profileId: 'profile-codex',
          accountId: null,
          auth: { state: 'known', value: 'unauthenticated', source, observedAt: now, freshness: 'fresh', verification: 'observed' },
          plan: { state: 'unknown', reason: 'NOT_REPORTED' },
          status: 'needs_auth',
          diagnostics: [],
        },
      ],
      routes: [
        ...(next.routes ?? []),
        { id: 'route-lm', connectionId: 'local-lm', providerId: 'lmstudio', displayName: 'LM Studio', billing: localBilling, capabilities: [], catalogRevision: 'fixture-lm' },
      ],
      models: [
        ...(next.models ?? []),
        { id: 'model-lm', routeId: 'route-lm', nativeId: 'ornith-1.5', name: 'Ornith 1.5', revision: 'fixture-lm', isDefault: false, capabilities: [], options: [], contextWindow: { state: 'unknown', reason: 'NOT_REPORTED' } },
      ],
    };
  }
  return next as Snapshot;
}

/** UI host. Consumes public SDK clients only; no account or provider logic is embedded. */
export function useDemo(factory: DemoFactory): Demo {
  const [state, setState] = useState<DemoState>(initialState);
  // The ref is the source of truth for handlers. It is written synchronously by
  // `patch` (never from render), so async callbacks that run before React commits
  // — back-to-back stream events, a rebuild followed by a read — see current state
  // instead of dropping text deltas or reading a stale snapshot.
  const stateRef = useRef(initialState);
  const clientRef = useRef<LocalAIClient | null>(null);
  const catalogRef = useRef<Catalog | null>(null);
  const epochRef = useRef(0);
  const controllerRef = useRef<AbortController | null>(null);
  const sessionContextRef = useRef<string | null>(null);
  const closedRef = useRef(false);
  const factoryRef = useRef(factory);
  factoryRef.current = factory;

  const patch = useCallback((update: Partial<DemoState>) => {
    stateRef.current = { ...stateRef.current, ...update };
    setState(stateRef.current);
  }, []);

  const rebuild = useCallback(
    (snapshot: Snapshot, mode: Mode, allowModeMix = false) => {
      const shape = snapshot as any;
      if (
        !shape ||
        !Array.isArray(shape.connections) ||
        !Array.isArray(shape.models) ||
        !Array.isArray(shape.routes) ||
        !Array.isArray(shape.providers)
      ) {
        throw new Error('INVALID_SNAPSHOT');
      }
      if (!allowModeMix) {
        if (mode === 'live' && (snapshot as any).mode !== 'live') throw new Error('SYNTHETIC_RUNTIME_REJECTED');
        if (mode === 'synthetic' && (snapshot as any).mode !== 'synthetic') throw new Error('LIVE_RUNTIME_REJECTED');
      }
      const visible = (mode === 'live' ? filterLiveCatalog(snapshot as any) : snapshot) as Snapshot;
      const catalog = createCatalog(visible as any);
      catalogRef.current = catalog;
      const current = stateRef.current;
      let selection = current.selection;
      let notice = current.notice;
      if (selection) {
        const reconciled = reconcileSelection(catalog, selection);
        selection = reconciled.selection;
        if (reconciled.needsReview) notice = 'The catalog changed. Review the updated selection before use.';
      }
      const resolved = resolveCandidates(catalog, { allowPaidFallback: current.allowPaid });
      if (!selection) {
        const first = resolved.find((c) => c.eligible) ?? resolved[0];
        if (first) {
          selection = { routeId: first.route.id, modelId: first.model.id, revision: first.model.revision, options: {} };
        }
      }
      const selected = resolved.find((c) => c.route.id === selection?.routeId && c.model.id === selection?.modelId);
      // The rail defaults to the first provider; when that provider is absent from
      // this catalog, fall back to All instead of showing an unexplained empty list.
      const providerIds = ((visible as any).providers ?? []).map((provider: any) => provider.id);
      const provider =
        current.provider === 'all' || current.provider === 'favorites' || providerIds.includes(current.provider) ? current.provider : 'all';
      patch({
        snapshot: visible,
        selection,
        notice,
        provider,
        playgroundProviderId: selected ? selected.route.providerId : current.playgroundProviderId,
      });
    },
    [patch],
  );

  const select = useCallback(
    (candidate: Candidate) => {
      patch({
        selection: { routeId: candidate.route.id, modelId: candidate.model.id, revision: candidate.model.revision, options: {} },
        playgroundProviderId: candidate.route.providerId,
        sensitiveConsent: false,
        debugError: null,
        notice: '',
      });
    },
    [patch],
  );

  const safeError = useCallback((error: unknown): string => userFacingError(error), []);

  const operate = useCallback(
    async (action: () => Promise<void>, includeDebug = false) => {
      try {
        await action();
      } catch (error) {
        const current = stateRef.current;
        patch({
          notice: safeError(error),
          debugError: includeDebug ? debugSessionError(error as any) : current.debugError,
        });
      }
    },
    [patch, safeError],
  );

  const stopClient = useCallback(async () => {
    controllerRef.current?.abort();
    controllerRef.current = null;
    const client = clientRef.current;
    clientRef.current = null;
    if (client) await client.close();
  }, []);

  const observe = useCallback(
    (client: LocalAIClient, currentEpoch: number) => {
      const controller = new AbortController();
      controllerRef.current = controller;
      void (async () => {
        try {
          const stream = client.events.subscribe(
            { after: '0' },
            { signal: controller.signal },
          ) as AsyncIterable<any>;
          for await (const event of stream) {
            if (closedRef.current || epochRef.current !== currentEpoch) break;
            if (!event?.payload || typeof event.payload.type !== 'string') continue;
            const current = stateRef.current;
            const events = [...current.events, event].slice(-100);
            let output = current.output;
            let chat = current.chat;
            let approval = current.approval;
            let sessionStatus = current.sessionStatus;
            let notice = current.notice;
            let debugError = current.debugError;
            if (event.sessionId === current.sessionId) {
              const payload = event.payload;
              if (payload.type === 'text_delta' && typeof payload.text === 'string') {
                output = (output + payload.text).slice(-65536);
                chat = appendAssistantText(chat, payload.text);
              }
              if (payload.type === 'approval_request') {
                approval = payload as ApprovalRequest;
                sessionStatus = 'Awaiting approval';
              }
              if (payload.type === 'approval_resolved') approval = null;
              if (payload.type === 'final') {
                sessionStatus =
                  payload.status === 'cancelled'
                    ? 'Cancelled'
                    : payload.status === 'denied'
                      ? 'Denied'
                      : payload.status === 'completed'
                        ? 'Completed'
                        : 'Error';
                approval = null;
                chat = markAssistantStatus(chat, sessionStatus);
              }
              if (payload.type === 'error') {
                sessionStatus = 'Error';
                notice = userFacingSessionError(payload);
                debugError = debugSessionError(payload);
                chat = markAssistantStatus(chat, 'Error');
              }
            }
            patch({ events, output, chat, approval, sessionStatus, notice, debugError });
          }
        } catch (error) {
          if (!closedRef.current && epochRef.current === currentEpoch && !controller.signal.aborted) {
            patch({ notice: safeError(error) });
          }
        }
      })();
    },
    [patch, safeError],
  );

  const scan = useCallback(async () => {
    const client = clientRef.current;
    if (!client) return;
    await operate(async () => {
      await client.discovery.refresh({ level: 'passive' });
      const flags = factoryRef.current.fixtureFlags;
      rebuild(await readCatalogSnapshot(client), stateRef.current.mode, Boolean(flags?.forceMode));
      patch({ page: 'Connections' });
    });
  }, [operate, patch, rebuild]);

  const probeConnection = useCallback(
    async (id: string): Promise<boolean> => {
      const client = clientRef.current;
      if (!client || stateRef.current.probing) return false;
      const currentEpoch = epochRef.current;
      patch({ notice: '', probing: id });
      try {
        await client.connections.probe({ connectionId: id });
        if (closedRef.current || epochRef.current !== currentEpoch) return false;
        const flags = factoryRef.current.fixtureFlags;
        rebuild(await readCatalogSnapshot(client), stateRef.current.mode, Boolean(flags?.forceMode));
        const current = stateRef.current;
        const snapshot = current.snapshot as any;
        const connection = snapshot?.connections?.find((item: any) => item.id === id);
        const routeIds = new Set((snapshot?.routes ?? []).filter((route: any) => route.connectionId === id).map((route: any) => route.id));
        const modelCount = (snapshot?.models ?? []).filter((model: any) => routeIds.has(model.routeId)).length;
        const status = connection?.auth?.state === 'known' ? connection.auth.value : 'authentication not reported';
        patch({
          probeResults: {
            ...current.probeResults,
            [id]: { ok: true, message: `Metadata check complete · ${status} · ${modelCount} model${modelCount === 1 ? '' : 's'} reported` },
          },
        });
        return true;
      } catch (error) {
        if (!closedRef.current && epochRef.current === currentEpoch) {
          const current = stateRef.current;
          patch({
            probeResults: {
              ...current.probeResults,
              [id]: { ok: false, message: `Metadata check unavailable · ${safeError(error)}` },
            },
          });
        }
        return false;
      } finally {
        if (!closedRef.current && epochRef.current === currentEpoch) patch({ probing: null });
      }
    },
    [patch, rebuild, safeError],
  );

  const probeAllConnections = useCallback(async () => {
    const client = clientRef.current;
    if (!client || stateRef.current.probing || stateRef.current.metadataBatch) return;
    if (factoryRef.current.fixtureFlags?.bulkProbeHook) {
      document.body.dataset.bulkProbe = 'started';
    }
    const currentEpoch = epochRef.current;
    // Read identifiers from a fresh snapshot rather than rendered state: right
    // after pairing, React may not have committed the attach patches yet.
    const fresh = await readCatalogSnapshot(client).catch(() => null);
    if (closedRef.current || epochRef.current !== currentEpoch || !clientRef.current) return;
    const ids = ((fresh as any)?.connections ?? []).map((connection: any) => connection.id);
    if (!ids.length) return;
    patch({ notice: '', metadataBatch: { completed: 0, total: ids.length } });
    for (const id of ids) {
      if (closedRef.current || epochRef.current !== currentEpoch || !clientRef.current) break;
      await probeConnection(id);
      if (closedRef.current || epochRef.current !== currentEpoch) break;
      const batch = stateRef.current.metadataBatch as { completed: number; total: number } | null;
      if (batch) patch({ metadataBatch: { completed: batch.completed + 1, total: batch.total } });
    }
    if (!closedRef.current && epochRef.current === currentEpoch) patch({ metadataBatch: null });
  }, [patch, probeConnection]);

  const attach = useCallback(
    async (create: () => Promise<LocalAIClient>, mode: Mode) => {
      const current = ++epochRef.current;
      await stopClient();
      sessionContextRef.current = null;
      catalogRef.current = null;
      const flags = factoryRef.current.fixtureFlags;
      patch({
        ...initialState,
        page: 'Selector lab',
        variant: 'rail',
        mode: flags?.forceMode ?? mode,
        favorites: [],
      });
      let client: LocalAIClient | undefined;
      try {
        client = await create();
        if (closedRef.current || epochRef.current !== current) {
          await client.close();
          return;
        }
        let snapshot = await readCatalogSnapshot(client);
        if (flags && (flags.staleAuth || flags.extraProfiles)) snapshot = applyFixtureFlags(snapshot, flags);
        if (flags?.probeResults) {
          const probeResults: Record<string, ProbeResult> = {};
          for (const connection of (snapshot as any).connections ?? []) {
            probeResults[connection.id] = { ok: false, message: 'Metadata check unavailable · provider runtime unavailable' };
          }
          patch({ probeResults });
        }
        if (closedRef.current || epochRef.current !== current) {
          await client.close();
          return;
        }
        rebuild(snapshot, flags?.forceMode ?? mode, Boolean(flags?.forceMode));
        clientRef.current = client;
        patch({ hasClient: true, handshake: client.handshake });
        if (client.handshake.methods.includes('events.subscribe')) observe(client, current);
        // Scripted fixture flags arrange their own snapshot; automated checks
        // drive metadata actions explicitly instead of auto-probing here.
        if ((flags?.forceMode ?? mode) === 'live' && !flags) {
          await scan();
          if (epochRef.current === current && clientRef.current) await probeAllConnections();
        }
      } catch {
        await client?.close().catch(() => {});
        if (epochRef.current === current) {
          patch({ connectionError: 'Unable to connect. No account or model data has been substituted.' });
        }
      }
    },
    [observe, patch, probeAllConnections, rebuild, scan, stopClient],
  );

  const attachLive = useCallback(
    (endpoint: string, token: string) => {
      const active = factoryRef.current;
      return attach(() => active.connectLive(endpoint, token), 'live');
    },
    [attach],
  );

  const startTurn = useCallback(async () => {
    const current = stateRef.current;
    if (['Starting', 'Running', 'Awaiting approval'].includes(current.sessionStatus)) return;
    // Surface why nothing happened instead of silently ignoring Send / Enter.
    if (!current.selection) {
      patch({ notice: 'Choose a model and account before starting a turn.' });
      return;
    }
    if (!current.grantConsent) {
      patch({ notice: 'Allow this session with the checkbox below the prompt before sending.' });
      return;
    }
    if (!clientRef.current) return;
    const client = clientRef.current;
    const epoch = epochRef.current;
    const selection = structuredClone(current.selection);
    const previousStatus = current.sessionStatus;
    const context = JSON.stringify({
      selection,
      workspaceId: current.workspaceId,
      allowPaid: current.allowPaid,
      mode: current.mode,
    });
    const userText = current.prompt;
    const assistantId = `assistant-${Date.now()}`;
    const userMessage: ChatMessage | null = userText.trim()
      ? { id: `user-${Date.now()}`, role: 'user', text: userText, status: 'sent' }
      : null;
    const pendingAssistant: ChatMessage | null = userMessage
      ? { id: assistantId, role: 'assistant', text: '', status: 'Running' }
      : null;
    patch({
      sessionStatus: 'Starting',
      output: '',
      debugError: null,
      approval: null,
      notice: '',
      chat: [...current.chat, ...(userMessage ? [userMessage] : []), ...(pendingAssistant ? [pendingAssistant] : [])],
    });
    await operate(async () => {
      const catalog = catalogRef.current;
      const resolved = catalog ? resolveCandidates(catalog, { allowPaidFallback: stateRef.current.allowPaid }) : [];
      const model = resolved.find((c) => c.route.id === selection.routeId && c.model.id === selection.modelId);
      if (!model) throw new Error('SELECTION_STALE');
      const latest = stateRef.current;
      const reuse = latest.sessionId && sessionContextRef.current === context && ['Ready', 'Completed'].includes(previousStatus);
      let session: any;
      if (reuse) {
        session = await client.sessions.get({ sessionId: latest.sessionId as string });
      } else {
        if (latest.sessionId) await client.sessions.close({ sessionId: latest.sessionId as string });
        if (latest.mode === 'synthetic') {
          await client.grants.request({ connectionIds: [model.connection.id], scopes: ['execution'], allowPaid: latest.allowPaid });
        }
        session = await client.sessions.create({
          selection,
          allowPaid: latest.allowPaid,
          ...(latest.mode === 'live' ? { workspaceId: latest.workspaceId, permissionMode: 'read_only' } : {}),
        } as any);
        sessionContextRef.current = context;
        // A fresh session starts a fresh visible thread.
        if (userMessage && pendingAssistant) patch({ chat: [userMessage, pendingAssistant] });
        else if (!userMessage) patch({ chat: [] });
      }
      if (epochRef.current !== epoch) return;
      patch({ sessionId: session.id, nativeResumption: Boolean(session.nativeResumption), resumeToken: null, turnId: null, sessionStatus: 'Running' });
      const turn = await client.turns.start({
        sessionId: session.id,
        prompt: stateRef.current.prompt,
        ...(stateRef.current.mode === 'synthetic' ? { simulateApproval: stateRef.current.simulateApproval } : { requestId: crypto.randomUUID() }),
      } as any);
      if (epochRef.current !== epoch) return;
      patch({ turnId: turn.turnId });
    }, true);
    const after = stateRef.current;
    if (after.notice && epochRef.current === epoch) patch({ sessionStatus: 'Error' });
  }, [operate, patch]);

  const cancelTurn = useCallback(async () => {
    const current = stateRef.current;
    if (current.turnId && clientRef.current) {
      await operate(async () => {
        await clientRef.current?.turns.cancel({ turnId: stateRef.current.turnId as string });
      });
    }
  }, [operate]);

  const suspendSession = useCallback(async () => {
    const current = stateRef.current;
    if (!clientRef.current || !current.sessionId || !current.nativeResumption) return;
    if (!clientRef.current.handshake.methods.includes('sessions.suspend')) return;
    await operate(async () => {
      const result = (await clientRef.current?.sessions.suspend({ sessionId: stateRef.current.sessionId as string })) as any;
      patch({ resumeToken: result.resumeToken, sessionId: null, turnId: null, sessionStatus: 'Suspended', output: '', approval: null });
    });
  }, [operate, patch]);

  const resumeSession = useCallback(async () => {
    const current = stateRef.current;
    if (!clientRef.current || !current.resumeToken) return;
    if (!clientRef.current.handshake.methods.includes('sessions.resume')) return;
    await operate(async () => {
      const session = (await clientRef.current?.sessions.resume({ resumeToken: stateRef.current.resumeToken as string })) as any;
      patch({ sessionId: session.id, nativeResumption: Boolean(session.nativeResumption), resumeToken: null, turnId: null, sessionStatus: 'Ready', output: '', approval: null });
    });
  }, [operate, patch]);

  const answerApproval = useCallback(
    async (allow: boolean) => {
      const current = stateRef.current;
      if (current.approval && clientRef.current) {
        await operate(async () => {
          await clientRef.current?.approvals.respond({ approvalId: stateRef.current.approval?.approvalId as string, allow });
        });
      }
    },
    [operate],
  );

  const revokeConnection = useCallback(async () => {
    if (!clientRef.current) return;
    await operate(async () => {
      await clientRef.current?.grants.revoke({});
      await stopClient();
      catalogRef.current = null;
      patch({
        ...initialState,
        page: stateRef.current.page,
        mode: stateRef.current.mode,
        favorites: [],
        notice: 'Connection revoked. Restart the native host and reload to pair again.',
      });
    });
  }, [operate, patch, stopClient]);

  const refreshDiagnostics = useCallback(async () => {
    if (!clientRef.current) return;
    await operate(async () => {
      const client = clientRef.current as LocalAIClient;
      patch({
        diagnostics: {
          report: await client.diagnostics.report(),
          adapters: await client.adapters.list(),
          grants: await client.grants.list(),
        },
      });
    });
  }, [operate, patch]);

  const downloadArtifact = useCallback(
    async (event: any) => {
      const client = clientRef.current;
      if (!client || event.sessionId !== stateRef.current.sessionId) return;
      if (!client.handshake.methods.includes('artifacts.read')) return;
      const epoch = epochRef.current;
      await operate(async () => {
        const result = await readArtifactBytes(client, event.sessionId, event.payload.artifactId);
        if (closedRef.current || epochRef.current !== epoch) return;
        // Always download as inert bytes; never navigate to or render a provider's executable content.
        const url = URL.createObjectURL(new Blob([result.bytes as BlobPart], { type: 'application/octet-stream' }));
        const anchor = document.createElement('a');
        anchor.href = url;
        anchor.download = result.displayName.replace(/[^a-zA-Z0-9_. -]/g, '_').slice(0, 128) || 'result.bin';
        anchor.rel = 'noreferrer';
        document.body.append(anchor);
        anchor.click();
        anchor.remove();
        setTimeout(() => URL.revokeObjectURL(url), 1000);
      });
    },
    [operate],
  );

  // Boot once. The fixture/live factories own discovery; rendering never does.
  useEffect(() => {
    const active = factoryRef.current;
    if (active.initialClient) {
      void attach(() => Promise.resolve(active.initialClient as LocalAIClient), active.initialMode ?? 'synthetic');
    } else {
      void attach(active.connectFixture, active.initialMode ?? 'synthetic');
    }
    return () => {
      closedRef.current = true;
      epochRef.current++;
      controllerRef.current?.abort();
      const client = clientRef.current;
      clientRef.current = null;
      if (client) void client.close();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const catalog = catalogRef.current;
  const all = useMemo(
    () => (catalog ? resolveCandidates(catalog, { allowPaidFallback: state.allowPaid }) : []),
    [catalog, state.allowPaid, state.snapshot],
  );
  const candidates = useMemo(
    () =>
      catalog
        ? resolveCandidates(catalog, {
            query: state.query,
            requiredCapabilities: state.required,
            eligibleOnly: state.onlyEligible,
            allowPaidFallback: state.allowPaid,
          })
        : [],
    [catalog, state.query, state.required, state.onlyEligible, state.allowPaid, state.snapshot],
  );
  const snapshot = state.snapshot as any;
  const providers = useMemo(() => snapshot?.providers ?? [], [snapshot]);
  const connections = useMemo(() => snapshot?.connections ?? [], [snapshot]);

  return {
    ...state,
    catalog,
    all,
    candidates,
    providers,
    connections,
    patch,
    select,
    startTurn,
    cancelTurn,
    suspendSession,
    resumeSession,
    answerApproval,
    scan,
    probeConnection,
    probeAllConnections,
    revokeConnection,
    refreshDiagnostics,
    downloadArtifact,
    connectLive: attachLive,
  };
}

function appendAssistantText(chat: ChatMessage[], text: string): ChatMessage[] {
  if (!chat.length || chat[chat.length - 1].role !== 'assistant') {
    return [...chat, { id: `assistant-${Date.now()}`, role: 'assistant', text, status: 'Running' }];
  }
  return chat.map((message, index) =>
    index === chat.length - 1 ? { ...message, text: (message.text + text).slice(-65536) } : message,
  );
}

function markAssistantStatus(chat: ChatMessage[], status: string): ChatMessage[] {
  if (!chat.length || chat[chat.length - 1].role !== 'assistant') return chat;
  return chat.map((message, index) => (index === chat.length - 1 ? { ...message, status } : message));
}
