/**
 * React Adapter for MVC SDSL
 *
 * Transforms DisplayDocument → React Components
 *
 * This adapter consumes the Generic Display Language produced by FormView
 * and renders it using React components with Tailwind styling.
 *
 * Future: These can be swapped to Shadcn/Radix components
 */

import React, { ChangeEvent } from 'react';
import type {
  DisplayDocument,
  DisplayElement,
  DisplayLayout,
  FormField,
  FormHandler,
  FormShape,
} from '../../../sdsl/types';

// ============================================================
// COMPONENT REGISTRY
// ============================================================

/**
 * ComponentRenderer: Function that renders a DisplayElement
 */
type ComponentRenderer = (
  element: DisplayElement,
  key: string | number,
  context: RenderContext
) => React.ReactNode;

/**
 * RenderContext: Passed through the render tree
 */
interface RenderContext {
  handler?: FormHandler;
  data?: Record<string, unknown>;
  mode?: 'view' | 'edit' | 'create';
}

/**
 * Component registry maps element types to renderers
 */
const componentRegistry: Map<string, ComponentRenderer> = new Map();

/**
 * Register a component renderer
 */
export function registerComponent(type: string, renderer: ComponentRenderer): void {
  componentRegistry.set(type, renderer);
}

/**
 * Get a component renderer
 */
export function getComponent(type: string): ComponentRenderer | undefined {
  return componentRegistry.get(type);
}

// ============================================================
// CORE RENDERERS
// ============================================================

/**
 * Render a DisplayElement to React
 */
export function renderElement(
  element: DisplayElement,
  key: string | number = 0,
  context: RenderContext = {}
): React.ReactNode {
  const renderer = componentRegistry.get(element.type);

  if (renderer) {
    return renderer(element, key, context);
  }

  // Fallback: render as div with children
  return (
    <div key={key} {...(element.props as React.HTMLAttributes<HTMLDivElement>)}>
      {element.text}
      {element.children?.map((child, i) => renderElement(child, i, context))}
    </div>
  );
}

/**
 * Render a DisplayLayout to React
 */
export function renderLayout(
  layout: DisplayLayout,
  context: RenderContext = {}
): React.ReactNode {
  const layoutClasses = getLayoutClasses(layout);

  return (
    <div className={layoutClasses}>
      {layout.children.map((child, i) => renderElement(child, i, context))}
    </div>
  );
}

/**
 * Render a DisplayDocument to React
 */
export function renderDocument(
  document: DisplayDocument,
  context: RenderContext = {}
): React.ReactNode {
  return (
    <div className="w-full">
      {document.title && (
        <h1 className="text-2xl font-bold mb-6">{document.title}</h1>
      )}
      {renderLayout(document.layout, context)}
    </div>
  );
}

// ============================================================
// LAYOUT HELPERS
// ============================================================

function getLayoutClasses(layout: DisplayLayout): string {
  const classes: string[] = [];

  switch (layout.type) {
    case 'stack':
      classes.push('flex flex-col');
      break;
    case 'row':
      classes.push('flex flex-row');
      break;
    case 'grid':
      classes.push('grid');
      if (layout.columns) {
        classes.push(`grid-cols-${layout.columns}`);
      }
      break;
    case 'card':
      classes.push('bg-white rounded-lg shadow-sm border border-gray-200 p-4');
      break;
    case 'page':
      classes.push('max-w-4xl mx-auto p-6');
      break;
  }

  if (layout.gap) {
    classes.push(`gap-${layout.gap}`);
  }

  if (layout.padding) {
    classes.push(`p-${layout.padding}`);
  }

  return classes.join(' ');
}

// ============================================================
// BUILT-IN COMPONENT RENDERERS
// ============================================================

// Text
registerComponent('text', (element, key) => (
  <span key={key} className={element.props?.className as string}>
    {element.text}
  </span>
));

