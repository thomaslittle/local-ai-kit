import { ArrowRightIcon, CheckCircleIcon, CubeIcon, CursorClickIcon, PlayIcon, BinocularsIcon, ScanIcon, ShieldCheckIcon, UsersIcon, type Icon } from '@phosphor-icons/react';
import { Link } from '@tanstack/react-router';
import type { Connection } from '@local-ai-kit/sdk';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { EmptyState } from '@/components/common/empty-state';
import { ProviderGlyph } from '@/components/common/provider-glyph';
import { Section } from '@/components/common/section';
import { billingVariant, known } from '@/lib/format';
import { useAllCandidates, useDemo } from '@/state/context';

const isVerified = (connection: Connection) => known(connection.auth) === 'authenticated' && connection.auth.state === 'known' && connection.auth.freshness === 'fresh';
const isStale = (connection: Connection) => known(connection.auth) === 'authenticated' && connection.auth.state === 'known' && connection.auth.freshness === 'stale';

const STEPS: { title: string; detail: string; icon: Icon }[] = [
  { title: 'Discover connections', detail: 'Passive, staged, and permissioned.', icon: BinocularsIcon },
  { title: 'Inspect the evidence', detail: 'Every field says how it is known.', icon: ScanIcon },
  { title: 'Select a route', detail: 'Account, model, and billing stay distinct.', icon: CursorClickIcon },
  { title: 'Run with permission', detail: 'Selection is never authorization.', icon: PlayIcon },
];

function Metric({ icon: Icon, value, label, note }: { icon: Icon; value: number; label: string; note: string }) {
  return (
    <Card className="group/metric relative gap-1 bg-card/70 p-5 backdrop-blur-sm [--card-spacing:--spacing(0)]">
      <span className="mb-3 grid size-9 place-items-center rounded-xl bg-primary/10 text-brand-foreground ring-1 ring-primary/20 transition-transform group-hover/metric:scale-105">
        <Icon className="size-4.5" aria-hidden="true" />
      </span>
      <span className="text-4xl font-semibold tracking-tight tabular-nums text-gradient">{value.toLocaleString()}</span>
      <span className="text-sm font-medium">{label}</span>
      <small className="text-[0.7rem] text-muted-foreground">{note}</small>
    </Card>
  );
}

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
  const all = useAllCandidates();
  const connections = useDemo((state) => state.snapshot?.connections ?? []);
  const modelCount = useDemo((state) => state.snapshot?.models.length ?? 0);
  const mode = useDemo((state) => state.mode);
  const verified = connections.filter(isVerified);
  const previous = connections.filter(isStale);
  const note = mode === 'synthetic' ? 'Example data' : 'Runtime-reported';
  const preview = all.filter((candidate) => candidate.eligible).slice(0, 3);

  return (
    <div className="grid gap-5">
      <Card className="relative isolate gap-0 overflow-hidden bg-card/70 p-0 backdrop-blur-sm [--card-spacing:--spacing(0)]">
        <div aria-hidden="true" className="grid-fade absolute inset-0 -z-10 opacity-70" />
        <div aria-hidden="true" className="absolute -top-24 -right-16 -z-10 size-72 rounded-full bg-primary/20 blur-3xl" />
        <div className="grid gap-8 p-6 md:p-8 lg:grid-cols-[minmax(0,1.1fr)_minmax(0,0.9fr)] lg:items-center">
          <div>
            <Badge variant="brand" className="mb-4">
              Local-first · evidence-first
            </Badge>
            <p className="text-gradient text-2xl font-semibold tracking-tight sm:text-3xl">Build around access. Not assumptions.</p>
            <p className="mt-3 max-w-xl text-sm leading-relaxed text-muted-foreground sm:text-base">
              Inspect first, select explicitly, execute only with permission. Provider, account, route, model and billing identity travel together, so an app never guesses what you have access to.
            </p>
            <div className="mt-6 flex flex-wrap gap-3">
              <Button size="lg" render={<Link to="/selector-lab" />} nativeButton={false} className="h-10 px-4">
                Explore model selectors
                <ArrowRightIcon data-icon="inline-end" />
              </Button>
              <Button size="lg" variant="outline" render={<Link to="/connections" />} nativeButton={false} className="h-10 px-4">
                Inspect connections
              </Button>
            </div>
          </div>
          <div aria-hidden="true" className="grid gap-2.5 [perspective:1200px]">
            {preview.map((candidate, index) => (
              <div
                key={candidate.key}
                className="flex items-center gap-3 rounded-xl border bg-background/60 p-3 shadow-lg backdrop-blur-md transition-transform duration-300 hover:-translate-y-0.5"
                style={{ marginLeft: `${index * 1.25}rem`, marginRight: `${(preview.length - 1 - index) * 1.25}rem` }}
              >
                <ProviderGlyph id={candidate.route.providerId} name={candidate.route.displayName || candidate.route.providerId} />
                <span className="grid min-w-0 flex-1">
                  <strong className="truncate text-sm">{candidate.model.name}</strong>
                  <small className="truncate text-xs text-muted-foreground">{candidate.connection.displayName}</small>
                </span>
                <Badge variant={billingVariant(candidate.billingLabel)}>{candidate.billingLabel}</Badge>
              </div>
            ))}
          </div>
        </div>
      </Card>

      <div className="grid grid-cols-2 gap-4 xl:grid-cols-4">
        <Metric icon={UsersIcon} value={connections.length} label="Detected profiles" note={note} />
        <Metric icon={ShieldCheckIcon} value={verified.length} label="Verified profiles" note={note} />
        <Metric icon={CubeIcon} value={modelCount} label="Route-scoped models" note={note} />
        <Metric icon={CheckCircleIcon} value={all.filter((candidate) => candidate.eligible).length} label="Eligible selections" note={note} />
      </div>

      <Section title="One catalog. Every application." description="Inspect first, select explicitly, execute only with permission.">
        <ol className="grid grid-cols-2 gap-4 xl:grid-cols-4">
          {STEPS.map((step, index) => (
            <li key={step.title} className="relative rounded-xl border bg-background/40 p-4">
              <div className="mb-3 flex items-center gap-2.5">
                <span className="grid size-8 place-items-center rounded-lg bg-primary/10 text-brand-foreground ring-1 ring-primary/20">
                  <step.icon className="size-4" aria-hidden="true" />
                </span>
                <Badge variant="brand">{String(index + 1)}</Badge>
              </div>
              <h3 className="text-sm font-semibold">{step.title}</h3>
              <p className="mt-1 text-xs text-muted-foreground">{step.detail}</p>
            </li>
          ))}
        </ol>
      </Section>

      <Section title="Live showroom" description="This page uses the public SDK against the native local service.">
        <p className="mb-4 max-w-3xl text-sm text-muted-foreground">
          Passive discovery reports installed clients. Account access, model catalogs, and usage remain tied to evidence from authorized probes. Empty results stay empty; no example records are inserted.
        </p>
        <Button render={<Link to="/selector-lab" />} nativeButton={false}>
          Explore model selectors
        </Button>
      </Section>

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
