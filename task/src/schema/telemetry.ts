/**
 * Telemetry Schema (Schema-first)
 * ===============================
 *
 * This schema defines how agent runtimes can be *observed and controlled*
 * from a top-level orchestrator.
 *
 * Important: this is configuration-only.
 * No runtime bindings, no Genkit/OpenTelemetry imports, no implementations.
 */

import { z } from 'zod';

export const LogLevelSchema = z.enum(['trace', 'debug', 'info', 'warn', 'error', 'fatal']);
export type LogLevel = z.infer<typeof LogLevelSchema>;

export const TelemetryProviderSchema = z.enum(['none', 'genkit', 'opentelemetry', 'console']);
export type TelemetryProvider = z.infer<typeof TelemetryProviderSchema>;

export const TelemetrySamplingSchema = z.object({
  /** 0..1 */
  traceSampleRate: z.number().min(0).max(1).optional(),
  /** 0..1 */
  metricSampleRate: z.number().min(0).max(1).optional(),
});
export type TelemetrySampling = z.infer<typeof TelemetrySamplingSchema>;

/**
 * A generic “telemetry sink” descriptor.
 * Runtimes can map these to concrete exporters.
 */
export const TelemetrySinkSchema = z.object({
  provider: TelemetryProviderSchema.default('none'),
  /** Optional name (e.g. "prod", "dev", "local"). */
  name: z.string().min(1).optional(),
  /** Optional endpoint (e.g. OTLP collector URL). */
  endpoint: z.string().min(1).optional(),
  /** Freeform provider options (kept opaque at schema layer). */
  options: z.record(z.string(), z.unknown()).optional(),
});
export type TelemetrySink = z.infer<typeof TelemetrySinkSchema>;

/**
 * Genkit-specific knobs (kept structural).
 */
export const GenkitTelemetrySchema = z.object({
  enabled: z.boolean().default(true),
  /** Enables Genkit tracing if the runtime supports it. */
  tracing: z.boolean().optional(),
  /** Enables Genkit metrics if the runtime supports it. */
  metrics: z.boolean().optional(),
  /** Enables Genkit logging if the runtime supports it. */
  logging: z.boolean().optional(),
  /** Optional Genkit "app" / "project" / "service" labeling. */
  labels: z.record(z.string(), z.string()).optional(),
});
export type GenkitTelemetry = z.infer<typeof GenkitTelemetrySchema>;

export const AgentTelemetryConfigSchema = z.object({
  /** Primary sink used by the runtime. */
  sink: TelemetrySinkSchema.optional(),
  sampling: TelemetrySamplingSchema.optional(),
  logLevel: LogLevelSchema.optional(),

  /**
   * Optional provider-specific blocks.
   * These remain purely declarative.
   */
  genkit: GenkitTelemetrySchema.optional(),

  /** Extra tags attached to all spans/logs/metrics. */
  tags: z.record(z.string(), z.string()).optional(),
});
export type AgentTelemetryConfig = z.infer<typeof AgentTelemetryConfigSchema>;
