/**
 * List Schema Tests
 */

import { describe, it, expect } from 'vitest';
import { ListShapeSchema, ListItemSchema, ListLayoutSchema, ListNavigationSchema } from '../src/schema/list';

describe('ListSchema', () => {
  it('should validate a minimal list shape', () => {
    const list = {
      id: 'list-1',
      name: 'Test List',
      fields: [],
      items: [],
      layout: {
        id: 'layout-1',
      },
    };

    const result = ListShapeSchema.safeParse(list);
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.type).toBe('list');
      expect(result.data.items).toEqual([]);
    }
  });

  it('should validate list items', () => {
    const item = {
      id: 'item-1',
      content: {
        title: 'Item Title',
        description: 'Item description',
      },
    };

    const result = ListItemSchema.safeParse(item);
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.content.title).toBe('Item Title');
    }
  });

  it('should validate list layout types', () => {
    const layouts = ['linear', 'grid', 'hierarchical'];
    layouts.forEach(type => {
      const layout = {
        id: 'layout-test',
        type,
      };
      const result = ListLayoutSchema.safeParse(layout);
      expect(result.success).toBe(true);
    });
  });

  it('should validate navigation options', () => {
    const nav = {
      search: true,
      pagination: true,
      filter: true,
      sort: true,
    };

    const result = ListNavigationSchema.safeParse(nav);
    expect(result.success).toBe(true);
  });

  it('should validate a full list shape with items and navigation', () => {
    const list = {
      id: 'list-full',
      name: 'Full List',
      fields: [],
      type: 'list',
      items: [
        {
          id: 'item-1',
          content: { title: 'First Item', value: 100 },
        },
        {
          id: 'item-2',
          content: { title: 'Second Item', value: 200 },
        },
      ],
      layout: {
        id: 'layout-full',
        type: 'grid',
        compact: true,
        zebra: true,
      },
      navigation: {
        search: true,
        pagination: true,
      },
    };

    const result = ListShapeSchema.safeParse(list);
    expect(result.success).toBe(true);
    if (result.success) {
      expect(result.data.items.length).toBe(2);
      expect(result.data.layout.type).toBe('grid');
      expect(result.data.navigation?.search).toBe(true);
    }
  });
});

