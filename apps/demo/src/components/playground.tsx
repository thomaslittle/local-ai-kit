import type * as React from 'react';
import { ArrowUpIcon, SquareIcon } from 'lucide-react';
import { Button } from './ui/button';
import { Badge, badgeVariant } from './ui/badge';
import { CardSection } from './ui/card';
import { AlertTitle, AlertDescription, Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyTitle } from './ui/feedback';
import { Select, Textarea, CheckRow, Label } from './ui/field';
import { cn } from '../lib/cn';
import { reasonLabel } from '../lib/format';
import { ConfigurationPanel } from './selectors';
import type { ChatMessage, Demo } from '../state/demo';

/* Chat composition adapted from shadcn-ui/chatbot-template (MIT):
   components/chat.tsx (empty state + thread + composer), prompt-form.tsx
   (model select beside the send action) and suggestions.tsx. The catalog,
   permission gates, and session engine stay Local AI Kit specific: every turn
   still needs an explicit route, consent, and Run click. */

const suggestions: { label: string; prompt: string }[] = [
  { label: 'Explain the selected route', prompt: 'Explain what this route reports: its account, billing, and native options.' },
  { label: 'Draft a plan with this model', prompt: 'Draft a short plan for trying this model route in an isolated workspace.' },
  { label: 'What can this model do?', prompt: 'Summarize the capabilities this model reports, without guessing.' },
  { label: 'Summarize the usage evidence', prompt: 'Summarize the usage windows linked to this route.' },
];

function Suggestions({ onSelect }: { onSelect: (prompt: string) => void }): React.ReactElement {
  return (
    <div className="flex flex-wrap justify-center gap-2">
      {suggestions.map((suggestion) => (
        <Button key={suggestion.label} variant="outline" size="sm" onClick={() => onSelect(suggestion.prompt)}>
          {suggestion.label}
        </Button>
      ))}
    </div>
  );
}

function ThreadMessage({ message, streaming }: { message: ChatMessage; streaming: boolean }): React.ReactElement {
  if (message.role === 'user') {
    return (
      <div className="flex justify-end">
        <div className="max-w-[80%] rounded-2xl rounded-br-md bg-secondary px-3.5 py-2.5 text-sm whitespace-pre-wrap break-words">
          {message.text}
        </div>
      </div>
    );
  }
  return (
    <div className="flex justify-start">
      <div className="max-w-[95%] min-w-0 rounded-2xl rounded-bl-md border bg-background/60 px-3.5 py-2.5 text-sm whitespace-pre-wrap break-words">
        {message.text || (streaming ? <span className="text-muted-foreground">Thinking…</span> : '')}
        {message.status && message.status !== 'Running' && message.text ? (
          <div className="mt-2">
            <Badge variant={message.status === 'Completed' ? 'supported' : 'warning'}>{message.status}</Badge>
          </div>
        ) : null}
      </div>
    </div>
  );
}

