import { BroadcastIcon, FlaskIcon } from '@phosphor-icons/react';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { useActions, useDemo } from '@/state/context';
import { cn } from '@/lib/utils';

/** Always-visible provenance strip: synthetic example data must never be mistaken for live results. */
export function ModeBanner() {
  const actions = useActions();
  const mode = useDemo((state) => state.mode);
  const live = mode === 'live';
  return (
    <div
      id="mode-banner"
      role="status"
      className={cn(
        mode,
        'flex flex-wrap items-center gap-x-3 gap-y-1 border-b px-4 py-2 text-xs md:px-6',
        live ? 'bg-linear-to-r from-success/15 via-success/5 to-transparent' : 'bg-linear-to-r from-info/15 via-info/5 to-transparent',
      )}
    >
      <span aria-hidden="true" className="relative flex size-2 shrink-0">
        <span className={cn('absolute inset-0 rounded-full motion-safe:animate-pulse-ring', live ? 'bg-success' : 'bg-info')} />
        <span className={cn('relative size-2 rounded-full', live ? 'bg-success' : 'bg-info')} />
      </span>
      {live ? <BroadcastIcon className="size-3.5 text-success" aria-hidden="true" /> : <FlaskIcon className="size-3.5 text-info" aria-hidden="true" />}
      <strong className="font-semibold whitespace-nowrap">{live ? 'Live connection' : 'Synthetic showcase'}</strong>
      <span className="text-muted-foreground">
        {live
          ? 'Only native data is shown. Execution requires your explicit action and a host-granted workspace.'
          : 'Example accounts and models. No credentials inspected. No inference used.'}
      </span>
      {live ? (
        <Button variant="ghost" size="xs" onClick={() => void actions.revokeConnection()} className="ml-auto">
          Revoke connection
        </Button>
      ) : (
        <Badge variant="info" className="ml-auto hidden sm:inline-flex">
          NO ACCOUNT REQUIRED
        </Badge>
      )}
    </div>
  );
}
