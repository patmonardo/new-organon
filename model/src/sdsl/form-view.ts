/**
 * FormView - The View in MVC SDSL
 *
 * Representation : Perspective dyad
 *
 * FormView transforms FormShape into Generic Display Language.
 * It produces DisplayDocument that Adapters consume.
 *
 * The View is responsible for:
 * - Transforming data into display representation
 * - Applying perspective (filtering, formatting)
 * - Producing Generic Display Language (not framework-specific)
 */

import type {
  FormShape,
  FormMode,
  FormContent,
  OperationResult,
  DisplayDocument,
  DisplayElement,
  DisplayLayout
} from './types';
import type { FormModel } from './form-model';

/**
 * FormView: Abstract base class for all Views in MVC SDSL
 *
 * Extend this class to create domain-specific views:
 * - CustomerView extends FormView
 * - InvoiceView extends FormView
 * - DashboardView extends FormView
 */
export abstract class FormView<T extends FormShape = FormShape> {
  protected _model: FormModel<T>;
  protected _mode: FormMode;

  constructor(model: FormModel<T>, mode: FormMode = 'view') {
    this._model = model;
    this._mode = mode;
  }

  // ============================================================
  // MODE - How the form is being used
  // ============================================================

  get mode(): FormMode {
    return this._mode;
  }

  setMode(mode: FormMode): void {
    this._mode = mode;
  }

  // ============================================================
  // REPRESENTATION - Transform data to display
  // ============================================================

  /**
   * Produce a DisplayDocument from the model
   * This is the main output method - produces Generic Display Language
   */
  abstract render(): DisplayDocument;

  /**
   * Serialize to a specific content format
   */
  serialize(content: FormContent): OperationResult<string | object> {
    try {
      const doc = this.render();

      switch (content) {
        case 'json':
          return {
            status: 'success',
            data: doc,
            message: 'Serialized to JSON'
          };

        case 'html':
          return {
            status: 'success',
            data: this.toHTML(doc),
            message: 'Serialized to HTML'
          };

        case 'xml':
          return {
            status: 'success',
            data: this.toXML(doc),
            message: 'Serialized to XML'
          };

        case 'jsx':
          // JSX is handled by the Adapter, not serialization
          return {
            status: 'success',
            data: doc,
            message: 'DisplayDocument ready for JSX adapter'
          };

        default:
          return {
            status: 'error',
            data: null,
            message: `Unknown content type: ${content}`
          };
      }
    } catch (error) {
      return {
        status: 'error',
        data: null,
        message: `Serialization error: ${error}`
      };
    }
  }

  // ============================================================
  // PERSPECTIVE - Filter and format
  // ============================================================

  /**
   * Filter fields based on mode and permissions
   * Override in subclass for custom filtering
   */
  protected filterFields(): T['fields'] {
    const fields = this._model.getFields();

    switch (this._mode) {
      case 'view':
        // Show all non-hidden fields
        return fields;

      case 'edit':
        // Show editable fields
        return fields.filter(f => !f.disabled);

      case 'create':
        // Show fields for creation
        return fields.filter(f => !f.disabled);

      default:
        return fields;
    }
  }

  /**
   * Format a field value for display
   * Override in subclass for custom formatting
   */
  protected formatValue(fieldId: string, value: unknown): string {
    if (value === null || value === undefined) {
      return '';
    }
    return String(value);
  }

  // ============================================================
  // SERIALIZATION HELPERS
  // ============================================================

  /**
   * Convert DisplayDocument to HTML string
   */
  protected toHTML(doc: DisplayDocument): string {
    const renderElement = (el: DisplayElement, indent: number = 0): string => {
      const spaces = '  '.repeat(indent);
      const props = el.props
        ? Object.entries(el.props)
            .map(([k, v]) => `${k}="${v}"`)
            .join(' ')
        : '';

      if (el.text) {
        return `${spaces}<${el.type}${props ? ' ' + props : ''}>${el.text}</${el.type}>`;
      }

      if (el.children && el.children.length > 0) {
        const children = el.children.map(c => renderElement(c, indent + 1)).join('\n');
        return `${spaces}<${el.type}${props ? ' ' + props : ''}>\n${children}\n${spaces}</${el.type}>`;
      }

      return `${spaces}<${el.type}${props ? ' ' + props : ''} />`;
    };

    const layoutEl: DisplayElement = {
      type: 'div',
      props: { class: `layout-${doc.layout.type}` },
      children: doc.layout.children
    };

    return `<!DOCTYPE html>
<html>
<head>
  <title>${doc.title || 'Form'}</title>
</head>
<body>
${renderElement(layoutEl, 1)}
</body>
</html>`;
  }

  /**
   * Convert DisplayDocument to XML string
   */
  protected toXML(doc: DisplayDocument): string {
    const renderElement = (el: DisplayElement, indent: number = 0): string => {
      const spaces = '  '.repeat(indent);
      const props = el.props
        ? Object.entries(el.props)
            .map(([k, v]) => `${k}="${v}"`)
            .join(' ')
        : '';

      if (el.text) {
        return `${spaces}<${el.type}${props ? ' ' + props : ''}>${el.text}</${el.type}>`;
      }

      if (el.children && el.children.length > 0) {
        const children = el.children.map(c => renderElement(c, indent + 1)).join('\n');
        return `${spaces}<${el.type}${props ? ' ' + props : ''}>\n${children}\n${spaces}</${el.type}>`;
      }

      return `${spaces}<${el.type}${props ? ' ' + props : ''} />`;
    };

    return `<?xml version="1.0" encoding="UTF-8"?>
<document title="${doc.title || 'Form'}">
  <layout type="${doc.layout.type}">
${doc.layout.children.map(c => renderElement(c, 2)).join('\n')}
  </layout>
</document>`;
  }
}

/**
 * SimpleFormView: Concrete implementation for simple use cases
 * Produces a basic form layout
 */
export class SimpleFormView<T extends FormShape = FormShape> extends FormView<T> {
  render(): DisplayDocument {
    const shape = this._model.shape;
    const fields = this.filterFields();

    // Build field elements
    const fieldElements: DisplayElement[] = fields.map(field => ({
      type: 'field',
      props: {
        id: field.id,
        type: field.type,
        label: field.label || field.id,
        required: field.required,
        disabled: field.disabled || this._mode === 'view',
      },
      text: this.formatValue(field.id, field.value)
    }));

    // Build layout based on mode
    const layout: DisplayLayout = {
      type: this._mode === 'view' ? 'card' : 'stack',
      gap: 4,
      children: fieldElements
    };

    // Add action buttons for edit/create modes
    if (this._mode !== 'view') {
      layout.children.push({
        type: 'actions',
        children: [
          {
            type: 'button',
            props: { action: 'submit', variant: 'primary' },
            text: this._mode === 'create' ? 'Create' : 'Save'
          },
          {
            type: 'button',
            props: { action: 'cancel', variant: 'secondary' },
            text: 'Cancel'
          }
        ]
      });
    }

    return {
      title: shape.title || shape.name || 'Form',
      layout,
      meta: shape.meta
    };
  }
}
