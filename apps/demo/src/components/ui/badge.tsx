import * as React from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '../../lib/cn';

const badgeVariants = cva(
  'inline-flex items-center gap-1 rounded-md border px-1.5 py-0.5 text-[10px] leading-5 font-medium whitespace-nowrap',
  {
    variants: {
      variant: {
        default: 'border-transparent bg-primary text-primary-foreground',
        secondary: 'border-transparent bg-secondary text-secondary-foreground',
        muted: 'border-border bg-muted/40 text-muted-foreground',
        outline: 'text-foreground',
        accent: 'border-teal-300/25 bg-teal-400/10 text-teal-200',
        supported: 'border-teal-300/25 bg-teal-400/10 text-teal-200',
        warning: 'border-amber-300/25 bg-amber-400/10 text-amber-200',
        destructive: 'border-destructive/30 bg-destructive/10 text-destructive',
      },
    },
    defaultVariants: { variant: 'muted' },
  },
);

export interface BadgeProps
  extends React.HTMLAttributes<HTMLSpanElement>,
    VariantProps<typeof badgeVariants> {}

export function Badge({ className, variant, ...props }: BadgeProps): React.ReactElement {
  return <span className={cn(badgeVariants({ variant }), className)} {...props} />;
}

/** Maps the legacy showcase badge kinds onto shadcn variants. */
export function badgeVariant(kind?: string): BadgeProps['variant'] {
  if (kind === 'accent' || kind === 'supported') return 'supported';
  if (kind === 'warning') return 'warning';
  if (kind === 'danger' || kind === 'destructive') return 'destructive';
  return 'muted';
}