// Heading
registerComponent('heading', (element, key) => {
  const level = (element.props?.level as number) || 1;
  const className = element.props?.className as string || '';
  const sizeClasses = ['text-3xl', 'text-2xl', 'text-xl', 'text-lg', 'text-base', 'text-sm'];
  const HeadingTag = level === 1 ? 'h1' : level === 2 ? 'h2' : level === 3 ? 'h3' : level === 4 ? 'h4' : level === 5 ? 'h5' : 'h6';

  return React.createElement(
    HeadingTag,
    { key, className: `font-bold ${sizeClasses[level - 1] || ''} ${className}` },
    element.text
  );
});

// Paragraph
registerComponent('paragraph', (element, key) => (
  <p key={key} className={`text-gray-700 ${element.props?.className || ''}`}>
    {element.text}
  </p>
));

// Container
registerComponent('container', (element, key, context) => (
  <div
    key={key}
    className={`${element.props?.className || ''}`}
  >
    {element.children?.map((child, i) => renderElement(child, i, context))}
  </div>
));

// Card
registerComponent('card', (element, key, context) => (
  <div
    key={key}
    className={`bg-white rounded-lg shadow-sm border border-gray-200 p-4 ${element.props?.className || ''}`}
  >
    {element.props?.title ? (
      <h3 className="text-lg font-semibold mb-3">{String(element.props.title)}</h3>
    ) : null}
    {element.children?.map((child, i) => renderElement(child, i, context))}
  </div>
));

// Stack (vertical)
registerComponent('stack', (element, key, context) => (
  <div
    key={key}
    className={`flex flex-col gap-4 ${element.props?.className || ''}`}
  >
    {element.children?.map((child, i) => renderElement(child, i, context))}
  </div>
));

// Row (horizontal)
registerComponent('row', (element, key, context) => (
  <div
    key={key}
    className={`flex flex-row gap-4 ${element.props?.className || ''}`}
  >
    {element.children?.map((child, i) => renderElement(child, i, context))}
  </div>
));

// Grid
registerComponent('grid', (element, key, context) => {
  const cols = (element.props?.columns as number) || 1;
  return (
    <div
      key={key}
      className={`grid grid-cols-${cols} gap-4 ${element.props?.className || ''}`}
    >
      {element.children?.map((child, i) => renderElement(child, i, context))}
    </div>
  );
});

// ============================================================
// FORM FIELD RENDERERS
// ============================================================

// Field wrapper
registerComponent('field', (element, key, context) => {
  const props = element.props as FormField & { value?: unknown };
  const fieldType = props.type || 'text';

  // Get field renderer
  const fieldRenderer = componentRegistry.get(`field:${fieldType}`);
  if (fieldRenderer) {
    return fieldRenderer(element, key, context);
  }

  // Default text field
  return renderTextField(element, key, context);
});

// Text field
function renderTextField(
  element: DisplayElement,
  key: string | number,
  context: RenderContext
): React.ReactNode {
  const props = element.props as FormField & { value?: unknown };

  return (
    <div key={key} className="mb-4">
      <label htmlFor={props.id} className="block text-sm font-medium text-gray-700 mb-1">
        {props.label}
        {props.required && <span className="text-red-500 ml-1">*</span>}
      </label>
      <input
        type="text"
        id={props.id}
        name={props.id}
        required={props.required}
        disabled={props.disabled}
        placeholder={props.placeholder}
        defaultValue={props.value as string}
        onChange={(e) => context.handler?.onChange?.(props.id, (e.target as HTMLInputElement).value)}
        className="w-full rounded-md border border-gray-300 py-2 px-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />
    </div>
  );
}

registerComponent('field:text', renderTextField);

// Email field
registerComponent('field:email', (element, key, context) => {
  const props = element.props as FormField & { value?: unknown };

  return (
    <div key={key} className="mb-4">
      <label htmlFor={props.id} className="block text-sm font-medium text-gray-700 mb-1">
        {props.label}
        {props.required && <span className="text-red-500 ml-1">*</span>}
      </label>
      <input
        type="email"
        id={props.id}
        name={props.id}
        required={props.required}
        disabled={props.disabled}
        placeholder={props.placeholder}
        defaultValue={props.value as string}
        onChange={(e) => context.handler?.onChange?.(props.id, (e.target as HTMLInputElement).value)}
        className="w-full rounded-md border border-gray-300 py-2 px-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />
    </div>
  );
});

