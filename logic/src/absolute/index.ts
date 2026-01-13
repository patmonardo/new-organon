/**
 * @absolute - Minimal Engine Bus Support
 *
 * Provides only the essential Event Bus infrastructure used by engines:
 * - EventBus interface and InMemoryEventBus implementation
 * - Command/Event message types
 * - Tracing utilities (startTrace, childSpan)
 *
 * All speculative AbsoluteConcept/FormProcessor code has been removed.
 * Focus is on supporting the Engine Bus pattern for Form:Entity engines.
 */

export * from './core';
export * from './form';
