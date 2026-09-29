import { createHttpTransport, createLocalAI, type LocalAIClient } from '@local-ai-kit/sdk';

/** The single place the demo opens an SDK client over the public HTTP transport. */
export function connectRuntime(endpoint: string, token: string): Promise<LocalAIClient> {
  return createLocalAI({ clientName: 'local-ai-kit-showcase', transport: createHttpTransport({ baseUrl: endpoint, token }) });
}
