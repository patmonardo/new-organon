export type TriadCommand = {
  kind: string;
  payload: unknown;
  meta?: Record<string, unknown>;
};

export type TriadEvent = {
  kind: string;
  payload: unknown;
  meta?: {
    traceId?: string;
    spanId?: string;
    parentSpanId?: string;
    engine?: string;
    timestamp?: string;
    [k: string]: unknown;
  };
};

export type EventBusLike = { publish?: (evt: TriadEvent) => void };

export default TriadCommand;
