import { createLocalAI, type LocalAIClient } from '@local-ai-kit/sdk';
import { createFixtureService } from '../../fixture/service.mjs';
import { fixtureTransport } from '../../fixture/transport.mjs';

/**
 * Example data for when no native runtime is paired: the same in-memory fixture service the tests use,
 * behind the public SDK client. It never touches accounts, credentials, or the network.
 */
export function openExampleData(): Promise<LocalAIClient> {
  return createLocalAI({ clientName: 'local-ai-kit-showcase', transport: fixtureTransport(createFixtureService()) });
}
