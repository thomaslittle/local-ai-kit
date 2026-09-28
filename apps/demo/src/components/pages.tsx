import type * as React from 'react';
import { Button } from './ui/button';
import { Badge, badgeVariant } from './ui/badge';
import { CardSection } from './ui/card';
import { Empty, Progress } from './ui/feedback';
import { known, compactIdentity } from '../lib/format';
import { connectionAuthLabel } from '../../lib/profile_auth.mjs';
import { isLocalRuntimeProvider } from '../../lib/catalog_visibility.mjs';
import type { Demo } from '../state/demo';

function Section({ title, description, children }: { title: string; description?: string; children: React.ReactNode }): React.ReactElement {
  return (
    <CardSection className="p-6">
      <div className="section-heading mb-4">
        <h2 className="mb-1 text-lg font-semibold tracking-tight">{title}</h2>
        {description ? <p className="text-xs text-muted-foreground">{description}</p> : null}
      </div>
      {children}
    </CardSection>
  );
}

const authenticated = (c: any): boolean => known(c.auth) === 'authenticated' && c.auth.freshness === 'fresh';
const previouslyAuthenticated = (c: any): boolean => known(c.auth) === 'authenticated' && c.auth.freshness === 'stale';

export function OverviewPage({ demo }: { demo: Demo }): React.ReactElement {
  const snapshot = demo.snapshot as any;
  const connections = snapshot?.connections ?? [];
  const verified = connections.filter(authenticated);
  const previous = connections.filter(previouslyAuthenticated);
  const metrics: [unknown, string][] = [
    [connections.length, 'Detected profiles'],
    [verified.length, 'Verified profiles'],
    [snapshot?.models.length ?? 0, 'Route-scoped models'],
    [demo.all.filter((c) => c.eligible).length, 'Eligible selections'],
  ];
  return (
    <div className="stack grid gap-5">
      <div className="metrics grid grid-cols-2 gap-4 xl:grid-cols-4">
        {metrics.map(([value, label]) => (
          <div key={label} className="metric flex flex-col gap-1 rounded-xl border bg-card p-5">
            <span className="metric-value text-3xl tracking-tight text-teal-200">{String(value)}</span>
            <span className="text-sm">{label}</span>
            <small className="text-[10px] text-muted-foreground">{demo.mode === 'synthetic' ? 'Example data' : 'Runtime-reported'}</small>
          </div>
        ))}
      </div>
      <Section title="One catalog. Every application." description="Inspect first, select explicitly, execute only with permission.">
        <div className="steps grid grid-cols-2 gap-4 xl:grid-cols-4">
          {['Discover connections', 'Inspect the evidence', 'Select a route', 'Run with permission'].map((s, i) => (
            <div key={s}>
              <Badge variant="accent">{String(i + 1)}</Badge>
              <h3 className="mt-3 text-sm font-semibold">{s}</h3>
            </div>
          ))}
        </div>
      </Section>
      <Section title="Live showroom" description="This page uses the public SDK against the native local service.">
        <p className="mb-4 text-sm text-muted-foreground">
          Passive discovery reports installed clients. Account access, model catalogs, and usage remain tied to evidence from authorized probes. Empty results
          stay empty; no example records are inserted.
        </p>
        <Button
          variant="default"
          onClick={() => {
            demo.patch({ page: 'Selector lab' });
          }}
        >
          Explore model selectors
        </Button>
      </Section>
      {verified.length ? (
        <Section title="Verified profiles" description="Only fresh authenticated profiles appear here.">
          <div className="example-grid grid grid-cols-1 gap-5 md:grid-cols-2">
            {verified.slice(0, 4).map((c: any) => (
              <div key={c.id} className="mini-card rounded-lg border bg-background/60 p-4">
                <h3 className="text-sm font-semibold">{c.displayName}</h3>
                <div className="my-2">
                  <Badge variant="supported">Authenticated</Badge>
                </div>
                <p className="text-xs text-muted-foreground">{known(c.plan)?.label ?? 'Plan not reported'}</p>
              </div>
            ))}
          </div>
        </Section>
      ) : previous.length ? null : (
        <Section title="Verified profiles" description="Only fresh authenticated profiles appear here.">
          <Empty>
            <h3 className="text-foreground">No authenticated profiles yet</h3>
            <p className="text-xs">Detected clients remain unverified until an explicit account and model check succeeds.</p>
          </Empty>
        </Section>
      )}
      {previous.length ? (
        <Section
          title="Previously authenticated"
          description="These profiles passed an earlier check, but that authentication evidence is stale."
        >
          <div data-testid="previously-authenticated-overview" className="example-grid grid grid-cols-1 gap-5 md:grid-cols-2">
            {previous.slice(0, 4).map((c: any) => (
              <div key={c.id} className="mini-card rounded-lg border bg-background/60 p-4">
                <h3 className="text-sm font-semibold">{c.displayName}</h3>
                <div className="my-2">
                  <Badge variant="warning">Previously authenticated</Badge>
                </div>
                <p className="text-xs text-muted-foreground">{`${demo.providers.find((p: any) => p.id === c.providerId)?.name ?? c.providerId} · Check is stale`}</p>
                <p className="text-xs text-muted-foreground">{known(c.plan)?.label ?? 'Plan not reported'}</p>
              </div>
            ))}
          </div>
        </Section>
      ) : null}
    </div>
  );
}

