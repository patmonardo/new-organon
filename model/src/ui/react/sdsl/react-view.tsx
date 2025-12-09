/**
 * ReactView - React/Next View Dyad Partner
 *
 * Extends SimpleFormView with React-specific rendering capabilities.
 * Uses react-adapter for the actual DisplayElement → JSX transformation.
 *
 * The ReactView is the View half of the React dyad:
 *   ReactView + ReactController = React MVC Dyad
 *
 * Both share the same FormModel.
 */

import React from 'react';
import { SimpleFormView } from '../../../sdsl/form-view';
import type { FormShape, FormMode, DisplayDocument, DisplayElement, FormHandler } from '../../../sdsl/types';
import type { FormModel } from '../../../sdsl/form-model';
import { renderDocument, renderElement, ReactAdapter, reactAdapter } from './react-adapter';

// ============================================================
// REACT VIEW TYPES
// ============================================================

/**
 * ReactViewOptions: Configuration for ReactView
 */
export interface ReactViewOptions {
  /** Custom adapter instance (defaults to shared reactAdapter) */
  adapter?: ReactAdapter;
  /** Additional CSS classes for the root element */
  className?: string;
  /** Whether to include form wrapper */
  asForm?: boolean;
}

/**
 * ReactRenderResult: What ReactView.render() returns
 */
export interface ReactRenderResult {
  /** The React element tree */
  element: React.ReactNode;
  /** The underlying DisplayDocument (for inspection/debugging) */
  document: DisplayDocument;
}

// ============================================================
// REACT VIEW CLASS
// ============================================================

/**
 * ReactView: React-specific FormView
 *
 * Extends SimpleFormView to produce React components directly.
 * Handles React-specific concerns like hooks integration and form wrappers.
 */
export class ReactView<T extends FormShape = FormShape> extends SimpleFormView<T> {
  protected _adapter: ReactAdapter;
  protected _options: ReactViewOptions;

  constructor(
    model: FormModel<T>,
    mode: FormMode = 'view',
    options: ReactViewOptions = {}
  ) {
    super(model, mode);
    this._adapter = options.adapter || reactAdapter;
    this._options = options;
  }

  // ============================================================
  // REACT-SPECIFIC RENDERING
  // ============================================================

  /**
   * Render to React components
   */
  renderReact(handler?: FormHandler): ReactRenderResult {
    // Get the DisplayDocument from base FormView
    const document = this.render();

    // Render to React via adapter
    const element = this._adapter.render(document, {
      handler,
      mode: this._mode,
    });

    return { element, document };
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
   * Render just the actions
   */
  renderActionsOnly(handler?: FormHandler): React.ReactNode {
    const actionsElement: DisplayElement = {
      type: 'actions',
      children: [
        {
          type: 'button',
          props: { id: 'submit', label: this._mode === 'create' ? 'Create' : 'Save', variant: 'primary' },
        },
        {
          type: 'button',
          props: { id: 'cancel', label: 'Cancel', variant: 'secondary' },
        },
      ],
    };
    return renderElement(actionsElement, 'actions', { handler, mode: this._mode });
  }

  // ============================================================
  // STATIC HELPERS
  // ============================================================

  /**
   * Create a ReactView from a FormShape directly
   */
  static fromShape<T extends FormShape>(
    shape: T,
    mode: FormMode = 'view',
    options?: ReactViewOptions
  ): ReactView<T> {
    // Import SimpleFormModel dynamically to avoid circular deps
    const { SimpleFormModel } = require('./form-model');
    const model = new SimpleFormModel(shape);
    return new ReactView(model, mode, options);
  }

  /**
   * Quick render a FormShape to React
   */
  static render<T extends FormShape>(
    shape: T,
    mode: FormMode = 'view',
    handler?: FormHandler
  ): React.ReactNode {
    const view = ReactView.fromShape(shape, mode);
    return view.renderReact(handler).element;
  }
}

// ============================================================
// REACT COMPONENT WRAPPERS
// ============================================================

// ============================================================
// HOOKS (for future client-side state management)
// ============================================================

/**
 * useFormView: Hook for managing FormView state (future)
 *
 * This would integrate with React state for client-side forms.
 * For now, we're focused on Server Components + Server Actions.
 */
// export function useFormView<T extends FormShape>(
//   initialShape: T,
//   initialMode: FormMode = 'view'
// ) {
//   const [shape, setShape] = React.useState(initialShape);
//   const [mode, setMode] = React.useState(initialMode);
//
//   const view = React.useMemo(
//     () => ReactView.fromShape(shape, mode),
//     [shape, mode]
//   );
//
//   return {
//     view,
//     shape,
//     mode,
//     setShape,
//     setMode,
//     render: (handler?: FormHandler) => view.renderReact(handler).element,
//   };
// }
