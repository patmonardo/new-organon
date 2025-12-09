/**
 * ReactShapeView - Direct Shape Rendering for React
 *
 * This view bypasses DisplayDocument and produces Shape objects directly,
 * which are then rendered by react-shape-adapter.
 *
 * Architecture:
 *   FormModel → ReactShapeView → Shapes → ReactShapeAdapter → React Components
 *
 * This is simpler than the DisplayDocument intermediate layer:
 *   FormModel → FormView → DisplayDocument → ReactAdapter → React Components
 */

import React from 'react';
import type { FormShape, FormMode, FormHandler } from '../../../sdsl/types';
import type { FormModel } from '../../../sdsl/form-model';
import { renderShape, renderShapes, ReactShapeAdapter, reactShapeAdapter } from './react-shape-adapter';

// ============================================================
// REACT SHAPE VIEW TYPES
// ============================================================

/**
 * ReactShapeViewOptions: Configuration for ReactShapeView
 */
export interface ReactShapeViewOptions {
  /** Custom adapter instance (defaults to shared reactShapeAdapter) */
  adapter?: ReactShapeAdapter;
  /** Additional CSS classes for the root element */
  className?: string;
  /** Whether to include form wrapper */
  asForm?: boolean;
}

/**
 * ReactShapeRenderResult: What ReactShapeView.render() returns
 */
export interface ReactShapeRenderResult {
  /** The React element tree */
  element: React.ReactNode;
  /** The underlying Shape(s) (for inspection/debugging) */
  shapes: any[];
}

// ============================================================
// REACT SHAPE VIEW CLASS
// ============================================================

/**
 * ReactShapeView: React-specific View that produces Shapes directly
 *
 * This view transforms FormModel data into Shape objects,
 * which are then rendered directly to React components.
 */
export class ReactShapeView<T extends FormShape = FormShape> {
  protected _model: FormModel<T>;
  protected _mode: FormMode;
  protected _adapter: ReactShapeAdapter;
  protected _options: ReactShapeViewOptions;

  constructor(
    model: FormModel<T>,
    mode: FormMode = 'view',
    options: ReactShapeViewOptions = {}
  ) {
    this._model = model;
    this._mode = mode;
    this._adapter = options.adapter || reactShapeAdapter;
    this._options = options;
  }

  // ============================================================
  // MODE MANAGEMENT
  // ============================================================

  get mode(): FormMode {
    return this._mode;
  }

  setMode(mode: FormMode): void {
    this._mode = mode;
  }

  // ============================================================
  // SHAPE PRODUCTION
  // ============================================================

  /**
   * Produce Shape objects from the model
   * Override this in subclasses for custom Shape production
   */
  protected produceShapes(): any[] {
    // Default: return the model's shape as-is
    // Subclasses can transform this into specific Shape types
    // (ButtonShape, ListShape, CardShape, etc.)
    return [this._model.shape];
  }

  // ============================================================
  // REACT RENDERING
  // ============================================================

  /**
   * Render to React components
   */
  renderReact(handler?: FormHandler): ReactShapeRenderResult {
    // Get Shape objects
    const shapes = this.produceShapes();

    // Render to React via adapter
    const element = this._adapter.renderMany(shapes, this._options.className);

    return { element, shapes };
  }

  /**
   * Render as a form element with action
   */
  renderForm(action: (formData: FormData) => Promise<void>, handler?: FormHandler): React.ReactNode {
    const { element } = this.renderReact(handler);

    return (
      <form action={action} className={this._options.className}>
        {element}
      </form>
    );
  }

  /**
   * Render a single Shape by index
   */
  renderShapeByIndex(index: number, handler?: FormHandler): React.ReactNode {
    const shapes = this.produceShapes();
    const shape = shapes[index];
    
    if (!shape) return null;

    return renderShape(shape, index);
  }

  // ============================================================
  // STATIC HELPERS
  // ============================================================

  /**
   * Create a ReactShapeView from a FormShape directly
   */
  static fromShape<T extends FormShape>(
    shape: T,
    mode: FormMode = 'view',
    options?: ReactShapeViewOptions
  ): ReactShapeView<T> {
    // Import SimpleFormModel dynamically to avoid circular deps
    const { SimpleFormModel } = require('./form-model');
    const model = new SimpleFormModel(shape);
    return new ReactShapeView(model, mode, options);
  }

  /**
   * Quick render a FormShape to React
   */
  static render<T extends FormShape>(
    shape: T,
    mode: FormMode = 'view',
    handler?: FormHandler
  ): React.ReactNode {
    const view = ReactShapeView.fromShape(shape, mode);
    return view.renderReact(handler).element;
  }

  /**
   * Render any Shape object directly (without FormModel)
   */
  static renderShape(shape: any): React.ReactNode {
    return renderShape(shape);
  }

  /**
   * Render multiple Shape objects directly (without FormModel)
   */
  static renderShapes(shapes: any[], containerClass?: string): React.ReactNode {
    return renderShapes(shapes, containerClass);
  }
}

// ============================================================
// REACT COMPONENT WRAPPERS
// ============================================================

/**
 * ShapeRenderer: Functional component for rendering any Shape
 */
export interface ShapeRendererProps {
  shape: any;
  className?: string;
}

export function ShapeRenderer({ shape, className }: ShapeRendererProps): React.ReactElement {
  return <div className={className}>{renderShape(shape)}</div>;
}

/**
 * FormShapeRenderer: Functional component for rendering FormShape
 */
export interface FormShapeRendererProps<T extends FormShape = FormShape> {
  shape: T;
  mode?: FormMode;
  handler?: FormHandler;
  className?: string;
}

export function FormShapeRenderer<T extends FormShape>({
  shape,
  mode = 'view',
  handler,
  className,
}: FormShapeRendererProps<T>): React.ReactElement {
  const view = ReactShapeView.fromShape(shape, mode, { className });
  const { element } = view.renderReact(handler);
  return <>{element}</>;
}

/**
 * FormWithAction: Form component with Server Action
 */
export interface FormWithActionProps<T extends FormShape = FormShape> {
  shape: T;
  mode?: FormMode;
  action: (formData: FormData) => Promise<void>;
  handler?: FormHandler;
  className?: string;
}

export function FormWithAction<T extends FormShape>({
  shape,
  mode = 'edit',
  action,
  handler,
  className,
}: FormWithActionProps<T>): React.ReactElement {
  const view = ReactShapeView.fromShape(shape, mode, { className });
  return <>{view.renderForm(action, handler)}</>;
}

