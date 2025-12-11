import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { ButtonShape } from '@graphics/schema/button';
import { CustomerButton } from './customer';

// Mock the renderer component with proper typing
vi.mock('./renderer', () => ({
  ButtonRenderer: ({ shape }: { shape: ButtonShape }) => (
    <div data-testid="button-renderer" data-shape={JSON.stringify(shape)}>
      {shape.label}
    </div>
  )
}));

describe('CustomerButton', () => {
  it('configures an Edit button with correct properties', () => {
    const button = new CustomerButton('123', 'edit');
    const { container } = render(<>{button.render()}</>);

    // Get the rendered component with shape data
    const rendererEl = screen.getByTestId('button-renderer');
    const shape = JSON.parse(rendererEl.getAttribute('data-shape') || '{}');

    // Verify the shape configuration
    expect(shape.variant).toBe('secondary');
    expect(shape.icon).toBe('pencil');
    expect(shape.label).toBe('Edit');
    expect(shape.href).toBe('/customers/123/edit');
    expect(shape.srOnly).toBe(true);
  });

  it('configures a Delete button with correct properties', () => {
    const button = new CustomerButton('123', 'delete');
    const { container } = render(<>{button.render()}</>);

    // Get the rendered component with shape data
    const rendererEl = screen.getByTestId('button-renderer');
    const shape = JSON.parse(rendererEl.getAttribute('data-shape') || '{}');

    // Verify the shape configuration
    expect(shape.variant).toBe('danger');
    expect(shape.icon).toBe('trash');
    expect(shape.label).toBe('Delete');
    expect(shape.confirmMessage).toContain('Are you sure');
    expect(shape.refreshAfterAction).toBe(true);
  });
});
