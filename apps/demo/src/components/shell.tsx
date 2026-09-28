import { useEffect, useRef, useState } from 'react';
import type * as React from 'react';
import { Box, ClipboardList, FlaskConical, Gauge, LayoutGrid, MessagesSquare, ScanSearch } from 'lucide-react';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { cn } from '../lib/cn';
import type { Demo, Page } from '../state/demo';

const pages: { name: Page; icon: typeof Box }[] = [
  { name: 'Overview', icon: LayoutGrid },
  { name: 'Connections', icon: ScanSearch },
  { name: 'Models', icon: Box },
  { name: 'Usage', icon: Gauge },
  { name: 'Selector lab', icon: FlaskConical },
  { name: 'Playground', icon: MessagesSquare },
  { name: 'Diagnostics', icon: ClipboardList },
];

export const descriptions: Record<Page, string> = {
  Overview: 'Discover the shape of your AI access.',
  Connections: 'Every account, profile, and access route stays distinct.',
  Models: 'A route-aware catalog. Native options, never guessed settings.',
  Usage: 'The right units. The right windows. No fictional totals.',
  'Selector lab': 'One selection engine. Eight ways to make it yours.',
  Playground: 'Review session controls. A turn needs an explicit route and authorization.',
  Diagnostics: 'Understand the evidence behind every public response.',
};

export function Shell({ demo, onOpenPalette, children }: { demo: Demo; onOpenPalette: () => void; children: React.ReactNode }): React.ReactElement {
  const live = demo.mode !== 'synthetic';
  return (
    <div className="shell mx-auto flex min-h-screen w-full max-w-[110rem] flex-col md:grid md:grid-cols-[13rem_minmax(0,1fr)]">
      <aside className="sidebar sticky top-0 z-10 flex flex-row items-center gap-2 overflow-x-auto border-b bg-card px-3 py-2 md:h-screen md:flex-col md:items-stretch md:gap-0 md:overflow-y-auto md:overflow-x-hidden md:px-3.5 md:py-6">
        <a
          href="#"
          aria-label="Local AI Kit home"
          onClick={(event) => {
            event.preventDefault();
            demo.patch({ page: 'Overview' });
          }}
          className="brand mr-2 flex shrink-0 items-center gap-2.5 rounded-lg px-1.5 py-1 text-foreground no-underline hover:no-underline md:mx-1.5 md:mb-10 md:mr-0"
        >
          <span aria-hidden="true" className="brand-mark grid h-9 w-9 place-items-center rounded-[0.625rem] border border-teal-300/30 bg-teal-400/10 text-xl text-teal-200">
            ◈
          </span>
          <span className="hidden sm:block md:block">
            <strong className="block text-[1.05rem] font-semibold tracking-tight">Local AI Kit</strong>
            <small className="block text-[0.45rem] font-bold tracking-[0.1rem] text-muted-foreground">DEVELOPER SHOWCASE</small>
          </span>
        </a>
        <div className="sidebar-label mb-3 ml-3 hidden text-[0.55rem] tracking-[0.15rem] text-muted-foreground md:block">WORKSPACE</div>
        <nav aria-label="Showcase sections" className="flex flex-row gap-1 md:grid">
          {pages.map(({ name, icon: Icon }) => (
            <Button
              key={name}
              variant="ghost"
              aria-label={name}
              aria-current={demo.page === name ? 'page' : 'false'}
              onClick={() => {
                // Models is the compare view of the selector; leaving it must not
                // strand Selector lab on the comparison table instead of the rail default.
                demo.patch({
                  page: name,
                  ...(name === 'Models' ? { variant: 'compare' } : name === 'Selector lab' && demo.page === 'Models' ? { variant: 'rail' } : {}),
                });
              }}
              className={cn(
                'nav-link h-auto shrink-0 justify-start gap-2.5 px-2.5 py-2 text-muted-foreground',
                demo.page === name && 'active bg-teal-400/10 text-teal-200 hover:bg-teal-400/10 hover:text-teal-200',
              )}
            >
              <span className="nav-inner flex items-center gap-2.5">
                <Icon aria-hidden="true" className="h-4 w-4" />
                <span className="hidden lg:inline md:inline">{name}</span>
              </span>
            </Button>
          ))}
        </nav>
        <div className="sidebar-bottom mt-auto hidden px-2 pt-5 text-xs md:block">
          <Badge>Implementation preview</Badge>
          <p className="my-3 text-muted-foreground">Build around access. Not assumptions.</p>
          <a href="https://github.com/thomaslittle/local-ai-kit" target="_blank" rel="noreferrer" className="text-teal-200 hover:underline">
            View repository ↗
          </a>
        </div>
      </aside>
      <main className="main min-w-0">
        <header className="topbar flex min-h-[4.5rem] items-center justify-between gap-4 border-b px-4 py-3 md:px-8">
          <span className="breadcrumb hidden items-center gap-3 text-xs text-muted-foreground sm:flex">
            Workspace <span className="text-border">/</span> {demo.page}
          </span>
          <div className="top-actions ml-auto flex w-full items-center justify-between gap-2.5 sm:w-auto sm:justify-end">
            <Button variant="outline" size="sm" onClick={onOpenPalette} aria-label="Open model command palette" className="shortcut text-muted-foreground">
              ⌘ K
            </Button>
            {live ? (
              <Button variant="outline" size="sm" onClick={() => demo.scan()} disabled={!demo.hasClient} className="connect-button text-xs">
                Scan local clients
              </Button>
            ) : (
              <ConnectButton demo={demo} />
            )}
          </div>
        </header>
        <div id="mode-banner" role="status" className={cn('mode-banner flex items-center gap-3 border-b px-4 py-2.5 text-[11px] md:px-8', demo.mode)}>
          <span aria-hidden="true" className="status-dot h-1.5 w-1.5 shrink-0 rounded-full bg-teal-300" />
          <strong className="whitespace-nowrap text-teal-100">{live ? 'Live connection' : 'Synthetic showcase'}</strong>
          <span className="text-teal-100/70">
            {live
              ? 'Only native data is shown. Execution requires your explicit action and a host-granted workspace.'
              : 'Example accounts and models. No credentials inspected. No inference used.'}
          </span>
          {live ? (
            <Button variant="ghost" size="sm" onClick={() => demo.revokeConnection()} className="ml-auto h-7 text-xs">
              Revoke connection
            </Button>
          ) : (
            <Badge className="ml-auto hidden text-[0.5rem] tracking-widest sm:inline-flex">NO ACCOUNT REQUIRED</Badge>
          )}
        </div>
        <div className="main-content mx-auto w-full max-w-[103rem] px-4 py-6 md:px-8 md:py-7">
          <div className="page-heading mb-7 flex items-center justify-between gap-4">
            <div className="min-w-0">
              <span className="eyebrow mb-3 block text-[0.55rem] font-bold tracking-[0.15rem] text-muted-foreground">
                {live ? 'CONNECTED WORKSPACE' : 'SDK & SELECTION PREVIEW'}
              </span>
              <h1 id="page-title" className="mb-2 text-3xl font-semibold tracking-tight">
                {demo.page}
              </h1>
              <p className="mb-0 text-sm text-muted-foreground">{descriptions[demo.page]}</p>
            </div>
            <Badge className="hidden shrink-0 sm:inline-flex">
              {demo.snapshot ? `${(demo.snapshot as any).models.length.toLocaleString()}${live ? '' : ' example '}models` : 'Not connected'}
            </Badge>
          </div>
          {demo.connectionError ? (
            <div id="connection-error" role="alert" className="notice mb-4 rounded-lg border border-amber-300/25 bg-amber-400/10 px-3.5 py-3 text-xs text-amber-100">
              {demo.connectionError}
            </div>
          ) : null}
          {demo.notice ? (
            <div role="alert" className="notice mb-4 rounded-lg border border-amber-300/25 bg-amber-400/10 px-3.5 py-3 text-xs text-amber-100">
              {demo.notice}
            </div>
          ) : null}
          {children}
          <footer className="page-footer mt-7 flex flex-col gap-1 border-t pt-4 text-[10px] text-muted-foreground md:flex-row md:justify-between">
            <span>Local-first by design. Honest about what is known.</span>
            <span>v0.1 · work in progress</span>
          </footer>
        </div>
      </main>
    </div>
  );
}

