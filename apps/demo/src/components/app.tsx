import { useEffect, useState } from 'react';
import type * as React from 'react';
import { useDemo, type DemoFactory } from '../state/demo';
import { Shell } from './shell';
import { OverviewPage, ConnectionsPage, UsagePage, DiagnosticsPage } from './pages';
import { SelectorsPage } from './selectors';
import { PlaygroundPage } from './playground';
import { PaletteDialog } from './palette';
import { Empty } from './ui/feedback';

function Page({ demo, onOpenPalette }: { demo: ReturnType<typeof useDemo>; onOpenPalette: () => void }): React.ReactElement {
  if (!demo.snapshot) {
    return (
      <Empty>
        <h3 className="text-foreground">No connected catalog</h3>
        <p className="text-xs">{demo.connectionError ?? 'Pair this browser with the running local helper to load its discovery snapshot.'}</p>
      </Empty>
    );
  }
  switch (demo.page) {
    case 'Overview':
      return <OverviewPage demo={demo} />;
    case 'Connections':
      return <ConnectionsPage demo={demo} />;
    case 'Usage':
      return <UsagePage demo={demo} />;
    case 'Playground':
      return <PlaygroundPage demo={demo} />;
    case 'Diagnostics':
      return <DiagnosticsPage demo={demo} />;
    default:
      return <SelectorsPage demo={demo} onOpenPalette={onOpenPalette} />;
  }
}

/** React host for the developer showcase. Consumes public SDK clients only. */
export function App(factory: DemoFactory): React.ReactElement {
  const demo = useDemo(factory);
  const [paletteOpen, setPaletteOpen] = useState(false);
  const openPalette = (): void => setPaletteOpen(true);

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent): void => {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'k' && !document.querySelector('dialog[open]')) {
        event.preventDefault();
        openPalette();
      }
    };
    document.addEventListener('keydown', onKeyDown);
    return () => document.removeEventListener('keydown', onKeyDown);
  }, []);

  return (
    <Shell demo={demo} onOpenPalette={openPalette}>
      <Page demo={demo} onOpenPalette={openPalette} />
      {paletteOpen ? <PaletteDialog demo={demo} onClose={() => setPaletteOpen(false)} /> : null}
    </Shell>
  );
}