export function ConnectionsPage({ demo }: { demo: Demo }): React.ReactElement {
  const localCatalogAvailable = (c: any): boolean =>
    isLocalRuntimeProvider(c) &&
    ((demo.snapshot as any)?.routes ?? []).some(
      (route: any) =>
        route.connectionId === c.id &&
        known(route.billing) === 'local' &&
        route.billing.freshness === 'fresh' &&
        ((demo.snapshot as any)?.models ?? []).some((model: any) => model.routeId === route.id),
    );
  const locallyUnauthenticated = (c: any): boolean => known(c.auth) === 'unauthenticated' && c.auth.freshness === 'fresh';

  const connectionCard = (c: any): React.ReactElement => {
    const provider = demo.providers.find((p: any) => p.id === c.providerId)?.name ?? c.providerId;
    const status = connectionAuthLabel(c);
    const local = isLocalRuntimeProvider(c);
    const catalogReady = localCatalogAvailable(c);
    return (
      <CardSection
        key={c.id}
        className="profile-card p-6"
        data-connection-id={c.id}
      >
        <div className="section-heading mb-4">
          <h2 className="mb-1 text-lg font-semibold tracking-tight">{c.displayName}</h2>
          <p className="text-xs text-muted-foreground">{provider}</p>
        </div>
        <div className="config-badges mb-4 flex flex-wrap gap-1.5">
          <Badge variant={badgeVariant(authenticated(c) ? 'supported' : local ? (catalogReady ? 'supported' : 'muted') : 'warning')}>{status}</Badge>
          <Badge variant={catalogReady ? 'supported' : 'muted'}>
            {local ? (catalogReady ? 'Local catalog ready' : 'Catalog not reported') : (c.auth.freshness ?? 'unavailable')}
          </Badge>
        </div>
        <dl className="facts my-5 grid grid-cols-[auto_minmax(0,1fr)] gap-x-3.5 gap-y-2.5 text-[11px]">
          <dt className="text-muted-foreground">Plan</dt>
          <dd className="text-right">{known(c.plan)?.label ?? 'Not reported'}</dd>
          <dt className="text-muted-foreground">Profile</dt>
          <dd className="mono identity-value truncate text-right" title={c.profileId} aria-label={`Profile identifier ${c.profileId}`}>
            {compactIdentity(c.profileId)}
          </dd>
          <dt className="text-muted-foreground">Account</dt>
          <dd className="mono identity-value truncate text-right" title={c.accountId ?? 'Unverified'} aria-label={`Account identifier ${c.accountId ?? 'Unverified'}`}>
            {compactIdentity(c.accountId ?? 'Unverified')}
          </dd>
          <dt className="text-muted-foreground">Installation</dt>
          <dd className="mono identity-value truncate text-right" title={c.installationId} aria-label={`Installation identifier ${c.installationId}`}>
            {compactIdentity(c.installationId)}
          </dd>
        </dl>
        {demo.probeResults?.[c.id] ? (
          <p role="status" aria-live="polite" className={`metadata-result mb-4 text-xs ${demo.probeResults[c.id].ok ? 'text-teal-200' : 'text-amber-200'}`}>
            {demo.probeResults[c.id].message}
          </p>
        ) : null}
        <details>
          <summary className="cursor-pointer text-[11px] text-muted-foreground">Evidence and freshness</summary>
          <pre className="code mt-3">{JSON.stringify({ auth: c.auth, plan: c.plan }, null, 2)}</pre>
        </details>
        <div className="mt-4 flex flex-wrap gap-2">
          {demo.mode === 'live' ? (
            <Button
              variant="outline"
              size="sm"
              disabled={Boolean(demo.probing) || Boolean(demo.metadataBatch)}
              onClick={() => void demo.probeConnection(c.id)}
            >
              {demo.probing === c.id ? 'Checking…' : local ? 'Refresh local catalog' : 'Check account and models'}
            </Button>
          ) : null}
          <Button
            variant="outline"
            size="sm"
            onClick={() => {
              demo.patch({ page: 'Selector lab', variant: 'rail', provider: c.providerId, query: c.displayName });
            }}
          >
            Inspect models
          </Button>
        </div>
      </CardSection>
    );
  };

  const cards = (list: any[]): React.ReactElement => (
    <div className="connections-grid grid grid-cols-1 gap-5 xl:grid-cols-2">{list.map(connectionCard)}</div>
  );

  const verified = demo.connections.filter(authenticated);
  const previous = demo.connections.filter(previouslyAuthenticated);
  const local = demo.connections.filter(isLocalRuntimeProvider);
  const unauthenticated = demo.connections.filter(locallyUnauthenticated);
  const detected = demo.connections.filter(
    (c: any) => !authenticated(c) && !previouslyAuthenticated(c) && !isLocalRuntimeProvider(c) && !locallyUnauthenticated(c),
  );
  const uncheckedDetected = detected.filter((c: any) => !demo.probeResults?.[c.id]);
  const batch = demo.metadataBatch;

  return (
    <div className="stack grid gap-5">
      {demo.mode === 'live' ? (
        <Section title="Profile metadata" description="Metadata-only: check every detected account and local catalog in sequence. No model prompts are sent.">
          {batch ? (
            <p role="status" aria-live="polite" className="mb-3 text-xs">{`${batch.completed} of ${batch.total} profiles checked`}</p>
          ) : null}
          <Button
            variant="default"
            data-testid="check-all-profiles"
            disabled={Boolean(demo.probing) || Boolean(batch)}
            onClick={() => demo.probeAllConnections()}
          >
            {batch
              ? `Checking profile ${Math.min(batch.completed + 1, batch.total)} of ${batch.total}…`
              : Object.keys(demo.probeResults ?? {}).length
                ? 'Recheck all profiles'
                : 'Check all detected profiles'}
          </Button>
        </Section>
      ) : null}
      {verified.length ? (
        <Section title="Authenticated profiles" description="Only fresh provider-confirmed authentication appears as a connected profile.">
          <div data-testid="verified-profiles" className="connections-grid grid grid-cols-1 gap-5 xl:grid-cols-2">
            {verified.map(connectionCard)}
          </div>
        </Section>
      ) : previous.length ? null : (
        <Section title="Authenticated profiles" description="Only fresh provider-confirmed authentication appears as a connected profile.">
          <Empty>
            <h3 className="text-foreground">No authenticated profiles yet</h3>
            <p className="text-xs">Run the metadata-only check below. No model prompts are sent.</p>
          </Empty>
        </Section>
      )}
      {previous.length ? (
        <Section
          title="Previously authenticated"
          description="These profiles passed an earlier check, but that evidence is stale. Check account and models again to confirm their current status."
        >
          <div data-testid="previously-authenticated-profiles" className="connections-grid grid grid-cols-1 gap-5 xl:grid-cols-2">
            {previous.map(connectionCard)}
          </div>
        </Section>
      ) : null}
      {local.length ? (
        <Section title="Local runtimes" description="Local model servers do not have account authentication. Their routes appear when the server reports a real model catalog.">
          <div data-testid="local-runtimes" className="connections-grid grid grid-cols-1 gap-5 xl:grid-cols-2">
            {local.map(connectionCard)}
          </div>
        </Section>
      ) : null}
      {unauthenticated.length ? (
        <Section
          title="Not authenticated"
          description="These explicit checks did not confirm a signed-in account. Their models are excluded; check again after signing in."
        >
          <div data-testid="unauthenticated-profiles" className="connections-grid grid grid-cols-1 gap-5 xl:grid-cols-2">
            {unauthenticated.map(connectionCard)}
          </div>
        </Section>
      ) : null}
      <details
        data-testid="detected-profiles"
        className="detected-profiles rounded-xl border bg-card p-6"
        open={demo.detectedExpanded}
        onToggle={(event) => demo.patch({ detectedExpanded: (event.currentTarget as HTMLDetailsElement).open })}
      >
        <summary className="cursor-pointer text-sm font-semibold">
          {uncheckedDetected.length
            ? `Other detected clients · ${uncheckedDetected.length} need a metadata check`
            : `Other detected clients · ${detected.length} checked, not confirmed`}
        </summary>
        <p className="my-3 text-xs text-muted-foreground">
          {uncheckedDetected.length
            ? 'Installation alone does not prove an account is signed in. Check profiles to request supported metadata; unsupported sources remain unverified.'
            : 'These profiles have a metadata result but are not confirmed as authenticated. Each result explains whether authentication was absent or the provider could not be checked.'}
        </p>
        {detected.length ? (
          cards(detected)
        ) : (
          <p className="text-xs text-muted-foreground">Every remaining profile has a current result above.</p>
        )}
      </details>
    </div>
  );
}

