import type * as React from 'react';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { Select, Textarea, CheckRow } from './ui/field';
import { Empty } from './ui/feedback';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from './ui/table';
import { cn } from '../lib/cn';
import { capNames, reasonLabel } from '../lib/format';
import { validateOption, type Candidate } from '@local-ai-kit/selector-core';
import type { Demo } from '../state/demo';

export const variants: [string, string, string][] = [
  ['rail', 'Provider rail', 'Providers first, models inside'],
  ['compact', 'Compact', 'A small composer control'],
  ['palette', 'Command palette', 'Keyboard-first global search'],
  ['capability', 'Capability first', 'Choose what the task needs'],
  ['compare', 'Compare', 'Inspect routes side by side'],
  ['allowance', 'Allowance aware', 'Keep billing sources visible'],
  ['preset', 'Task presets', 'Requirements, not model hype'],
  ['mobile', 'Mobile drill-down', 'One decision at a time'],
];

const capabilityOptions: [string, string][] = [
  ['generation.text', 'Text generation'],
  ['input.text', 'Text input'],
  ['input.image', 'Image input'],
  ['structured.output', 'Structured output'],
  ['tools', 'Tool support'],
  ['offline', 'Offline operation'],
];

export function ModelRow({ candidate, demo, onSelect }: { candidate: Candidate; demo: Demo; onSelect?: (candidate: Candidate) => void }): React.ReactElement {
  const selected = demo.selection?.modelId === candidate.model.id;
  return (
    <div data-model-row="" className={cn('model-row my-1 min-w-0 rounded-lg border border-transparent', selected && 'selected border-teal-300/40 bg-teal-400/10')}>
      <button
        type="button"
        onClick={() => (onSelect ?? demo.select)(candidate)}
        className="model-button block w-full rounded-lg border border-transparent bg-transparent px-2.5 py-3 text-left hover:border-transparent"
        data-testid={candidate.model.id}
        data-eligible={String(candidate.eligible)}
        aria-pressed={selected}
      >
        <div className="model-line flex items-start justify-between gap-2.5">
          <strong className="text-xs font-semibold break-words">{candidate.model.name}</strong>
          {selected ? <Badge variant="accent">Selected</Badge> : null}
        </div>
        <div className="model-meta my-1.5 flex flex-wrap gap-x-3 gap-y-1 text-[10px] text-muted-foreground">
          <span>{candidate.connection.displayName}</span>
          <span>{candidate.billingLabel}</span>
        </div>
        <div className="capabilities flex flex-wrap gap-1">
          {candidate.model.capabilities.map((c) => (
            <Badge key={c.id} variant={c.support === 'supported' ? 'supported' : 'muted'}>
              {capNames[c.id] ?? c.id}
            </Badge>
          ))}
        </div>
        {!candidate.eligible ? (
          <div className="reason mt-2 text-[0.6rem] break-words text-amber-200">
            {candidate.reasons.map(reasonLabel).join(' · ')}
          </div>
        ) : null}
      </button>
    </div>
  );
}

function ModelList({ candidates, demo, onSelect }: { candidates: Candidate[]; demo: Demo; onSelect?: (candidate: Candidate) => void }): React.ReactElement {
  if (!candidates.length) return <EmptyRoutes demo={demo} />;
  return (
    <div className="model-list p-1.5">
      {candidates.slice(0, 50).map((c) => (
        <ModelRow key={c.key} candidate={c} demo={demo} onSelect={onSelect} />
      ))}
      {candidates.length > 50 ? (
        <p className="list-footnote px-4 py-2.5 text-xs text-muted-foreground">
          {`Showing 50 of ${candidates.length.toLocaleString()} matches. Narrow the search to inspect the rest.`}
        </p>
      ) : null}
    </div>
  );
}

