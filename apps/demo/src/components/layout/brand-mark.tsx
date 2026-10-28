import markGraphite from '@/assets/brand/originlm-mark-small-graphite.svg';
import { cn } from '@/lib/utils';

/**
 * The OriginLM app icon: the graphite symbol on the amber tile, exactly as the
 * desktop build and the window icon draw it. No wordmark, so the shell reads
 * the same at every sidebar width.
 */
export function BrandMark({ size = 34, className }: { size?: number; className?: string }) {
  return (
    <span
      className={cn('grid shrink-0 place-items-center rounded-[26%] bg-primary', className)}
      style={{ width: size, height: size }}
    >
      <img src={markGraphite} alt="" aria-hidden="true" draggable={false} style={{ width: size * 0.68 }} />
    </span>
  );
}
