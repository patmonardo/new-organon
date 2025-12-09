import { describe, it, expect } from 'vitest';
import { ButtonRenderer } from './button';
import { ButtonShape } from '@/model/schema/button';

describe('ButtonRenderer', () => {
  it('should render a button with label', () => {
    const shape: ButtonShape = {
      id: 'btn-1',
      label: 'Click me',
      variant: 'primary',
      disabled: false,
      refreshAfterAction: false,
      srOnly: false,
    };

    // Smoke test: ensure component can be instantiated
    expect(() => ButtonRenderer({ shape })).not.toThrow();
  });

  it('should render a link button with href', () => {
    const shape: ButtonShape = {
      id: 'btn-2',
      label: 'Go somewhere',
      href: '/destination',
      variant: 'secondary',
      disabled: false,
      refreshAfterAction: false,
      srOnly: false,
    };

    expect(() => ButtonRenderer({ shape })).not.toThrow();
  });

  it('should handle disabled state', () => {
    const shape: ButtonShape = {
      id: 'btn-3',
      label: 'Disabled',
      disabled: true,
      variant: 'primary',
      refreshAfterAction: false,
      srOnly: false,
    };

    expect(() => ButtonRenderer({ shape })).not.toThrow();
  });
});