// Number field
registerComponent('field:number', (element, key, context) => {
  const props = element.props as FormField & { value?: unknown };

  return (
    <div key={key} className="mb-4">
      <label htmlFor={props.id} className="block text-sm font-medium text-gray-700 mb-1">
        {props.label}
        {props.required && <span className="text-red-500 ml-1">*</span>}
      </label>
      <input
        type="number"
        id={props.id}
        name={props.id}
        required={props.required}
        disabled={props.disabled}
        placeholder={props.placeholder}
        defaultValue={props.value as number}
        onChange={(e) => context.handler?.onChange?.(props.id, Number((e.target as HTMLInputElement).value))}
        className="w-full rounded-md border border-gray-300 py-2 px-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />
    </div>
  );
});

// Select field
registerComponent('field:select', (element, key, context) => {
  const props = element.props as FormField & {
    value?: unknown;
    options?: Array<{ value: string; label: string }>;
  };

  return (
    <div key={key} className="mb-4">
      <label htmlFor={props.id} className="block text-sm font-medium text-gray-700 mb-1">
        {props.label}
        {props.required && <span className="text-red-500 ml-1">*</span>}
      </label>
      <select
        id={props.id}
        name={props.id}
        required={props.required}
        disabled={props.disabled}
        defaultValue={props.value as string}
        onChange={(e) => context.handler?.onChange?.(props.id, (e.target as HTMLSelectElement).value)}
        className="w-full rounded-md border border-gray-300 py-2 px-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      >
        {props.required && !props.value && (
          <option value="" disabled>
            -- Select {props.label} --
          </option>
        )}
        {props.options?.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </div>
  );
});

// Date field
registerComponent('field:date', (element, key, context) => {
  const props = element.props as FormField & { value?: unknown };
  let defaultDate = '';

  if (props.value) {
    const dateObj = new Date(props.value as string);
    if (!isNaN(dateObj.getTime())) {
      defaultDate = dateObj.toISOString().split('T')[0];
    }
  }

  return (
    <div key={key} className="mb-4">
      <label htmlFor={props.id} className="block text-sm font-medium text-gray-700 mb-1">
        {props.label}
        {props.required && <span className="text-red-500 ml-1">*</span>}
      </label>
      <input
        type="date"
        id={props.id}
        name={props.id}
        required={props.required}
        disabled={props.disabled}
        defaultValue={defaultDate}
        onChange={(e) => context.handler?.onChange?.(props.id, (e.target as HTMLInputElement).value)}
        className="w-full rounded-md border border-gray-300 py-2 px-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />
    </div>
  );
});

// Textarea field
registerComponent('field:textarea', (element, key, context) => {
  const props = element.props as FormField & { value?: unknown; rows?: number };

  return (
    <div key={key} className="mb-4">
      <label htmlFor={props.id} className="block text-sm font-medium text-gray-700 mb-1">
        {props.label}
        {props.required && <span className="text-red-500 ml-1">*</span>}
      </label>
      <textarea
        id={props.id}
        name={props.id}
        required={props.required}
        disabled={props.disabled}
        placeholder={props.placeholder}
        defaultValue={props.value as string}
        rows={props.rows || 4}
        onChange={(e) => context.handler?.onChange?.(props.id, (e.target as HTMLTextAreaElement).value)}
        className="w-full rounded-md border border-gray-300 py-2 px-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />
    </div>
  );
});

// ============================================================
// ACTION RENDERERS
// ============================================================

// Button
registerComponent('button', (element, key, context) => {
  const props = element.props as {
    id?: string;
    label?: string;
    variant?: 'primary' | 'secondary' | 'ghost' | 'danger';
    type?: 'submit' | 'button' | 'reset';
    disabled?: boolean;
  };

  const variantClasses = {
    primary: 'bg-blue-600 text-white hover:bg-blue-700',
    secondary: 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50',
    ghost: 'text-gray-600 hover:bg-gray-100',
    danger: 'bg-red-600 text-white hover:bg-red-700',
  };

  const variant = props.variant || 'primary';

  return (
    <button
      key={key}
      type={props.type || 'button'}
      disabled={props.disabled}
      onClick={() => {
        if (props.id) {
          context.handler?.onAction?.(props.id);
        }
      }}
      className={`px-4 py-2 text-sm font-medium rounded-md transition-colors ${variantClasses[variant]} ${props.disabled ? 'opacity-50 cursor-not-allowed' : ''}`}
    >
      {props.label || element.text}
    </button>
  );
});

// Action group
registerComponent('actions', (element, key, context) => (
  <div key={key} className="flex justify-end gap-3 mt-6 pt-4 border-t border-gray-200">
    {element.children?.map((child, i) => renderElement(child, i, context))}
  </div>
));

// ============================================================
// DISPLAY VALUE RENDERERS (for view mode)
// ============================================================

// Display value (read-only field display)
registerComponent('value', (element, key) => {
  const props = element.props as { label?: string; value?: unknown };

  return (
    <div key={key} className="mb-3">
      {props.label && (
        <dt className="text-sm font-medium text-gray-500">{props.label}</dt>
      )}
      <dd className="mt-1 text-sm text-gray-900">{String(props.value ?? '-')}</dd>
    </div>
  );
});

// ============================================================
// REACT ADAPTER CLASS
// ============================================================

/**
 * ReactAdapter: Transforms DisplayDocument to React components
 *
 * Usage:
 *   const adapter = new ReactAdapter();
 *   const jsx = adapter.render(displayDocument, { handler, data });
 */
export class ReactAdapter {
  /**
   * Render a DisplayDocument to React
   */
  render(
    document: DisplayDocument,
    context: RenderContext = {}
  ): React.ReactNode {
    return renderDocument(document, context);
  }

  /**
   * Render a DisplayElement to React
   */
  renderElement(
    element: DisplayElement,
    context: RenderContext = {}
  ): React.ReactNode {
    return renderElement(element, 0, context);
  }

  /**
   * Register a custom component renderer
   */
  register(type: string, renderer: ComponentRenderer): void {
    registerComponent(type, renderer);
  }
}

/**
 * Default ReactAdapter instance
 */
export const reactAdapter = new ReactAdapter();

// ============================================================
// REACT COMPONENT WRAPPER
// ============================================================

/**
 * DisplayDocumentView: React component that renders a DisplayDocument
 */
export interface DisplayDocumentViewProps {
  document: DisplayDocument;
  handler?: FormHandler;
  data?: Record<string, unknown>;
  mode?: 'view' | 'edit' | 'create';
}

export function DisplayDocumentView({
  document,
  handler,
  data,
  mode = 'view',
}: DisplayDocumentViewProps): React.ReactElement {
  return <>{renderDocument(document, { handler, data, mode })}</>;
}

/**
 * FormShapeView: React component that renders a FormShape via FormView
 *
 * This is the high-level component for rendering forms.
 * It creates a FormView, renders to DisplayDocument, then to React.
 */
export interface FormShapeViewProps {
  shape: FormShape;
  handler?: FormHandler;
  mode?: 'view' | 'edit' | 'create';
}

// Note: FormShapeView would import SimpleFormView and render through it
// For now, we just render the shape directly
export function FormShapeView({
  shape,
  handler,
  mode = 'view',
}: FormShapeViewProps): React.ReactElement {
  // Build a simple DisplayDocument from the FormShape
  const document: DisplayDocument = {
    title: shape.title || shape.name,
    layout: {
      type: mode === 'view' ? 'card' : 'stack',
      children: [
        // Render fields
        ...shape.fields.map((field) => ({
          type: mode === 'view' ? 'value' : 'field',
          props: {
            ...field,
            value: field.value,
          },
        })),
        // Render actions in edit/create mode
        ...(mode !== 'view'
          ? [
              {
                type: 'actions',
                children: [
                  {
                    type: 'button',
                    props: { id: 'cancel', label: 'Cancel', variant: 'secondary' },
                  },
                  {
                    type: 'button',
                    props: { id: 'submit', label: 'Save', variant: 'primary', type: 'submit' },
                  },
                ],
              },
            ]
          : []),
      ],
    },
  };

  return <DisplayDocumentView document={document} handler={handler} mode={mode} />;
}
