import type { Connection } from '@local-ai-kit/sdk';

export function connectionAuthLabel(connection: Pick<Connection, 'auth'> | null | undefined): string;
