/**
 * Malloy UI - Speculative Bubble
 *
 * Mirrors React pattern initially:
 * - MalloyModel → MalloyController → MalloyView
 * - Same flow as React
 * - See how it evolves
 * - Take inspiration for React UI
 *
 * Structure:
 * - sdsl/ - Malloy adapters, controllers, views
 * - components/ - Malloy React components (future)
 */

// Malloy SDSL adapters/controllers/views
export * from './sdsl';
export * from './components/MalloyModelVisualizer';
export * from './components/MalloyViewBuilder';

// Malloy UI components (future)
export * from './components';