function EmptyRoutes({ demo }: { demo: Demo }): React.ReactElement {
  if (!demo.all.length) {
    return (
      <Empty>
        <h3 className="text-foreground">No routes match the current filters.</h3>
        <p className="text-xs">Check account and model metadata on an authorized connection to load routes.</p>
      </Empty>
    );
  }
  if (demo.required.length) {
    return (
      <Empty>
        <h3 className="text-foreground">No routes have confirmed all selected capabilities.</h3>
        <p className="text-xs">Unknown support is not treated as supported.</p>
      </Empty>
    );
  }
  return (
    <Empty>
      <h3 className="text-foreground">No routes match the current filters.</h3>
      <p className="text-xs">Clear filters or broaden the search to see more routes.</p>
    </Empty>
  );
}

function ProviderRail({ demo }: { demo: Demo }): React.ReactElement {
  const groups: [string, string][] = [
    ['all', 'All connections'],
    ['favorites', 'Favorites'],
    ...demo.providers.map((p: any) => [p.id, p.name] as [string, string]),
  ];
  return (
    <div data-testid="provider-rail" className="provider-rail border-r bg-card/60 p-2">
      {groups.map(([id, name]) => {
        const count = id === 'favorites' ? demo.favorites.length : id === 'all' ? demo.all.length : demo.all.filter((c) => c.route.providerId === id).length;
        return (
          <Button
            key={id}
            variant="ghost"
            onClick={() => demo.patch({ provider: id })}
            data-provider={id}
            aria-label={name}
            aria-pressed={demo.provider === id}
            className={cn(
              'provider mb-1 h-auto w-full justify-start px-2 py-2.5 text-left text-[11px] text-muted-foreground',
              demo.provider === id && 'active bg-teal-400/10 text-teal-200 hover:bg-teal-400/10 hover:text-teal-200',
            )}
          >
            <span className="provider-content flex min-w-0 items-center gap-1.5">
              <span aria-hidden="true" className="provider-glyph w-5 shrink-0 text-center text-sm font-bold">
                {id === 'favorites' ? '★' : id === 'all' ? '≋' : name.slice(0, 1)}
              </span>
              <span className="truncate">{name}</span>
              <span className="provider-count ml-auto font-mono text-[0.55rem]">{count}</span>
            </span>
          </Button>
        );
      })}
    </div>
  );
}

