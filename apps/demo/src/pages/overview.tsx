import { CostTrend } from '@/components/common/cost-trend';
import { ShieldCheckIcon, GaugeIcon } from '@phosphor-icons/react';
import { Link } from '@tanstack/react-router';
import type { Connection } from '@local-ai-kit/sdk';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { EmptyState } from '@/components/common/empty-state';
import { ProviderGlyph } from '@/components/common/provider-glyph';
import { Section } from '@/components/common/section';
import { known } from '@/lib/format';
import { useDemo } from '@/state/context';

import { percent, readingOf } from '@/lib/usage';

const isVerified = (connection: Connection) => known(connection.auth) === 'authenticated' && connection.auth.state === 'known' && connection.auth.freshness === 'fresh';
const isStale = (connection: Connection) => known(connection.auth) === 'authenticated' && connection.auth.state === 'known' && connection.auth.freshness === 'stale';

function ProfileCard({ connection, stale }: { connection: Connection; stale?: boolean }) {
  const providers = useDemo((state) => state.snapshot?.providers);
  const provider = providers?.find((item) => item.id === connection.providerId)?.name ?? connection.providerId;
  return (
    <div className="flex items-start gap-3 rounded-xl border bg-background/40 p-4">
      <ProviderGlyph id={connection.providerId} name={provider} />
      <div className="min-w-0">
        <h3 className="text-sm font-semibold">{connection.displayName}</h3>
        <div className="my-2">
          <Badge variant={stale ? 'warning' : 'success'}>{stale ? 'Previously authenticated' : 'Authenticated'}</Badge>
        </div>
        {stale ? <p className="text-xs text-muted-foreground">{`${provider} · Check is stale`}</p> : null}
        <p className="text-xs text-muted-foreground">{known(connection.plan)?.label ?? 'Plan not reported'}</p>
      </div>
    </div>
  );
}

