import { Badge } from '@/components/ui/badge';
import { usePageMeta } from '@/lib/pages';
import { useDemo } from '@/state/context';

export function PageHeading() {
  const { page, description } = usePageMeta();
  const live = useDemo((state) => state.mode !== 'synthetic');
  const modelCount = useDemo((state) => state.snapshot?.models.length);
  return (
    <div className="mb-8 flex items-end justify-between gap-4">
      <div className="min-w-0">
        <span className="mb-3 block text-[0.65rem] font-semibold tracking-[0.2em] text-brand-foreground uppercase">
          {live ? 'Connected workspace' : 'SDK & selection preview'}
        </span>
        <h1 id="page-title" className="text-gradient text-3xl font-semibold tracking-tight sm:text-4xl">
          {page}
        </h1>
        <p className="mt-2 max-w-2xl text-sm text-muted-foreground sm:text-base">{description}</p>
      </div>
      <Badge variant="outline" className="hidden h-7 shrink-0 px-3 sm:inline-flex">
        {modelCount === undefined ? 'Not connected' : `${modelCount.toLocaleString()}${live ? ' ' : ' example '}models`}
      </Badge>
    </div>
  );
}
