/**
 * React Adapter Tests
 *
 * Tests the transformation of DisplayDocument → React components
 */

import { describe, it, expect } from 'vitest';
import {
  renderElement,
  renderLayout,
  renderDocument,
  ReactAdapter,
  reactAdapter,
  registerComponent,
} from '../src/sdsl/react-adapter';
import type { DisplayDocument, DisplayElement, DisplayLayout } from '../src/sdsl/types';

describe('React Adapter', () => {
  describe('renderElement', () => {
    it('renders text element', () => {
      const element: DisplayElement = {
        type: 'text',
        text: 'Hello World',
      };

      const result = renderElement(element);
      expect(result).toBeDefined();
    });

    it('renders heading element', () => {
      const element: DisplayElement = {
        type: 'heading',
        text: 'Title',
        props: { level: 1 },
      };

      const result = renderElement(element);
      expect(result).toBeDefined();
    });

    it('renders container with children', () => {
      const element: DisplayElement = {
        type: 'container',
        children: [
          { type: 'text', text: 'Child 1' },
          { type: 'text', text: 'Child 2' },
        ],
      };

      const result = renderElement(element);
      expect(result).toBeDefined();
    });

    it('renders card element', () => {
      const element: DisplayElement = {
        type: 'card',
        props: { title: 'Card Title' },
        children: [{ type: 'paragraph', text: 'Card content' }],
      };

      const result = renderElement(element);
      expect(result).toBeDefined();
    });
  });

  describe('renderLayout', () => {
    it('renders stack layout', () => {
      const layout: DisplayLayout = {
        type: 'stack',
        children: [
          { type: 'text', text: 'Item 1' },
          { type: 'text', text: 'Item 2' },
        ],
      };

      const result = renderLayout(layout);
      expect(result).toBeDefined();
    });

    it('renders grid layout', () => {
      const layout: DisplayLayout = {
        type: 'grid',
        columns: 2,
        gap: 4,
        children: [
          { type: 'text', text: 'Cell 1' },
          { type: 'text', text: 'Cell 2' },
        ],
      };

      const result = renderLayout(layout);
      expect(result).toBeDefined();
    });
  });

  describe('renderDocument', () => {
    it('renders complete document', () => {
      const document: DisplayDocument = {
        title: 'Test Document',
        layout: {
          type: 'stack',
          children: [
            { type: 'heading', text: 'Welcome', props: { level: 1 } },
            { type: 'paragraph', text: 'This is a test document.' },
          ],
        },
      };

      const result = renderDocument(document);
      expect(result).toBeDefined();
    });
  });

  describe('ReactAdapter class', () => {
    it('creates default instance', () => {
      expect(reactAdapter).toBeDefined();
      expect(reactAdapter).toBeInstanceOf(ReactAdapter);
    });

    it('renders document via adapter', () => {
      const document: DisplayDocument = {
        title: 'Adapter Test',
        layout: {
          type: 'card',
          children: [{ type: 'text', text: 'Content' }],
        },
      };

      const result = reactAdapter.render(document);
      expect(result).toBeDefined();
    });

    it('registers custom component', () => {
      const adapter = new ReactAdapter();
      adapter.register('custom', (element, key) => {
        return null; // Custom renderer returns null for testing
      });

      // Custom component should be registered globally
      const element: DisplayElement = { type: 'custom', text: 'Custom' };
      const result = renderElement(element);
      expect(result).toBeNull();
    });
  });

  describe('Form field renderers', () => {
    it('renders text field', () => {
      const element: DisplayElement = {
        type: 'field',
        props: {
          id: 'name',
          type: 'text',
          label: 'Name',
          required: true,
        },
      };

      const result = renderElement(element);
      expect(result).toBeDefined();
    });

    it('renders select field', () => {
      const element: DisplayElement = {
        type: 'field:select',
        props: {
          id: 'country',
          label: 'Country',
          options: [
            { value: 'us', label: 'United States' },
            { value: 'uk', label: 'United Kingdom' },
          ],
        },
      };

      const result = renderElement(element);
      expect(result).toBeDefined();
    });
  });

  describe('Action renderers', () => {
    it('renders button', () => {
      const element: DisplayElement = {
        type: 'button',
        props: {
          id: 'submit',
          label: 'Submit',
          variant: 'primary',
        },
      };

      const result = renderElement(element);
      expect(result).toBeDefined();
    });

    it('renders action group', () => {
      const element: DisplayElement = {
        type: 'actions',
        children: [
          { type: 'button', props: { id: 'cancel', label: 'Cancel', variant: 'secondary' } },
          { type: 'button', props: { id: 'submit', label: 'Save', variant: 'primary' } },
        ],
      };

      const result = renderElement(element);
      expect(result).toBeDefined();
    });
  });
});
