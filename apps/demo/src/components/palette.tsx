import { useEffect, useMemo, useRef, useState } from 'react';
import type * as React from 'react';
import { resolveCandidates } from '@local-ai-kit/selector-core';
import { Button } from './ui/button';
import { Dialog, DialogFooter, DialogHeader, DialogTitle } from './ui/dialog';
import type { Demo } from '../state/demo';

export function PaletteDialog({ demo, onClose }: { demo: Demo; onClose: () => void }): React.ReactElement {
  const dialogRef = useRef<HTMLDialogElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const [query, setQuery] = useState('');

  useEffect(() => {
    dialogRef.current?.showModal();
    inputRef.current?.focus();
  }, []);

  const candidates = useMemo(
    () => (demo.catalog ? resolveCandidates(demo.catalog, { query, allowPaidFallback: demo.allowPaid }).slice(0, 40) : []),
    [demo.catalog, demo.allowPaid, query, demo.snapshot],
  );

  return (
    <Dialog ref={dialogRef} id="palette-dialog" aria-label="Find a provider model" onClose={onClose} className="palette-dialog">
      <DialogHeader>
        <DialogTitle>Find a route</DialogTitle>
        <Button variant="outline" size="sm" type="button" onClick={() => dialogRef.current?.close()}>
          Close
        </Button>
      </DialogHeader>
      <input
        ref={inputRef}
        type="search"
        aria-label="Search every model"
        placeholder="Search every provider, model, account…"
        value={query}
        onChange={(event) => setQuery((event.target as HTMLInputElement).value)}
        onKeyDown={(event) => {
          if (event.key === 'ArrowDown') {
            event.preventDefault();
            dialogRef.current?.querySelector<HTMLButtonElement>('.palette-results button')?.focus();
          }
        }}
        className="palette-search flex h-10 w-full rounded-md border border-input bg-background/60 px-3 py-2 text-sm outline-none placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring"
      />
      <div
        className="palette-results mt-3 max-h-[25rem] overflow-y-auto"
        onKeyDown={(event) => {
          if (!['ArrowDown', 'ArrowUp'].includes(event.key)) return;
          event.preventDefault();
          const buttons = [...(event.currentTarget as HTMLDivElement).querySelectorAll('button')];
          const index = buttons.indexOf(document.activeElement as HTMLButtonElement);
          const next = index + (event.key === 'ArrowDown' ? 1 : -1);
          if (next < 0) inputRef.current?.focus();
          else buttons[Math.min(buttons.length - 1, next)]?.focus();
        }}
      >
        {candidates.map((c) => (
          <Button
            key={c.key}
            variant="ghost"
            onClick={() => {
              demo.select(c);
              dialogRef.current?.close();
            }}
            className="palette-result mb-1 h-auto w-full flex-col items-start gap-1 border border-transparent p-3 text-left"
          >
            <strong className="block text-sm">{c.model.name}</strong>
            <small className="mt-1 block text-[10px] text-muted-foreground">{`${c.connection.displayName} · ${c.billingLabel}`}</small>
          </Button>
        ))}
        {!candidates.length ? <p className="text-xs text-muted-foreground">No matching routes.</p> : null}
      </div>
      <DialogFooter className="palette-footer">
        <kbd className="inline-block rounded border border-border px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">↑ ↓</kbd>
        <span>Navigate</span>
        <kbd className="inline-block rounded border border-border px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">Enter</kbd>
        <span>Inspect</span>
        <kbd className="inline-block rounded border border-border px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">Esc</kbd>
        <span>Close</span>
      </DialogFooter>
    </Dialog>
  );
}
