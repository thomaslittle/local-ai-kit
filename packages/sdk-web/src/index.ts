/** Browser bridge detection. Importing performs no request and starts nothing. */
export type Bridge = 'extension' | 'pairing' | 'none';

export interface DetectOptions {
  /** How long to wait for the extension content bridge. */
  extensionTimeoutMs?: number;
  /** Loopback origin used by the pairing fallback. */
  pairingOrigin?: string;
  /** How long to wait for the pairing endpoint. */
  pairingTimeoutMs?: number;
}

const PAGE_SOURCE = 'originlm-web';
const EXTENSION_SOURCE = 'originlm-extension';
/** The service install binds this loopback port for the pairing fallback. */
export const DEFAULT_PAIRING_ORIGIN = 'http://127.0.0.1:47821';

/**
 * The extension answers a page ping only when its content bridge is loaded.
 * No answer means "not available here", never an error.
 */
export function detectExtension(timeoutMs = 300): Promise<boolean> {
  if (typeof window === 'undefined') return Promise.resolve(false);
  const id = `ping-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}`;
  return new Promise((resolve) => {
    let settled = false;
    const finish = (value: boolean) => {
      if (settled) return;
      settled = true;
      window.removeEventListener('message', onMessage);
      clearTimeout(timer);
      resolve(value);
    };
    function onMessage(event: MessageEvent) {
      const data = event.data as { source?: unknown; type?: unknown; id?: unknown } | null;
      if (!data || data.source !== EXTENSION_SOURCE || data.type !== 'pong' || data.id !== id) return;
      finish(true);
    }
    const timer = setTimeout(() => finish(false), timeoutMs);
    window.addEventListener('message', onMessage);
    window.postMessage({source: PAGE_SOURCE, type: 'ping', id}, window.location.origin);
  });
}

/**
 * Reachability only: an opaque `no-cors` response cannot be read, which is the
 * point. The pairing code and the scoped token are a separate, explicit step.
 */
export async function detectPairing(
  origin: string = DEFAULT_PAIRING_ORIGIN,
  timeoutMs = 400,
): Promise<boolean> {
  if (typeof fetch === 'undefined') return false;
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), timeoutMs);
  try {
    await fetch(new URL('/runtime/info', origin).toString(), {
      mode: 'no-cors',
      cache: 'no-store',
      signal: controller.signal,
    });
    return true;
  } catch {
    return false;
  } finally {
    clearTimeout(timer);
  }
}

/**
 * Extension first, then pairing. `none` means the page should show the
 * "Connect your local AI" flow with an install link.
 */
export async function detect(options: DetectOptions = {}): Promise<Bridge> {
  if (await detectExtension(options.extensionTimeoutMs ?? 300)) return 'extension';
  if (await detectPairing(options.pairingOrigin, options.pairingTimeoutMs ?? 400)) return 'pairing';
  return 'none';
}

/** Badge copy for the three states, with at most three provider names. */
export function badgeLabel(bridge: Bridge, providers: string[] = []): string {
  if (bridge === 'none') return 'Not detected';
  const shown = providers.slice(0, 3).join(', ');
  const extra = providers.length > 3 ? ` +${providers.length - 3}` : '';
  return shown ? `Local AI connected · ${shown}${extra}` : 'Local AI connected';
}
