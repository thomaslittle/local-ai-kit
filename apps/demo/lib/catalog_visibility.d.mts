import type { Snapshot } from '@local-ai-kit/sdk';

export function isLocalRuntimeProvider(connection: { providerId?: string } | null | undefined): boolean;
export function filterLiveCatalog(snapshot: Snapshot): Snapshot;