export function ConnectButton({ demo }: { demo: Demo }): React.ReactElement {
  const [open, setOpen] = useState(false);
  return (
    <>
      <Button variant="outline" size="sm" onClick={() => setOpen(true)} className="connect-button text-xs">
        Connect local runtime
      </Button>
      {open ? <ConnectDialog demo={demo} onClose={() => setOpen(false)} /> : null}
    </>
  );
}

export function ConnectDialog({ demo, onClose }: { demo: Demo; onClose: () => void }): React.ReactElement {
  const dialogRef = useRef<HTMLDialogElement>(null);
  const [endpoint, setEndpoint] = useState('http://127.0.0.1:4499');
  const [token, setToken] = useState('');
  useEffect(() => {
    dialogRef.current?.showModal();
  }, []);
  return (
    <dialog
      ref={dialogRef}
      aria-label="Connect an authorized local runtime"
      onClose={onClose}
      className="connect-dialog w-[min(40rem,calc(100%-2rem))] rounded-2xl border bg-popover p-6 text-popover-foreground shadow-xl backdrop:bg-black/70 backdrop:backdrop-blur-sm"
    >
      <form
        onSubmit={(event) => {
          event.preventDefault();
          const sessionToken = token;
          setToken('');
          dialogRef.current?.close();
          demo.connectLive(endpoint, sessionToken);
        }}
      >
        <div className="mb-5 flex items-center justify-between gap-4">
          <h2 className="text-lg font-semibold tracking-tight">Connect local runtime</h2>
          <Button variant="outline" size="sm" type="button" onClick={() => dialogRef.current?.close()}>
            Close
          </Button>
        </div>
        <p className="mb-4 text-xs text-muted-foreground">Connect only to a runtime you started and explicitly authorized. Use an app-session token—not an AI provider key.</p>
        <div className="mb-4 flex flex-col gap-1.5 text-xs">
          <label htmlFor="runtime-endpoint">Runtime endpoint</label>
          <input id="runtime-endpoint" type="url" required value={endpoint} onChange={(event) => setEndpoint(event.target.value)} className="flex h-10 w-full rounded-md border border-input bg-background/60 px-3 py-2 text-sm outline-none focus-visible:ring-2 focus-visible:ring-ring" />
        </div>
        <div className="mb-4 flex flex-col gap-1.5 text-xs">
          <label htmlFor="app-token">App session token</label>
          <input id="app-token" type="password" autoComplete="off" required value={token} onChange={(event) => setToken(event.target.value)} className="flex h-10 w-full rounded-md border border-input bg-background/60 px-3 py-2 text-sm outline-none focus-visible:ring-2 focus-visible:ring-ring" />
        </div>
        <p className="fine-print mb-4 text-[10px] leading-relaxed text-muted-foreground">Live execution is disabled in this preview. Failed live connection attempts never fall back to example accounts.</p>
        <Button type="submit" variant="default">
          Connect runtime
        </Button>
      </form>
    </dialog>
  );
}