export default function OverviewPage() {
  const snapshot = useDemo((state) => state.snapshot);
  const connections = snapshot?.connections ?? [];
  const modelCount = snapshot?.models.length ?? 0;
  const mode = useDemo((state) => state.mode);
  const verified = connections.filter(isVerified);
  const previous = connections.filter(isStale);
  const pools = snapshot?.quotaPools ?? [];
  const issues = snapshot?.issues?.filter((issue) => issue.state === 'open') ?? [];
  const windows = pools.flatMap((pool) => pool.windows.map((window) => ({ pool, window })));

  return (
    <div className="grid gap-8">
      <div className="flex flex-wrap items-end justify-between gap-4 border-b pb-5">
        <p className="max-w-xl text-base text-muted-foreground">
          {mode === 'synthetic' ? 'Example accounts and allowances. Pair a runtime to see your own.' : 'Your provider-reported access, with unknowns left visible.'}
        </p>
        <div className="flex gap-3">
          <Button render={<Link to="/connections" />} nativeButton={false}>Manage connections</Button>
          <Button variant="outline" render={<Link to="/selector-lab" />} nativeButton={false}>Build with it</Button>
        </div>
      </div>

      <dl className="grid grid-cols-2 gap-6 md:grid-cols-4">
        {[
          ['Profiles', connections.length], ['Authenticated now', verified.length],
          ['Route-scoped models', modelCount], ['Open issues', snapshot?.issues ? issues.length : 'Unknown'],
        ].map(([label, value]) => (
          <div key={label} className="border-l pl-4">
            <dt className="text-sm text-muted-foreground">{label}</dt>
            <dd className="mt-2 text-3xl font-semibold tabular-nums">{value}</dd>
          </div>
        ))}
      </dl>

      <CostTrend />

      <section aria-labelledby="allowances-title">
        <div className="mb-4 flex items-baseline justify-between gap-4">
          <h2 id="allowances-title" className="text-xl font-semibold">Usage and allowances</h2>
          <Link to="/usage" className="text-sm underline underline-offset-4">All usage details</Link>
        </div>
        {windows.length ? (
          <ul className="divide-y border-y">
            {windows.map(({ pool, window }) => {
              const reading = readingOf(window);
              const owners = connections.filter((connection) => pool.connectionIds.includes(connection.id));
              const provider = [...new Set(owners.map((connection) =>
                snapshot?.providers.find((item) => item.id === connection.providerId)?.name ?? connection.providerId
              ))].join(', ') || 'Provider not reported';
              const reset = window.resetsAt ? new Date(window.resetsAt) : null;
              return (
                <li key={`${pool.id}:${window.id}`} className="grid gap-4 py-5 sm:grid-cols-[minmax(0,1fr)_minmax(0,1fr)] xl:grid-cols-[minmax(0,1fr)_minmax(0,1fr)_minmax(0,1fr)]">
                  <div className="min-w-0">
                    <h3 className="font-semibold">{provider}</h3>
                    <p className="text-sm text-muted-foreground">{window.label}</p>
                    <p className="mt-1 text-xs text-muted-foreground">{owners.map((owner) => owner.displayName).join(', ') || 'Connection not reported'}</p>
                  </div>
                  <div>
                    <p className="font-medium tabular-nums">
                      {reading ? reading.fraction === null
                        ? `${reading.used} ${reading.unit}, total unknown`
                        : `${percent(reading.fraction)} used`
                        : 'Usage not reported'}
                    </p>
                    {reading?.fraction != null ? (
                      <meter className="usage-meter mt-2 h-3 w-full" min={0} max={1} value={reading.fraction}
                        aria-label={`${provider} ${window.label} used share`} />
                    ) : null}
                    <p className="mt-1 text-xs text-muted-foreground">{reading?.freshness ?? 'Unknown evidence'}</p>
                  </div>
                  <div className="text-sm text-muted-foreground">
                    {reset && Number.isFinite(reset.getTime()) ? (
                      <p>Resets <time dateTime={window.resetsAt ?? undefined}>{reset.toLocaleString()}</time></p>
                    ) : <p>Reset not reported</p>}
                    <p className="mt-1">{pool.routeIds.length} linked routes; {pool.sharing === 'verified' ? 'shared pool' : 'sharing unknown'}</p>
                  </div>
                </li>
              );
            })}
          </ul>
        ) : (
          <EmptyState icon={GaugeIcon} title="No allowances reported yet"
            description="Connect a runtime and authorize metadata checks. Providers that do not report usage stay unknown." />
        )}
        <p className="mt-3 text-xs text-muted-foreground">Shared pools appear once. Allowances from different providers are never added together.</p>
      </section>

      {issues.length ? (
        <section aria-labelledby="issues-title">
          <h2 id="issues-title" className="mb-3 text-xl font-semibold">Needs attention</h2>
          <ul className="divide-y border-y">
            {issues.map((issue) => <li key={issue.id} className="py-3">
              <p className="font-medium">{issue.title}</p>
              <p className="text-sm text-muted-foreground">{issue.summary}</p>
            </li>)}
          </ul>
          <Link to="/diagnostics" className="mt-3 inline-block text-sm underline">Review diagnostics</Link>
        </section>
      ) : null}

      {verified.length ? (
        <Section title="Verified profiles" description="Only fresh authenticated profiles appear here.">
          <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
            {verified.slice(0, 4).map((connection) => (
              <ProfileCard key={connection.id} connection={connection} />
            ))}
          </div>
        </Section>
      ) : previous.length ? null : (
        <Section title="Verified profiles" description="Only fresh authenticated profiles appear here.">
          <EmptyState icon={ShieldCheckIcon} title="No authenticated profiles yet" description="Detected clients remain unverified until an explicit account and model check succeeds." />
        </Section>
      )}

      {previous.length ? (
        <Section title="Previously authenticated" description="These profiles passed an earlier check, but that authentication evidence is stale.">
          <div data-testid="previously-authenticated-overview" className="grid grid-cols-1 gap-4 md:grid-cols-2">
            {previous.slice(0, 4).map((connection) => (
              <ProfileCard key={connection.id} connection={connection} stale />
            ))}
          </div>
        </Section>
      ) : null}
    </div>
  );
}
