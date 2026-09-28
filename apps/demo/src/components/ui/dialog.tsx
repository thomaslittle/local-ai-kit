import * as React from 'react';
import { cn } from '../../lib/cn';

export interface DialogProps extends React.DialogHTMLAttributes<HTMLDialogElement> {
  onClose?: () => void;
}

/** shadcn-styled wrapper around the native <dialog> element. */
export const Dialog = React.forwardRef<HTMLDialogElement, DialogProps>(
  ({ className, children, onClose, ...props }, ref) => (
    <dialog
      ref={ref}
      onClose={onClose}
      className={cn(
        'w-[min(40rem,calc(100%-2rem))] rounded-2xl border bg-popover p-6 text-popover-foreground shadow-xl backdrop:bg-black/70 backdrop:backdrop-blur-sm open:animate-in',
        className,
      )}
      {...props}
    >
      {children}
    </dialog>
  ),
);
Dialog.displayName = 'Dialog';

export function DialogHeader({ className, ...props }: React.HTMLAttributes<HTMLDivElement>): React.ReactElement {
  return <div className={cn('mb-5 flex items-center justify-between gap-4', className)} {...props} />;
}

export function DialogTitle({ className, ...props }: React.HTMLAttributes<HTMLHeadingElement>): React.ReactElement {
  return <h2 className={cn('text-lg font-semibold tracking-tight', className)} {...props} />;
}

export function DialogFooter({ className, ...props }: React.HTMLAttributes<HTMLDivElement>): React.ReactElement {
  return (
    <div
      className={cn('mt-4 flex items-center gap-2 border-t pt-3.5 text-[10px] text-muted-foreground', className)}
      {...props}
    />
  );
}