export function PlaygroundPage({ demo }: { demo: Demo }): React.ReactElement {
  const live = demo.mode !== 'synthetic';
  const methods = (demo.handshake as any)?.methods ?? [];
  const executable = methods.includes('turns.start');
  const busy = ['Running', 'Awaiting approval', 'Starting'].includes(demo.sessionStatus);
  const providers = demo.providers.filter((provider: any) => demo.all.some((candidate) => candidate.route.providerId === provider.id));
  const selectedCandidate = demo.all.find((candidate) => candidate.route.id === demo.selection?.routeId && candidate.model.id === demo.selection?.modelId);
  const providerId = demo.playgroundProviderId ?? selectedCandidate?.route.providerId ?? providers[0]?.id ?? '';
  const providerCandidates = demo.all.filter((candidate) => candidate.route.providerId === providerId);
  const isBusy = demo.sessionStatus === 'Running' || demo.sessionStatus === 'Starting';
  const lastMessage = demo.chat.at(-1);

  function submitComposer(event?: React.FormEvent): void {
    event?.preventDefault();
    if (!demo.prompt.trim() || busy) return;
    demo.startTurn();
  }

  return (
    <div className="selector-grid playground-grid grid items-start gap-5 lg:grid-cols-[minmax(0,1fr)_19rem]">
      <div className="stack grid min-w-0 gap-5">
        <CardSection className="p-6">
          <div className="section-heading mb-4">
            <h2 className="mb-1 text-lg font-semibold tracking-tight">Session playground</h2>
            <p className="text-xs text-muted-foreground">Review the live session controls. Nothing runs until you select a route and authorize a turn.</p>
          </div>
          <div className="notice mb-4 rounded-lg border border-sky-300/25 bg-sky-400/10 px-3.5 py-3 text-xs text-sky-100">
            {live
              ? 'A live turn uses your selected account allowance. The host controls allowed workspaces and permissions; this demo requests read-only execution.'
              : 'Simulation only. The fixture service returns predetermined text; it does not call an AI provider.'}
          </div>
          {live && !executable ? (
            <p className="notice mb-4 rounded-lg border border-border bg-card px-3.5 py-3 text-xs">This runtime allows metadata checks only. Enable execution and approve a workspace in the host application before sending a prompt.</p>
          ) : null}
          <div className="mb-4 flex flex-col gap-1.5 text-xs">
            <Label htmlFor="playground-provider">Provider</Label>
            <Select
              id="playground-provider"
              aria-label="Provider"
              value={providerId}
              onChange={(event) => {
                demo.patch({ playgroundProviderId: (event.target as HTMLSelectElement).value, selection: null, sensitiveConsent: false, debugError: null, notice: '' });
              }}
            >
              {providers.map((provider: any) => (
                <option key={provider.id} value={provider.id}>
                  {provider.name}
                </option>
              ))}
            </Select>
          </div>
          <div className="mb-4 flex flex-col gap-1.5 text-xs">
            <Label htmlFor="playground-model">Model</Label>
            <Select
              id="playground-model"
              aria-label="Model"
              value={selectedCandidate && selectedCandidate.route.providerId === providerId ? selectedCandidate.key : ''}
              disabled={!providerCandidates.length}
              onChange={(event) => {
                const candidate = providerCandidates.find((item) => item.key === (event.target as HTMLSelectElement).value);
                if (candidate) demo.select(candidate);
              }}
            >
              <option value="" disabled>
                {providerCandidates.length ? 'Choose a model and account' : 'No models reported for this provider'}
              </option>
              {providerCandidates.map((candidate) => (
                <option key={candidate.key} value={candidate.key}>
                  {`${candidate.model.name} · ${candidate.connection.displayName}${candidate.eligible ? '' : ` · needs attention: ${candidate.reasons.map(reasonLabel).join(', ')}`}`}
                </option>
              ))}
            </Select>
          </div>
          {selectedCandidate && !selectedCandidate.eligible ? (
            <p data-testid="model-eligibility" className="notice mb-4 rounded-lg border border-amber-300/25 bg-amber-400/10 px-3.5 py-3 text-xs text-amber-100">
              {`This model is reported but needs attention before execution: ${selectedCandidate.reasons.map(reasonLabel).join(' · ')}`}
            </p>
          ) : null}
          <form onSubmit={submitComposer}>
            <div className="rounded-xl border bg-background/60">
              <Textarea
                aria-label="Prompt"
                rows={4}
                value={demo.prompt}
                disabled={busy}
                placeholder="Send a message…"
                onChange={(event) => demo.patch({ prompt: (event.target as HTMLTextAreaElement).value })}
                onKeyDown={(event) => {
                  if (event.key === 'Enter' && !event.shiftKey && !(event.nativeEvent as KeyboardEvent).isComposing) {
                    event.preventDefault();
                    submitComposer();
                  }
                }}
                className="border-0 bg-transparent p-3.5 focus-visible:ring-0"
              />
              <div className="flex items-center gap-2 px-3 pb-3">
                <span className="min-w-0 truncate text-[11px] text-muted-foreground">
                  {selectedCandidate ? `${selectedCandidate.model.name} · ${selectedCandidate.connection.displayName}` : 'No route selected'}
                </span>
                {isBusy ? (
                  <Button
                    type="button"
                    size="icon-sm"
                    variant="outline"
                    aria-label="Stop generating"
                    onClick={() => demo.cancelTurn()}
                    disabled={!busy || !demo.turnId}
                    className="ml-auto"
                  >
                    <SquareIcon />
                  </Button>
                ) : (
                  <Button
                    type="submit"
                    size="icon-sm"
                    variant="default"
                    aria-label="Send message"
                    disabled={!demo.prompt.trim() || busy}
                    className="ml-auto"
                  >
                    <ArrowUpIcon />
                  </Button>
                )}
              </div>
            </div>
          </form>
          <div className="my-2">
            <CheckRow
              label={live ? 'Allow this live request to use the selected account' : 'Allow this simulated session'}
              checked={demo.grantConsent}
              onChange={(event) => demo.patch({ grantConsent: (event.target as HTMLInputElement).checked })}
            />
          </div>
          {live ? (
            <div className="mb-4 flex flex-col gap-1.5 text-xs">
              <Label htmlFor="host-workspace">Host workspace name</Label>
              <input
                id="host-workspace"
                aria-label="Host workspace name"
                value={demo.workspaceId}
                onChange={(event) => demo.patch({ workspaceId: (event.target as HTMLInputElement).value })}
                className="flex h-10 w-full rounded-md border border-input bg-background/60 px-3 py-2 text-sm outline-none focus-visible:ring-2 focus-visible:ring-ring"
              />
            </div>
          ) : (
            <CheckRow
              label="Simulate a tool approval"
              checked={demo.simulateApproval}
              onChange={(event) => demo.patch({ simulateApproval: (event.target as HTMLInputElement).checked })}
            />
          )}
          <div className="actions mt-4 flex flex-wrap items-center gap-2.5">
            <Button
              variant="default"
              onClick={() => demo.startTurn()}
              disabled={busy || !executable || !demo.grantConsent || !demo.selection}
            >
              {live ? 'Run live turn' : 'Simulate turn'}
            </Button>
            <Button variant="outline" onClick={() => demo.cancelTurn()} disabled={!busy || !demo.turnId}>
              Cancel turn
            </Button>
            {methods.includes('sessions.suspend') ? (
              <Button variant="outline" onClick={() => demo.suspendSession()} disabled={busy || !demo.sessionId || !demo.nativeResumption}>
                Suspend native session
              </Button>
            ) : null}
            {methods.includes('sessions.resume') ? (
              <Button variant="outline" onClick={() => demo.resumeSession()} disabled={busy || !demo.resumeToken}>
                Resume native session
              </Button>
            ) : null}
            <Badge variant={busy ? 'accent' : 'muted'}>{demo.sessionStatus}</Badge>
          </div>
        </CardSection>

        <CardSection className="p-6">
          <div className="section-heading mb-4">
            <h2 className="mb-1 text-lg font-semibold tracking-tight">Stream output</h2>
          </div>
          {demo.chat.length === 0 && !demo.output ? (
            <Empty className="mb-4">
              <EmptyHeader>
                <EmptyTitle>What can I help with?</EmptyTitle>
                <EmptyDescription>Pick a route and start a gated turn. Responses stream through the session you explicitly authorize.</EmptyDescription>
              </EmptyHeader>
              <EmptyContent>
                <Suggestions onSelect={(prompt) => demo.patch({ prompt })} />
              </EmptyContent>
            </Empty>
          ) : (
            <div className="mb-4 flex min-h-0 w-full flex-1 flex-col gap-4">
              {demo.chat.map((message) => (
                <ThreadMessage key={message.id} message={message} streaming={isBusy && message.id === lastMessage?.id} />
              ))}
              {demo.sessionStatus === 'Starting' && (!lastMessage || lastMessage.role !== 'assistant') ? (
                <div className="flex items-center gap-2 px-3 text-sm text-muted-foreground">Thinking…</div>
              ) : null}
            </div>
          )}
          <div id="session-status" role="status" aria-live="polite" className="session-status mb-3 text-[11px] text-teal-200">
            {demo.sessionStatus}
          </div>
          <pre id="response-text" className="response mb-4 min-h-28 rounded-lg bg-background/60 p-5 font-[inherit] break-words whitespace-pre-wrap text-teal-100/90">
            {demo.output || 'Output appears only after you explicitly start a turn.'}
          </pre>
          {demo.approval ? (
            <div className="approval mb-4 rounded-lg border border-amber-300/25 bg-amber-400/10 p-4">
              <h3 className="mb-1 text-sm font-semibold">{demo.approval.action}</h3>
              <p className="text-xs">{demo.approval.detail}</p>
              <div className="actions mt-3 flex flex-wrap gap-2.5">
                <Button variant="default" onClick={() => demo.answerApproval(true)}>
                  Approve
                </Button>
                <Button variant="destructive" onClick={() => demo.answerApproval(false)}>
                  Deny
                </Button>
              </div>
            </div>
          ) : null}
          {demo.debugError ? (
            <>
              <div className="relative mb-4 w-full rounded-lg border border-destructive/40 bg-destructive/10 px-4 py-3 text-sm text-destructive">
                <AlertTitle>Request failed</AlertTitle>
                <AlertDescription>{demo.notice || `The session failed. Error code: ${demo.debugError.code}.`}</AlertDescription>
              </div>
              <section data-testid="turn-debug" aria-label="Provider error debug details" className="debug-box mb-4 rounded-lg border border-red-300/30 bg-red-950/30 p-4">
                <div className="debug-heading flex items-center justify-between gap-3">
                  <h3 className="text-[0.8rem] font-semibold">Provider error details</h3>
                  <Badge variant={badgeVariant('warning')}>{demo.debugError.code}</Badge>
                </div>
                <pre className="debug-detail my-3 max-h-70 overflow-auto font-mono text-xs break-words whitespace-pre-wrap">{demo.debugError.message}</pre>
                <p className="fine-print text-[10px] text-muted-foreground">Credential and session-token fields are redacted.</p>
              </section>
            </>
          ) : null}
          <div className="actions flex flex-wrap items-center gap-2.5">
            {demo.events
              .filter((e: any) => e.sessionId === demo.sessionId && e.payload.type === 'artifact')
              .map((e: any) => (
                <Button
                  key={e.payload.artifactId ?? e.sequence}
                  variant="outline"
                  size="sm"
                  onClick={() => demo.downloadArtifact(e)}
                  disabled={!methods.includes('artifacts.read')}
                >
                  {'Download result (' + e.payload.size + ' bytes)'}
                </Button>
              ))}
          </div>
        </CardSection>

        <CardSection className={cn('p-6')}>
          <div className="section-heading mb-4">
            <h2 className="mb-1 text-lg font-semibold tracking-tight">Event timeline</h2>
            <p className="text-xs text-muted-foreground">{`${demo.events.length} recent events; bounded to 100.`}</p>
          </div>
          <div className="timeline">
            {demo.events
              .slice(-12)
              .reverse()
              .map((e: any) => (
                <div key={e.sequence} className="event-row flex items-center gap-4 border-b py-2.5 text-[11px] last:border-b-0">
                  <span className="mono">{e.sequence}</span>
                  <Badge>{e.payload.type}</Badge>
                  <small className="mono ml-auto">{e.sessionId}</small>
                </div>
              ))}
          </div>
        </CardSection>
      </div>
      <ConfigurationPanel demo={demo} />
    </div>
  );
}
