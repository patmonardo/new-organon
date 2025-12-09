/**
 * React Shape Adapter for MVC SDSL
 *
 * Direct rendering: Shape → React Components
 *
 * This adapter renders Shape objects directly to React components,
 * bypassing the DisplayDocument intermediate representation.
 * It uses the ui/react components that consume Shape schemas.
 */

import React from 'react';
import type { FormShape } from '../../../sdsl/types';
import { ButtonShape } from '../../../schema/button';
import { ListShape } from '../../../schema/list';
import { CardShape, StatCardShape, ContainerCardShape } from '../../../schema/card';
import { LinkShape } from '../../../schema/link';

// Import Shape renderers
import {
  ButtonRenderer,
  LinkRenderer,
  ListRenderer,
  BreadcrumbsRenderer,
  PaginationRenderer,
  CardRenderer,
  StatCardRenderer,
  ContainerCardRenderer,
  SearchRenderer,
} from '../';

// ============================================================
// SHAPE RENDERER REGISTRY
// ============================================================

/**
 * ShapeRenderer: Function that renders a Shape object
 */
type ShapeRenderer<T = any> = (shape: T, key?: string | number) => React.ReactNode;

/**
 * Shape renderer registry maps shape types to renderers
 */
const shapeRegistry: Map<string, ShapeRenderer> = new Map();

/**
 * Register a shape renderer
 */
export function registerShapeRenderer<T = any>(
  type: string,
  renderer: ShapeRenderer<T>
): void {
  shapeRegistry.set(type, renderer);
}

/**
 * Get a shape renderer
 */
export function getShapeRenderer(type: string): ShapeRenderer | undefined {
  return shapeRegistry.get(type);
}

// ============================================================
// REGISTER BUILT-IN SHAPE RENDERERS
// ============================================================

// Button shapes
registerShapeRenderer('button', (shape: ButtonShape, key) => (
  <ButtonRenderer key={key} shape={shape} />
));

// Link shapes
registerShapeRenderer('link', (shape: LinkShape, key) => (
  <LinkRenderer key={key} link={shape} />
));

// List shapes
registerShapeRenderer('list', (shape: ListShape, key) => (
  <ListRenderer key={key} list={shape} />
));

// Card shapes
registerShapeRenderer('card', (shape: CardShape, key) => (
  <CardRenderer key={key} shape={shape} />
));

registerShapeRenderer('stat-card', (shape: StatCardShape, key) => (
  <StatCardRenderer key={key} shape={shape} />
));

registerShapeRenderer('container-card', (shape: ContainerCardShape, key) => (
  <ContainerCardRenderer key={key} shape={shape} />
));

// ============================================================
// CORE RENDERING FUNCTIONS
// ============================================================

/**
 * Render a single Shape object to React
 */
export function renderShape(
  shape: any,
  key?: string | number
): React.ReactNode {
  // All shapes should have a 'type' property
  const shapeType = shape.type;
  
  if (!shapeType) {
    console.warn('Shape missing type property:', shape);
    return null;
  }

  // Get registered renderer
  const renderer = shapeRegistry.get(shapeType);
  
  if (renderer) {
    return renderer(shape, key);
  }

  // Fallback: render as a simple div with shape info
  console.warn(`No renderer registered for shape type: ${shapeType}`);
  return (
    <div key={key} className="p-4 border border-yellow-300 bg-yellow-50">
      <p className="text-sm text-gray-600">
        Unrendered shape type: <code>{shapeType}</code>
      </p>
    </div>
  );
}

/**
 * Render an array of Shapes
 */
export function renderShapes(
  shapes: any[],
  containerClass?: string
): React.ReactNode {
  if (!shapes || shapes.length === 0) {
    return null;
  }

  return (
    <div className={containerClass}>
      {shapes.map((shape, index) => renderShape(shape, index))}
    </div>
  );
}

// ============================================================
// REACT SHAPE ADAPTER CLASS
// ============================================================

/**
 * ReactShapeAdapter: Transforms Shape objects directly to React components
 *
 * Usage:
 *   const adapter = new ReactShapeAdapter();
 *   const jsx = adapter.render(shapes);
 */
export class ReactShapeAdapter {
  /**
   * Render a single Shape to React
   */
  render(shape: any): React.ReactNode {
    return renderShape(shape);
  }

  /**
   * Render multiple Shapes to React
   */
  renderMany(shapes: any[], containerClass?: string): React.ReactNode {
    return renderShapes(shapes, containerClass);
  }

  /**
   * Register a custom shape renderer
   */
  register<T = any>(type: string, renderer: ShapeRenderer<T>): void {
    registerShapeRenderer(type, renderer);
  }
}

/**
 * Default ReactShapeAdapter instance
 */
export const reactShapeAdapter = new ReactShapeAdapter();

// ============================================================
// REACT COMPONENT WRAPPERS
// ============================================================

/**
 * ShapeView: React component that renders a Shape
 */
export interface ShapeViewProps {
  shape: any;
  className?: string;
}

export function ShapeView({ shape, className }: ShapeViewProps): React.ReactElement {
  return <div className={className}>{renderShape(shape)}</div>;
}

/**
 * ShapesView: React component that renders multiple Shapes
 */
export interface ShapesViewProps {
  shapes: any[];
  containerClass?: string;
}

export function ShapesView({ shapes, containerClass }: ShapesViewProps): React.ReactElement {
  return <>{renderShapes(shapes, containerClass)}</>;
}

