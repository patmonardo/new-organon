/**
 * Adapter - Runtime Binding for MVC SDSL
 *
 * Adapters translate Generic Display Language (DisplayDocument)
 * into runtime-specific output (React components, HTML, etc.)
 *
 * The Adapter is responsible for:
 * - Consuming DisplayDocument from the Controller
 * - Translating to runtime-specific format
 * - Managing runtime lifecycle (events, state, etc.)
 */

import type {
  DisplayDocument,
  DisplayElement,
  FormHandler,
  OperationResult
} from './types';
import type { FormController } from './form-controller';

/**
 * AdapterOutput: What an adapter produces
 * Generic type T depends on the runtime
 */
export interface AdapterOutput<T> {
  content: T;
  handler: FormHandler;
  meta?: Record<string, unknown>;
}

/**
 * Adapter: Abstract interface for runtime adapters
 *
 * Implement this interface for each runtime:
 * - ReactAdapter: DisplayDocument → React components
 * - NestAdapter: DisplayDocument → API response
 * - CLIAdapter: DisplayDocument → terminal output
 */
export interface Adapter<T> {
  /**
   * Adapter name/identifier
   */
  readonly name: string;

  /**
   * Transform DisplayDocument to runtime output
   */
  transform(doc: DisplayDocument, handler: FormHandler): AdapterOutput<T>;

  /**
   * Transform a single DisplayElement
   */
  transformElement(element: DisplayElement): T;

  /**
   * Adapt a controller (convenience method)
   */
  adapt(controller: FormController): AdapterOutput<T>;
}

/**
 * AbstractAdapter: Base implementation with common logic
 */
export abstract class AbstractAdapter<T> implements Adapter<T> {
  abstract readonly name: string;

  transform(doc: DisplayDocument, handler: FormHandler): AdapterOutput<T> {
    const layoutElement: DisplayElement = {
      type: 'layout',
      props: {
        type: doc.layout.type,
        gap: doc.layout.gap,
        columns: doc.layout.columns,
        padding: doc.layout.padding,
      },
      children: doc.layout.children
    };

    return {
      content: this.transformElement(layoutElement),
      handler,
      meta: doc.meta
    };
  }

  abstract transformElement(element: DisplayElement): T;

  adapt(controller: FormController): AdapterOutput<T> {
    const doc = controller.display();
    const handler = controller.createHandler();
    return this.transform(doc, handler);
  }
}

/**
 * JSONAdapter: Produces JSON output (for APIs, testing)
 */
export class JSONAdapter extends AbstractAdapter<object> {
  readonly name = 'json';

  transformElement(element: DisplayElement): object {
    return {
      type: element.type,
      props: element.props,
      text: element.text,
      children: element.children?.map(c => this.transformElement(c))
    };
  }
}

/**
 * HTMLAdapter: Produces HTML string output
 */
export class HTMLAdapter extends AbstractAdapter<string> {
  readonly name = 'html';

  private elementMap: Record<string, string> = {
    'layout': 'div',
    'field': 'div',
    'button': 'button',
    'actions': 'div',
    'text': 'span',
    'heading': 'h2',
  };

  transformElement(element: DisplayElement): string {
    const tag = this.elementMap[element.type] || element.type;
    const props = element.props
      ? Object.entries(element.props)
          .filter(([_, v]) => v !== undefined)
          .map(([k, v]) => `${this.toHtmlAttr(k)}="${v}"`)
          .join(' ')
      : '';

    const openTag = props ? `<${tag} ${props}>` : `<${tag}>`;
    const closeTag = `</${tag}>`;

    if (element.text) {
      return `${openTag}${element.text}${closeTag}`;
    }

    if (element.children && element.children.length > 0) {
      const children = element.children.map(c => this.transformElement(c)).join('\n');
      return `${openTag}\n${children}\n${closeTag}`;
    }

    return `<${tag}${props ? ' ' + props : ''} />`;
  }

  private toHtmlAttr(key: string): string {
    // Convert camelCase to kebab-case for HTML attributes
    return key.replace(/([A-Z])/g, '-$1').toLowerCase();
  }
}

/**
 * AdapterRegistry: Registry of available adapters
 */
export class AdapterRegistry {
  private adapters: Map<string, Adapter<unknown>> = new Map();

  register<T>(adapter: Adapter<T>): void {
    this.adapters.set(adapter.name, adapter);
  }

  get<T>(name: string): Adapter<T> | undefined {
    return this.adapters.get(name) as Adapter<T> | undefined;
  }

  list(): string[] {
    return Array.from(this.adapters.keys());
  }
}

/**
 * Default adapter registry with built-in adapters
 */
export const defaultAdapters = new AdapterRegistry();
defaultAdapters.register(new JSONAdapter());
defaultAdapters.register(new HTMLAdapter());

// React adapter would be added separately since it requires React
// defaultAdapters.register(new ReactAdapter());
