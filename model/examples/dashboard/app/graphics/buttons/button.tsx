import { ReactNode } from 'react';
import { ButtonShape } from '@graphics/schema/button';
import { ButtonShapeAdapter } from './adapter';

/**
 * Abstract base class for all button types
 */
export abstract class Button<T extends ButtonShape> {
  /**
   * Get the button shape configuration
   */
  protected abstract getButtonShape(): T;

  /**
   * Render the button as a React component
   */
  public render(): ReactNode {
    const shape = this.getButtonShape();
    return this.renderButton(shape);
  }

  /**
   * Render implementation - uses adapter
   */
  protected renderButton(shape: T): ReactNode {
    return ButtonShapeAdapter.toJSX(shape);
  }
}
