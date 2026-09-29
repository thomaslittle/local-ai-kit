import { useState } from 'react';
import { CheckIcon, CopyIcon } from '@phosphor-icons/react';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';

/** Monospace block with a copy action. The clipboard is only touched on click. */
export function CodeBlock({ children, className, label, id }: { children: string; className?: string; label?: string; id?: string }) {
  const [copied, setCopied] = useState(false);
  async function copy() {
    try {
      await navigator.clipboard.writeText(children);
      setCopied(true);
      setTimeout(() => setCopied(false), 1600);
    } catch {
      setCopied(false);
    }
  }
  return (
    <div className="group/code relative">
      {label ? <span className="mb-2 block text-[0.65rem] font-medium tracking-widest text-muted-foreground uppercase">{label}</span> : null}
      <pre id={id} className={cn('code', className)}>
        {children}
      </pre>
      <Button
        variant="outline"
        size="icon-xs"
        aria-label={copied ? 'Copied' : 'Copy to clipboard'}
        onClick={copy}
        className={cn('absolute right-2 opacity-0 transition-opacity group-hover/code:opacity-100 focus-visible:opacity-100', label ? 'top-8' : 'top-2')}
      >
        {copied ? <CheckIcon /> : <CopyIcon />}
      </Button>
    </div>
  );
}
