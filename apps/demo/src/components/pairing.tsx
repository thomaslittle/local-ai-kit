import { useState } from 'react';
import type * as React from 'react';
import { createHttpTransport, createLocalAI, getPairingCode, issuePairingCode, type LocalAIClient } from '@local-ai-kit/sdk';
import { Button } from './ui/button';
import { Input } from './ui/field';
import { Card } from './ui/card';

async function connect(endpoint: string, token: string): Promise<LocalAIClient> {
  return createLocalAI({ clientName: 'local-ai-kit-showcase', transport: createHttpTransport({ baseUrl: endpoint, token }) });
}

export function Pairing({ onPaired }: { onPaired: (client: LocalAIClient) => void }): React.ReactElement {
  const [code, setCode] = useState('');
  const [status, setStatus] = useState('');
  const [failed, setFailed] = useState(false);
  const [working, setWorking] = useState(false);

  async function showCode(): Promise<void> {
    setWorking(true);
    setFailed(false);
    setStatus('Getting the current code…');
    try {
      let pairing;
      let issued = false;
      try {
        pairing = await getPairingCode({ baseUrl: window.location.origin });
      } catch (error) {
        if ((error as any)?.code !== 'HTTP_403') throw error;
        pairing = await issuePairingCode({ baseUrl: window.location.origin });
        issued = true;
      }
      setCode(pairing.code);
      setStatus(`${issued ? 'New code issued. ' : ''}Code expires in about ${pairing.expiresInSeconds} seconds.`);
    } catch {
      setFailed(true);
      setStatus('Could not get a pairing code from this local runtime. Check that the helper is running and reload the page.');
    } finally {
      setWorking(false);
    }
  }

  async function submit(): Promise<void> {
    setWorking(true);
    setFailed(false);
    setStatus('Connecting…');
    try {
      const response = await fetch('/v1/pair', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ code: code.trim() }),
        credentials: 'omit',
        cache: 'no-store',
        redirect: 'error',
      });
      if (!response.ok) throw new Error('PAIRING_DENIED');
      const data = await response.json();
      if (typeof data.token !== 'string') throw new Error('INVALID_PAIRING');
      const client = await connect(window.location.origin, data.token);
      setCode('');
      onPaired(client);
    } catch {
      setFailed(true);
      setStatus('Pairing failed. Get a fresh code and try again.');
      setWorking(false);
    }
  }

  return (
    <main className="pairing-screen">
      <Card className="pairing-card grid w-full max-w-[32rem] gap-4 p-6">
        <span className="text-[0.55rem] font-bold tracking-[0.15rem] text-muted-foreground">LOCAL AI KIT</span>
        <h1 className="text-3xl font-semibold tracking-tight">Connect your local runtime</h1>
        <p className="text-sm text-muted-foreground">
          Connect to the local helper to inspect available providers and models. Pairing checks metadata only; model turns still require your action.
        </p>
        <label htmlFor="pairing-code" className="text-xs font-medium">
          Pairing code
        </label>
        <Input
          id="pairing-code"
          type="text"
          maxLength={12}
          autoComplete="off"
          spellCheck={false}
          placeholder="12-character code"
          value={code}
          onChange={(event) => setCode(event.target.value)}
          onKeyDown={(event) => {
            if (event.key === 'Enter' && !working) {
              event.preventDefault();
              void submit();
            }
          }}
          className="tracking-[0.15em]"
        />
        <div className="pairing-actions flex flex-wrap gap-2.5">
          <Button variant="outline" onClick={() => void showCode()} disabled={working}>
            Show current pairing code
          </Button>
          <Button variant="default" onClick={() => void submit()} disabled={working}>
            Pair with this runtime
          </Button>
        </div>
        <p aria-live="polite" role={failed ? 'alert' : undefined} className="min-h-[1.5em] text-xs text-muted-foreground">
          {status}
        </p>
        <p className="fine-print text-[10px] leading-relaxed text-muted-foreground">
          Access is limited to the permissions granted when starting this runtime. Your provider credentials stay with the native client.
        </p>
      </Card>
    </main>
  );
}
