import type { LocalAIClient } from './client.ts';
import type { InventoryChange, Snapshot } from './generated.ts';
import { LocalAIError } from './errors.ts';
import { createSnapshotStore, type SnapshotStore } from './store.ts';

export interface LiveSnapshotStore extends SnapshotStore {
  /**
   * Subscribe from the loaded snapshot's cursor and coalesce changes into one
   * catalog read. Returns a cleanup function. Calling twice is idempotent.
   */
  connect(): () => void;
}

/**
 * The broker is the polling/retry owner. A consumer only receives a small
 * change notification and re-reads the ready snapshot; it never runs provider
 * discovery in the UI.
 */
export function createLiveSnapshotStore(
  client: LocalAIClient,
  initial?: Snapshot,
): LiveSnapshotStore {
  const store = createSnapshotStore(client, initial);
  let controller: AbortController | undefined;
  let started = false;
  let disposed = false;
  let pending: ReturnType<typeof setTimeout> | undefined;
  let reading = false;

  const schedule = () => {
    if (disposed) return;
    if (pending) return;
    pending = setTimeout(() => {
      pending = undefined;
      if (disposed || reading) return;
      reading = true;
      store.load().catch(() => {
        // The store already exposes the redacted LocalAIError to observers.
      }).finally(() => {
        reading = false;
        if (!disposed) schedule();
      });
    }, 40);
  };

  const run = async (signal: AbortSignal) => {
    // One initial read is explicit: the store never reads at import time.
    try {
      await store.load();
    } catch {
      return;
    }
    while (!disposed && !signal.aborted) {
      const cursor = store.getSnapshot().snapshot?.cursor;
      try {
        for await (const change of client.inventory.subscribe(
          cursor ? { after: cursor } : {},
          { signal },
        ) as AsyncIterable<InventoryChange>) {
          if (signal.aborted) return;
          if (change.changes.length > 0 || change.issuesChanged) schedule();
        }
      } catch (error) {
        if (disposed || signal.aborted) return;
        if (error instanceof LocalAIError && error.code === 'EVENT_GAP') {
          // The journal no longer replays this cursor: full resynchronization is
          // the only correct recovery. Do not guess at missing changes.
          try { await store.load(); } catch { /* surfaced through state */ }
          continue;
        }
        // A revoked or closed client must not spin. A transient error backs off.
        if (error instanceof LocalAIError && ['CLOSED', 'LOCAL_ACCESS_DENIED', 'TRANSPORT_CLOSED'].includes(error.code)) return;
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
    }
  };

  return Object.assign(store, {
    connect() {
      if (started || disposed) return () => {};
      started = true;
      controller = new AbortController();
      void run(controller.signal);
      return () => {
        controller?.abort();
        controller = undefined;
        started = false;
      };
    },
    dispose() {
      disposed = true;
      if (pending) clearTimeout(pending);
      pending = undefined;
      controller?.abort();
      controller = undefined;
      store.dispose();
    },
  });
}
