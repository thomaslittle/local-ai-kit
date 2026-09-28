/** Presentation helpers. Runtime-supplied labels are always rendered as text nodes. */

export const capNames: Record<string, string> = {
  'generation.text': 'Text generation',
  'input.text': 'Text input',
  'input.image': 'Image input',
  'structured.output': 'Structured output',
  tools: 'Tool support',
  offline: 'Offline operation',
};

export function reasonLabel(reason: string): string {
  const [code, id] = reason.split(':', 2);
  const capability = capNames[id] ?? id;
  if (code === 'CAPABILITY_UNKNOWN') return `${capability} is not confirmed`;
  if (code === 'CAPABILITY_UNSUPPORTED') return `${capability} is not supported`;
  if (code === 'CAPABILITY_CONDITIONAL') return `${capability} has unmet requirements`;
  return (
    (
      {
        AUTH_UNVERIFIED: 'Authentication has not been checked',
        AUTH_REQUIRED: 'Authentication needs attention',
        BILLING_UNKNOWN: 'Billing classification is not reported',
        BILLING_CONSENT_REQUIRED: 'Metered access needs your consent',
        RATE_LIMITED: 'The provider reports no ordinary usage allowance right now',
      } as Record<string, string>
    )[code] ?? reason
  );
}

export function compactIdentity(value: string): string {
  const text = String(value);
  return text.length > 28 ? `${text.slice(0, 25)}…` : text;
}

export function titleCase(text: string): string {
  return String(text)
    .replaceAll('_', ' ')
    .replace(/\b\w/g, (c) => c.toUpperCase());
}

type Observation = { state: string; value?: unknown } | null | undefined;

export function known(observation: Observation): any {
  if (observation && typeof observation === 'object' && observation.state === 'known') {
    return (observation as { value: unknown }).value;
  }
  return undefined;
}
