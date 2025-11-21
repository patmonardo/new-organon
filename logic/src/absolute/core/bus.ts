import type { Event, EventBus } from './message';
export type { Event, EventBus } from './message';

export class InMemoryEventBus implements EventBus {
  private handlers = new Map<string, Set<(e: Event) => void>>();

  publish(event: Event): void {
    this.handlers.get(event.kind)?.forEach((h) => h(event));
  }

  subscribe(kind: string, handler: (e: Event) => void): () => void {
    let set = this.handlers.get(kind);
    if (!set) {
      set = new Set();
      this.handlers.set(kind, set);
    }
    set.add(handler);
    return () => set!.delete(handler);
  }
}
