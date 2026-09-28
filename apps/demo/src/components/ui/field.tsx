import * as React from 'react';
import { cn } from '../../lib/cn';

export function Label({ className, ...props }: React.LabelHTMLAttributes<HTMLLabelElement>): React.ReactElement {
  return <label className={cn('text-xs font-medium text-foreground', className)} {...props} />;
}

export function Field({ className, ...props }: React.HTMLAttributes<HTMLDivElement>): React.ReactElement {
  return <div className={cn('field flex flex-col gap-1.5 text-xs', className)} {...props} />;
}

export function FieldHint({ className, ...props }: React.HTMLAttributes<HTMLElement>): React.ReactElement {
  return <small className={cn('text-[10px] text-muted-foreground', className)} {...props} />;
}

const inputClass =
  'flex h-10 w-full min-w-0 rounded-md border border-input bg-background/60 px-3 py-2 text-sm placeholder:text-muted-foreground outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50';

export const Input = React.forwardRef<HTMLInputElement, React.InputHTMLAttributes<HTMLInputElement>>(
  ({ className, ...props }, ref) => (
    <input ref={ref} className={cn(inputClass, className)} {...props} />
  ),
);
Input.displayName = 'Input';

export const Textarea = React.forwardRef<HTMLTextAreaElement, React.TextareaHTMLAttributes<HTMLTextAreaElement>>(
  ({ className, ...props }, ref) => (
    <textarea
      ref={ref}
      className={cn(inputClass, 'min-h-20 h-auto resize-y py-2.5', className)}
      {...props}
    />
  ),
);
Textarea.displayName = 'Textarea';

export const Select = React.forwardRef<HTMLSelectElement, React.SelectHTMLAttributes<HTMLSelectElement>>(
  ({ className, children, ...props }, ref) => (
    <select
      ref={ref}
      className={cn(inputClass, 'h-10 cursor-pointer pr-8 text-xs', className)}
      {...props}
    >
      {children}
    </select>
  ),
);
Select.displayName = 'Select';

export function Checkbox({
  className,
  ...props
}: React.InputHTMLAttributes<HTMLInputElement>): React.ReactElement {
  return (
    <input
      type="checkbox"
      className={cn('h-4 w-4 shrink-0 rounded-sm border-input accent-teal-300', className)}
      {...props}
    />
  );
}

export function CheckRow({
  label,
  ...props
}: React.InputHTMLAttributes<HTMLInputElement> & { label: React.ReactNode }): React.ReactElement {
  return (
    <label className="check flex cursor-pointer items-center gap-2 text-[11px] text-muted-foreground">
      <Checkbox {...props} />
      <span>{label}</span>
    </label>
  );
}