export function SelectorView({ demo, onOpenPalette }: { demo: Demo; onOpenPalette: () => void }): React.ReactElement {
  const groupFiltered =
    demo.provider === 'favorites'
      ? demo.candidates.filter((c) => demo.favorites.includes(c.key))
      : demo.query.trim() || demo.provider === 'all'
        ? demo.candidates
        : demo.candidates.filter((c) => c.route.providerId === demo.provider);

  let content: React.ReactNode;
  if (demo.variant === 'rail') {
    content = (
      <div className="rail-layout grid min-h-[31rem] grid-cols-[10rem_minmax(0,1fr)] sm:grid-cols-[10rem_minmax(0,1fr)]">
        <ProviderRail demo={demo} />
        <div className="rail-models min-w-0">
          <div className="list-heading flex items-center justify-between gap-2.5 border-b px-4 py-4 text-xs">
            <strong>
              {demo.query
                ? 'Search all connections'
                : demo.provider === 'favorites'
                  ? 'Favorite routes'
                  : (demo.providers.find((p: any) => p.id === demo.provider)?.name ?? 'All connections')}
            </strong>
            <Badge>{String(groupFiltered.length)}</Badge>
          </div>
          <ModelList candidates={groupFiltered} demo={demo} />
        </div>
      </div>
    );
  } else if (demo.variant === 'compact') {
    content = (
      <div className="variant-stage p-7">
        <div className="mini-composer mb-5 rounded-xl border bg-background/60 p-6">
          <h3 className="mb-2 text-sm font-semibold">A model picker that fits anywhere</h3>
          <p className="mb-4 text-xs text-muted-foreground">Provider, account, and model stay together—even in one compact control.</p>
          <Select
            aria-label="Provider and model"
            className="compact-select"
            value={demo.selection?.modelId ?? ''}
            onChange={(event) => {
              const found = demo.all.find((c) => c.model.id === event.target.value);
              if (found) demo.select(found);
            }}
          >
            {demo.candidates.length ? (
              demo.providers.map((p: any) => (
                <optgroup key={p.id} label={p.name}>
                  {demo.candidates
                    .filter((c) => c.route.providerId === p.id)
                    .map((c) => (
                      <option key={c.key} value={c.model.id}>
                        {`${c.model.name} · ${c.connection.displayName}${c.eligible ? '' : ' · needs attention'}`}
                      </option>
                    ))}
                </optgroup>
              ))
            ) : (
              <option value="" disabled>
                No model routes reported
              </option>
            )}
          </Select>
          {!demo.candidates.length ? null : !demo.selection ? (
            <p className="mt-2 text-xs text-muted-foreground">Choose a model and account</p>
          ) : null}
          <Textarea placeholder="Your application’s prompt goes here. This preview does not send it." aria-label="Prompt preview" rows={4} className="mt-4" readOnly={false} />
        </div>
        <p className="text-xs text-muted-foreground">The configuration panel uses the same native option descriptors as every other selector.</p>
      </div>
    );
  } else if (demo.variant === 'palette') {
    content = (
      <div className="variant-stage p-7">
        <div className="palette-promo mb-5 rounded-xl border bg-background/60 p-6">
          <Badge variant="accent" className="mb-4">
            Keyboard first
          </Badge>
          <h3 className="mb-2 text-sm font-semibold">Find a route without leaving the keyboard.</h3>
          <p className="mb-4 text-xs text-muted-foreground">Search model names, provider names, accounts, and native identifiers.</p>
          <Button variant="default" onClick={onOpenPalette}>
            Open command palette
          </Button>{' '}
          <kbd className="ml-3 inline-block rounded border border-border px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">Ctrl / ⌘ + K</kbd>
        </div>
        <ModelList candidates={demo.candidates.slice(0, 4)} demo={demo} />
      </div>
    );
  } else if (demo.variant === 'capability') {
    content = (
      <div>
        <div className="requirements flex flex-wrap gap-4 border-b px-4 py-3.5">
          {capabilityOptions.map(([id, label]) => (
            <CheckRow
              key={id}
              label={label}
              checked={demo.required.includes(id)}
              onChange={(event) => {
                const checked = (event.target as HTMLInputElement).checked;
                demo.patch({ required: checked ? [...demo.required, id] : demo.required.filter((r) => r !== id) });
              }}
            />
          ))}
        </div>
        <ModelList candidates={demo.candidates} demo={demo} />
      </div>
    );
  } else if (demo.variant === 'compare') {
    content = (
      <Table>
        <TableHeader>
          <TableRow>
            {['Model / route', 'Billing', 'Generation', 'Text input', 'Image input', 'Structure', 'Tools', 'Select'].map((t) => (
              <TableHead key={t}>{t}</TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {demo.candidates.length ? (
            demo.candidates.slice(0, 50).map((c) => (
              <TableRow key={c.key}>
                <TableCell>
                  <strong className="text-xs">{c.model.name}</strong>
                  <small className="mt-1 block text-[0.55rem] text-muted-foreground">{c.connection.displayName}</small>
                </TableCell>
                <TableCell>{c.billingLabel}</TableCell>
                {['generation.text', 'input.text', 'input.image', 'structured.output', 'tools'].map((id) => (
                  <TableCell key={id}>{c.model.capabilities.find((cap) => cap.id === id)?.support ?? 'unknown'}</TableCell>
                ))}
                <TableCell>
                  <Button variant="outline" size="sm" onClick={() => demo.select(c)} data-testid={c.model.id} aria-label={`Inspect ${c.model.name} through ${c.connection.displayName}`}>
                    Inspect
                  </Button>
                </TableCell>
              </TableRow>
            ))
          ) : (
            <TableRow className="compare-empty">
              <TableCell colSpan={8} className="p-0 text-center">
                <EmptyRoutes demo={demo} />
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
    );
  } else if (demo.variant === 'allowance') {
    const labels = [...new Set(demo.candidates.map((c) => c.billingLabel))];
    content = labels.length ? (
      <div className="allowance-groups">
        {labels.map((label) => (
          <section key={label} className="allowance-group border-b px-3 py-5 last:border-b-0">
            <h3 className="px-2 text-sm font-semibold">{label}</h3>
            <p className="px-2 text-[11px] text-muted-foreground">
              {label === 'Included allowance'
                ? 'Existing allowance, not unlimited or free.'
                : label === 'Metered access'
                  ? 'A separate billing decision is required.'
                  : 'Use the evidence reported by this connection.'}
            </p>
            <ModelList candidates={demo.candidates.filter((c) => c.billingLabel === label)} demo={demo} />
          </section>
        ))}
      </div>
    ) : (
      <div className="variant-stage p-7">
        <EmptyRoutes demo={demo} />
      </div>
    );
  } else if (demo.variant === 'preset') {
    const presets: [string, string[]][] = [
      ['Text task', ['generation.text']],
      ['Structured result', ['generation.text', 'structured.output']],
      ['Offline task', ['generation.text', 'offline']],
      ['Tool-assisted task', ['generation.text', 'tools']],
    ];
    content = (
      <div>
        <div className="preset-grid grid grid-cols-1 gap-2.5 p-4 sm:grid-cols-2">
          {presets.map(([name, required]) => (
            <Button
              key={name}
              variant="outline"
              onClick={() => demo.patch({ required, onlyEligible: true })}
              className="preset h-auto flex-col items-start gap-1 p-3.5 text-left text-xs"
            >
              <strong>{name}</strong>
              <small className="mt-1 text-[0.55rem] text-muted-foreground">{required.map((id) => capNames[id] ?? id).join(' + ')}</small>
            </Button>
          ))}
        </div>
        <p className="list-footnote px-4 py-2.5 text-xs text-muted-foreground">
          Presets filter requirements. They do not automatically select an account, start a turn, or claim a model is best.
        </p>
        <ModelList candidates={demo.candidates} demo={demo} />
      </div>
    );
  } else {
    content = <MobileDrilldown demo={demo} />;
  }

  return (
    <div id="variant-view" data-current-variant={demo.variant} className="selector-surface min-w-0 overflow-hidden rounded-xl border bg-card">
      {content}
    </div>
  );
}

function MobileDrilldown({ demo }: { demo: Demo }): React.ReactElement {
  const back = (): void => {
    demo.patch({ mobileStep: Math.max(0, demo.mobileStep - 1) });
  };
  let body: React.ReactNode;
  if (demo.mobileStep === 0) {
    body = demo.providers.map((p: any) => (
      <Button
        key={p.id}
        variant="outline"
        onClick={() => demo.patch({ mobileProvider: p.id, mobileStep: 1 })}
        className="drill-row mb-1.5 block w-full text-left"
      >
        {p.name}
      </Button>
    ));
  } else if (demo.mobileStep === 1) {
    body = demo.connections
      .filter((c: any) => c.providerId === demo.mobileProvider)
      .map((c: any) => (
        <Button
          key={c.id}
          variant="outline"
          onClick={() => demo.patch({ mobileConnection: c.id, mobileStep: 2 })}
          className="drill-row mb-1.5 block w-full text-left"
        >
          {c.displayName}
        </Button>
      ));
  } else if (demo.mobileStep === 2) {
    body = (
      <ModelList
        candidates={demo.candidates.filter((c) => c.connection.id === demo.mobileConnection)}
        demo={demo}
        onSelect={(candidate) => {
          demo.select(candidate);
          demo.patch({ mobileStep: 3 });
        }}
      />
    );
  } else {
    body = (
      <div className="drill-finished p-4 text-xs">
        <Badge variant="accent">Route selected</Badge>
        <h3 className="mt-4 text-sm font-semibold">Now choose its native options.</h3>
        <p className="text-muted-foreground">The configuration panel below retains the exact account, model identifier, and supported settings.</p>
        <Button
          variant="outline"
          size="sm"
          className="mt-3"
          onClick={() => document.querySelector('#configuration-panel')?.scrollIntoView({ behavior: 'smooth' })}
        >
          Review configuration
        </Button>
      </div>
    );
  }
  return (
    <div className="phone-stage flex justify-center p-4 sm:p-7">
      <div className="phone w-full max-w-[22rem] overflow-hidden rounded-[1.25rem] border bg-background">
        <div className="phone-header flex items-center justify-between gap-2.5 border-b p-4 text-xs">
          {demo.mobileStep ? (
            <Button variant="outline" size="sm" onClick={back}>
              ← Back
            </Button>
          ) : (
            <span>Choose a provider</span>
          )}
          <Badge>{`${demo.mobileStep + 1} / 4`}</Badge>
        </div>
        <div className="phone-body min-h-[17rem] p-2">{body}</div>
      </div>
    </div>
  );
}

export function SelectorsPage({ demo, onOpenPalette }: { demo: Demo; onOpenPalette: () => void }): React.ReactElement {
  return (
    <div>
      <div aria-label="Selector variants" className="variant-tabs mb-5 grid grid-cols-2 gap-1.5 sm:grid-cols-4 2xl:grid-cols-8">
        {variants.map(([id, label, description], i) => (
          <Button
            key={id}
            variant="outline"
            onClick={() => demo.patch({ variant: id })}
            data-variant={id}
            aria-pressed={demo.variant === id}
            title={description}
            className={cn(
              'variant-tab h-auto flex-col items-start gap-0.5 p-2.5 text-left text-[11px]',
              demo.variant === id && 'active border-teal-300/40 bg-teal-400/10 text-teal-200 hover:bg-teal-400/10 hover:text-teal-200',
            )}
          >
            <span className="flex items-center gap-2.5">
              <small className="text-[10px] text-muted-foreground">{String(i + 1).padStart(2, '0')}</small>
              {label}
            </span>
          </Button>
        ))}
      </div>
      <div className="catalog-toolbar mb-4 flex flex-wrap items-center gap-x-4 gap-y-1">
        <div className="search-wrap relative min-w-60 flex-1">
          <span aria-hidden="true" className="absolute top-1 left-3 text-2xl text-muted-foreground">
            ⌕
          </span>
          <input
            type="search"
            id="catalog-query"
            placeholder="Search models, providers, accounts…"
            aria-label="Search models"
            value={demo.query}
            onChange={(event) => demo.patch({ query: (event.target as HTMLInputElement).value })}
            className="w-full rounded-lg border border-input bg-card py-2.5 pr-3 pl-9 text-sm outline-none placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring"
          />
        </div>
        <CheckRow
          label="Only eligible routes"
          checked={demo.onlyEligible}
          onChange={(event) => demo.patch({ onlyEligible: (event.target as HTMLInputElement).checked })}
        />
        <CheckRow
          label="Include metered routes"
          checked={demo.allowPaid}
          onChange={(event) => demo.patch({ allowPaid: (event.target as HTMLInputElement).checked })}
        />
      </div>
      <div className="selector-grid grid items-start gap-5 lg:grid-cols-[minmax(0,1fr)_19rem]">
        <SelectorView demo={demo} onOpenPalette={onOpenPalette} />
        <ConfigurationPanel demo={demo} />
      </div>
    </div>
  );
}

export function ConfigurationPanel({ demo }: { demo: Demo }): React.ReactElement {
  const selected = demo.all.find((c) => c.model.id === demo.selection?.modelId);
  if (!selected) {
    return (
      <aside id="configuration-panel" className="configuration card min-w-0 rounded-xl border bg-card p-5">
        <Empty>
          <h3 className="text-foreground">{demo.all.length ? 'Choose a model' : 'No model route available'}</h3>
          <p className="text-xs">
            {demo.all.length
              ? 'Inspect a route to see its native settings and execution identity.'
              : 'Check account and model metadata on an authorized connection to load a catalog.'}
          </p>
        </Empty>
        <Button variant="outline" size="sm" className="mt-3" onClick={() => demo.patch({ page: 'Connections' })}>
          Open Connections
        </Button>
      </aside>
    );
  }
  const { model, route, connection } = selected;
  const selection = demo.selection as NonNullable<Demo['selection']>;

  const setOption = (descriptor: any, value: unknown): void => {
    if (value !== undefined) {
      const error = validateOption(descriptor, value);
      if (error) {
        demo.patch({ notice: error });
        return;
      }
      if ((descriptor.billingSensitive || descriptor.permissionSensitive) && !demo.sensitiveConsent) {
        demo.patch({ notice: 'Review and allow sensitive configuration changes before changing this option.' });
        return;
      }
    }
    const options = { ...(selection.options as Record<string, unknown>) };
    if (value === undefined) delete options[descriptor.id];
    else options[descriptor.id] = value;
    demo.patch({ selection: { ...selection, options }, notice: '' });
  };

  const context = (model as any).contextWindow?.state === 'known' ? (model as any).contextWindow.value : undefined;

  return (
    <aside id="configuration-panel" className="configuration card min-w-0 rounded-xl border bg-card p-5">
      <div className="config-heading mb-2.5 flex items-baseline justify-between gap-2">
        <span className="eyebrow text-[0.5rem] font-bold tracking-[0.15rem] text-muted-foreground">Selected route</span>
        <Button
          variant="ghost"
          size="sm"
          data-testid="favorite-current"
          aria-label={demo.favorites.includes(selected.key) ? 'Remove current route from favorites' : 'Save current route to favorites'}
          onClick={() => {
            demo.patch({
              favorites: demo.favorites.includes(selected.key)
                ? demo.favorites.filter((k) => k !== selected.key)
                : [...demo.favorites, selected.key],
            });
          }}
          className="favorite px-2 py-1 text-[10px] text-muted-foreground"
        >
          {demo.favorites.includes(selected.key) ? '★ Saved' : '☆ Save'}
        </Button>
      </div>
      <h2 className="text-lg font-semibold tracking-tight break-words">{model.name}</h2>
      <p className="text-xs text-muted-foreground">{connection.displayName}</p>
      <div className="config-badges my-4 flex flex-wrap gap-1.5">
        <Badge variant="accent">{selected.billingLabel}</Badge>
        <Badge variant={selected.eligible ? 'supported' : 'warning'}>{selected.eligible ? 'Eligible in snapshot' : 'Needs attention'}</Badge>
      </div>
      <dl className="facts my-5 grid grid-cols-[auto_minmax(0,1fr)] gap-x-3.5 gap-y-2.5 text-[11px]">
        <dt className="text-muted-foreground">Provider</dt>
        <dd className="text-right break-words">{demo.providers.find((p: any) => p.id === route.providerId)?.name ?? route.providerId}</dd>
        <dt className="text-muted-foreground">Native model</dt>
        <dd className="mono text-right break-all">{model.nativeId}</dd>
        <dt className="text-muted-foreground">Context</dt>
        <dd className="text-right">{context ? `${Number(context).toLocaleString()} · reported` : 'Not reported'}</dd>
        <dt className="text-muted-foreground">Source</dt>
        <dd className="text-right">{demo.mode === 'synthetic' ? 'Synthetic example' : 'Connected runtime'}</dd>
      </dl>
      <div className="config-options mt-5 border-t pt-4">
        <h3 className="mb-2 text-sm font-semibold">Native options</h3>
        {model.options.length ? (
          model.options.map((descriptor: any) => (
            <OptionField key={descriptor.id} descriptor={descriptor} value={(selection.options as Record<string, unknown>)[descriptor.id]} onSet={setOption} />
          ))
        ) : (
          <p className="text-xs text-muted-foreground">This model advertises no editable options. Nothing is assumed.</p>
        )}
      </div>
      {model.options.some((o: any) => o.billingSensitive || o.permissionSensitive) ? (
        <label className="check mt-4 flex items-center gap-2 text-[11px]">
          <input
            type="checkbox"
            checked={demo.sensitiveConsent}
            onChange={(event) => demo.patch({ sensitiveConsent: (event.target as HTMLInputElement).checked })}
            className="h-4 w-4 accent-teal-300"
          />
          Allow sensitive configuration changes
        </label>
      ) : null}
      {selected.reasons.length ? (
        <div className="notice mt-4 rounded-lg border border-amber-300/25 bg-amber-400/10 p-3.5 text-xs text-amber-100">
          {selected.reasons.map(reasonLabel).join(' · ')}
        </div>
      ) : null}
      <details className="mt-5 border-t pt-3.5">
        <summary className="cursor-pointer text-[11px] text-muted-foreground">Exact selection payload</summary>
        <pre id="selection-json" className="code mt-3">
          {JSON.stringify(selection, null, 2)}
        </pre>
      </details>
      <p className="fine-print mt-4 text-[10px] leading-relaxed text-muted-foreground">
        Selection is not authorization. The runtime must recheck access, settings, and permissions before execution.
      </p>
    </aside>
  );
}

function OptionField({ descriptor, value, onSet }: { descriptor: any; value: unknown; onSet: (descriptor: any, value: unknown) => void }): React.ReactElement {
  const id = `native-${descriptor.id}`;
  const control = descriptor.control;
  let input: React.ReactNode;
  if (control.type === 'select') {
    input = (
      <Select
        id={id}
        aria-label={descriptor.label}
        value={value === undefined ? '' : JSON.stringify(value)}
        onChange={(event) => {
          const raw = (event.target as HTMLSelectElement).value;
          onSet(descriptor, raw === '' ? undefined : JSON.parse(raw));
        }}
      >
        <option value="">
          {control.defaultValue === undefined
            ? 'Use provider default'
            : `Provider default (${control.choices.find((c: any) => c.value === control.defaultValue)?.label ?? control.defaultValue})`}
        </option>
        {control.choices.map((choice: any) => (
          <option key={String(choice.value)} value={JSON.stringify(choice.value)} disabled={Boolean(choice.disabledReason)}>
            {choice.label + (choice.disabledReason ? ` — ${choice.disabledReason}` : '')}
          </option>
        ))}
      </Select>
    );
  } else if (control.type === 'boolean') {
    input = (
      <Select
        id={id}
        aria-label={descriptor.label}
        value={value === undefined ? '' : String(value)}
        onChange={(event) => {
          const raw = (event.target as HTMLSelectElement).value;
          onSet(descriptor, raw === '' ? undefined : raw === 'true');
        }}
      >
        <option value="">Use provider default</option>
        <option value="true">Enabled</option>
        <option value="false">Disabled</option>
      </Select>
    );
  } else if (control.type === 'number') {
    input = (
      <input
        id={id}
        aria-label={descriptor.label}
        type="number"
        min={control.min}
        max={control.max}
        step={control.step}
        value={(value as number) ?? ''}
        placeholder="Provider default"
        onChange={(event) => {
          const target = event.target as HTMLInputElement;
          onSet(descriptor, target.value === '' ? undefined : target.valueAsNumber);
        }}
        className="flex h-10 w-full rounded-md border border-input bg-background/60 px-3 py-2 text-sm outline-none placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring"
      />
    );
  } else if (control.type === 'text') {
    input = (
      <input
        id={id}
        aria-label={descriptor.label}
        type="text"
        maxLength={control.maxLength}
        value={(value as string) ?? ''}
        placeholder="Provider default"
        onChange={(event) => onSet(descriptor, (event.target as HTMLInputElement).value === '' ? undefined : (event.target as HTMLInputElement).value)}
        className="flex h-10 w-full rounded-md border border-input bg-background/60 px-3 py-2 text-sm outline-none placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring"
      />
    );
  } else {
    input = <div className="unavailable text-xs text-muted-foreground">{`Inspectable, not editable: ${control.nativeType}`}</div>;
  }
  const note = [`${descriptor.scope} option`, descriptor.billingSensitive ? 'May affect billing' : null, descriptor.permissionSensitive ? 'Changes permissions' : null]
    .filter(Boolean)
    .join(' · ');
  return (
    <div className="field mb-4 flex flex-col gap-1.5 text-xs">
      <label htmlFor={id}>{descriptor.label}</label>
      {input}
      <small className="text-[10px] text-muted-foreground">{note}</small>
    </div>
  );
}
