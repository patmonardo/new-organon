// Unified message contract: Commands (utterance) and Events (resonance).
// Prakāśa → schema (pure Form/Shape). Kriyā → orchestration (Projector). Sthiti → World/Services.

export type Meta = Record<string, unknown>;

import type {
  DialecticalInfo,
  FactStoreInfo,
  EventMeta,
} from '../form/invariants';

// Unscoped (stringly-typed)
export type Command<P = unknown> = { kind: string; payload: P; meta?: Meta };
export type Event<P = unknown> = { kind: string; payload?: P; meta?: Meta };

// Optional semantic aliases
export type Utterance<P = unknown> = Command<P>;
export type Resonance<P = unknown> = Event<P>;

// Bus interface (simple pub/sub)
export interface EventBus {
  publish(event: Event): void;
  subscribe(kind: string, handler: (e: Event) => void): () => void;
}

// Scoped (preferred)
export const EngineScopes = [
  'shape',
  'entity',
  'context',
  'property',
  'relation',
  'morph',
] as const;
export type EngineScope = (typeof EngineScopes)[number];

export const EngineVerbs = [
  'create',
  'delete',
  'describe',
  'setCore',
  'setState',
  'patchState',
  'setFacets',
  'mergeFacets',
  'setSignature',
  'mergeSignature',
] as const;
export type EngineVerb = (typeof EngineVerbs)[number];

export type Kind<S extends EngineScope, V extends string> = `${S}.${V}`;

export type ScopedCommand<
  S extends EngineScope,
  V extends string,
  P = unknown,
> = {
  kind: Kind<S, V>;
  payload: P;
  meta?: Meta;
};

export type ScopedEvent<
  S extends EngineScope,
  V extends string,
  P = unknown,
> = {
  kind: Kind<S, V>;
  payload?: P;
  meta?: Meta;
};

// Helpers
export const cmd = <S extends EngineScope, V extends string>(s: S, v: V) =>
  `${s}.${v}` as Kind<S, V>;
export const evt = <S extends EngineScope>(s: S, v: string) =>
  `${s}.${v}` as Kind<S, string>;

// Type guards
export function isCommand(x: unknown): x is Command {
  return !!x && typeof (x as any).kind === 'string' && 'payload' in (x as any);
}
export function isEvent(x: unknown): x is Event {
  return !!x && typeof (x as any).kind === 'string';
}