export function UsagePage({ demo }: { demo: Demo }): React.ReactElement {
  return (
    <div className="stack grid gap-5">
      <p className="intro text-sm text-muted-foreground">Windows belong to quota pools, not model rows. Shared pools appear once; unknown totals stay unknown.</p>
      <div className="usage-grid grid grid-cols-1 gap-5 xl:grid-cols-2">
        {((demo.snapshot as any)?.quotaPools ?? []).map((pool: any) => (
          <CardSection key={pool.id} data-pool={pool.id} className="p-6">
            <div className="section-heading mb-4 flex items-center justify-between gap-3">
              <h2 className="text-lg font-semibold tracking-tight">{pool.windows[0]?.label ?? pool.id}</h2>
              <Badge variant={pool.sharing === 'verified' ? 'accent' : 'muted'}>
                {pool.sharing === 'verified' ? 'Verified sharing' : 'Sharing unknown'}
              </Badge>
            </div>
            {pool.windows.map((window: any) => {
              const reading = known(window.reading) as any;
              return (
                <div key={window.id} className="usage-window mb-4">
                  {reading ? (
                    <div>
                      <div className="usage-number text-3xl tracking-tight">
                        {reading.used}
                        <span className="text-[0.8rem] text-muted-foreground">{` ${reading.unit}${reading.limit === null ? '' : ` / ${reading.limit}`}`}</span>
                      </div>
                      {reading.limit !== null && typeof reading.usedFraction === 'number' ? (
                        <Progress value={Math.max(0, Math.min(1, reading.usedFraction))} max={1} aria-label={`${window.label} used fraction`} className="my-4" />
                      ) : (
                        <p className="my-2 text-xs text-muted-foreground">No published total · no percentage implied</p>
                      )}
                      <div className="usage-foot mb-5 flex items-center justify-between gap-3 text-[11px] text-muted-foreground">
                        <Badge variant={window.reading.freshness === 'stale' ? 'warning' : 'muted'}>{window.reading.freshness ?? 'unknown'}</Badge>
                        <span>{window.resetsAt ? `Resets ${new Date(window.resetsAt).toLocaleTimeString()}` : 'Reset not reported'}</span>
                      </div>
                    </div>
                  ) : (
                    <Empty>
                      <h3 className="text-foreground">Usage unavailable</h3>
                      <p className="text-xs">{window.reading.reason}</p>
                    </Empty>
                  )}
                </div>
              );
            })}
            <p className="fine-print text-[10px] leading-relaxed text-muted-foreground">
              {`${pool.routeIds.length} linked route${pool.routeIds.length === 1 ? '' : 's'}. ${demo.mode === 'synthetic' ? 'Illustrative allowance only.' : ''}`}
            </p>
          </CardSection>
        ))}
      </div>
    </div>
  );
}

