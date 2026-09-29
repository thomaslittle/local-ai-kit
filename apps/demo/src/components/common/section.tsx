import type { ReactNode } from 'react';
import { Card } from '@/components/ui/card';
import { cn } from '@/lib/utils';

interface SectionProps {
  title: string;
  description?: string;
  action?: ReactNode;
  className?: string;
  children?: ReactNode;
}

/** A titled card. The title is a real heading so pages stay navigable by outline. */
export function Section({ title, description, action, className, children }: SectionProps) {
  return (
    <Card className={cn('bg-card/70 p-6 backdrop-blur-sm [--card-spacing:--spacing(0)]', className)}>
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div className="min-w-0">
          <h2 className="text-lg font-semibold tracking-tight">{title}</h2>
          {description ? <p className="mt-1 text-sm text-muted-foreground">{description}</p> : null}
        </div>
        {action}
      </div>
      {children ? <div className="mt-5">{children}</div> : null}
    </Card>
  );
}
