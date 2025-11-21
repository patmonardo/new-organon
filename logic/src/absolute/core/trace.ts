import { randomUUID } from 'node:crypto';

export type TraceMeta = {
  traceId: string;
  spanId: string;
  parentSpanId?: string;
  correlationId?: string;
  engine?: string;
  timestamp: string;
  [k: string]: unknown;
};

export function startTrace(
  engine: string,
  correlationId?: string,
  extra?: Record<string, unknown>,
) {
  return {
    traceId: randomUUID(),
    spanId: randomUUID(),
    correlationId,
    engine,
    timestamp: new Date().toISOString(),
    ...(extra ?? {}),
  } as TraceMeta;
}

export function childSpan(parent: TraceMeta, extra?: Record<string, unknown>) {
  return {
    traceId: parent.traceId,
    spanId: randomUUID(),
    parentSpanId: parent.spanId,
    correlationId: parent.correlationId,
    engine: parent.engine,
    timestamp: new Date().toISOString(),
    ...(extra ?? {}),
  } as TraceMeta;
}