export function DiagnosticsPage({ demo }: { demo: Demo }): React.ReactElement {
  return (
    <div className="stack grid gap-5">
      <Section title="SDK / protocol inspector" description="Only normalized public responses are shown. No credentials are displayed.">
        <Button variant="outline" size="sm" onClick={() => demo.refreshDiagnostics()}>
          Refresh diagnostics
        </Button>
        <div className="mt-4">
          {demo.diagnostics ? (
            <pre className="code">{JSON.stringify(demo.diagnostics, null, 2)}</pre>
          ) : (
            <p className="text-xs text-muted-foreground">Diagnostics have not been requested.</p>
          )}
        </div>
        <details className="mt-5 border-t pt-3.5">
          <summary className="cursor-pointer text-[11px] text-muted-foreground">Negotiated public methods</summary>
          <pre className="code mt-3">{JSON.stringify(demo.handshake ?? {}, null, 2)}</pre>
        </details>
      </Section>
      <Section title="Integration recipe" description="Use the same selection with a compatible runtime. Token values are never included.">
        <pre className="code">{`import { createLocalAI, createHttpTransport } from "@local-ai-kit/sdk";

const ai = await createLocalAI({
  transport: createHttpTransport({
    baseUrl: "http://127.0.0.1:4499",
    token: appSessionToken, // obtained through explicit local pairing
  }),
});

const selection = ${JSON.stringify(demo.selection, null, 2)};
// Request grants and validate before starting a session.
// No turn is started merely by creating or selecting a client.`}</pre>
      </Section>
      <Section title="Known preview boundaries" description="These are implementation limitations, not hidden successful fallbacks.">
        <p className="text-sm text-muted-foreground">
          This showcase consumes the public SDK. Native discovery, model/account metadata and execution are reported per adapter. Inspect the negotiated methods
          and repository support matrix; unavailable features are not silently substituted.
        </p>
      </Section>
    </div>
  );
}
