/** Safe SDK errors never include raw response bodies, tokens, or local paths. */
export class LocalAIError extends Error {
  readonly code: string;
  readonly retryable: boolean;
  constructor(code: string, retryable = false) {
    const safe = /^[A-Z][A-Z0-9_]{0,63}$/.test(code) ? code : 'PROTOCOL_INVALID';
    super(`Local AI request failed (${safe})`);
    this.name = 'LocalAIError'; this.code = safe; this.retryable = retryable;
  }
}
export function requireMethod(method: string): void {
  if (!/^[a-zA-Z][a-zA-Z0-9]*\.[a-zA-Z][a-zA-Z0-9]*$/.test(method) || method.length > 128) throw new LocalAIError('INVALID_METHOD');
}
export function limit(value: number, min = 1024, max = 16 * 1024 * 1024): number {
  if (!Number.isSafeInteger(value) || value < min || value > max) throw new LocalAIError('INVALID_LIMIT');
  return value;
}
