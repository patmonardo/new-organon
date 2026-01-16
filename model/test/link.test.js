/**
 * Link Schema Tests
 */
import { describe, it, expect } from 'vitest';
import { LinkShapeSchema, LinkTypeSchema, LinkVariantSchema, LinkSizeSchema } from '../src/schema/link';
describe('LinkSchema', () => {
    it('should validate a minimal link shape', () => {
        const link = {
            id: 'link-1',
            name: 'Test Link',
            fields: [],
            layout: {
                id: 'layout-1',
                label: 'Click me',
            },
        };
        const result = LinkShapeSchema.safeParse(link);
        expect(result.success).toBe(true);
        if (result.success) {
            expect(result.data.type).toBe('navigate');
            expect(result.data.layout.label).toBe('Click me');
        }
    });
    it('should validate link types', () => {
        const types = ['action', 'navigate', 'external', 'reference'];
        types.forEach(type => {
            const result = LinkTypeSchema.safeParse(type);
            expect(result.success).toBe(true);
        });
    });
    it('should validate link variants', () => {
        const variants = ['primary', 'secondary', 'ghost', 'danger', 'button'];
        variants.forEach(variant => {
            const result = LinkVariantSchema.safeParse(variant);
            expect(result.success).toBe(true);
        });
    });
    it('should validate link sizes', () => {
        const sizes = ['small', 'medium', 'large'];
        sizes.forEach(size => {
            const result = LinkSizeSchema.safeParse(size);
            expect(result.success).toBe(true);
        });
    });
    it('should validate a full link shape with all options', () => {
        const link = {
            id: 'link-full',
            name: 'Full Link',
            fields: [],
            type: 'external',
            layout: {
                id: 'layout-full',
                label: 'External Link',
                href: 'https://example.com',
                icon: 'external-link',
                variant: 'button',
                size: 'large',
                disabled: false,
                target: '_blank',
                className: 'custom-class',
            },
        };
        const result = LinkShapeSchema.safeParse(link);
        expect(result.success).toBe(true);
        if (result.success) {
            expect(result.data.type).toBe('external');
            expect(result.data.layout.href).toBe('https://example.com');
            expect(result.data.layout.target).toBe('_blank');
        }
    });
});
//# sourceMappingURL=link.test.js.map