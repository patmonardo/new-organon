/**
 * ReactController - React/Next Controller Dyad Partner
 *
 * Extends FormController with React/Next.js specific capabilities:
 * - Server Actions generation
 * - FormData parsing
 * - Next.js revalidation/redirect
 *
 * The ReactController is the Controller half of the React dyad:
 *   ReactView + ReactController = React MVC Dyad
 *
 * Both share the same FormModel.
 */

import type { FormShape, FormMode, OperationResult, FormHandler } from '../../../sdsl/types';
import { FormController, SimpleFormController } from '../../../sdsl/form-controller';
import { ReactView } from './react-view';
import type { FormModel } from '../../../sdsl/form-model';
import { SimpleFormModel } from '../../../sdsl/form-model';

// ============================================================
// TYPES
// ============================================================

/**
 * ServerActionResult: What Server Actions return
 */
export interface ServerActionResult {
  success: boolean;
  message?: string;
  errors?: Record<string, string[]>;
  redirect?: string;
  revalidate?: string | string[];
}

/**
 * ServerAction: A Next.js Server Action function
 */
export type ServerAction = (formData: FormData) => Promise<ServerActionResult>;

/**
 * ReactControllerOptions: Configuration for ReactController
 */
export interface ReactControllerOptions {
  /** Path to redirect after successful submission */
  redirectOnSuccess?: string;
  /** Paths to revalidate after mutation */
  revalidatePaths?: string[];
  /** Custom form data parser */
  parseFormData?: (formData: FormData) => Record<string, unknown>;
}

// ============================================================
// REACT CONTROLLER CLASS
// ============================================================

/**
 * ReactController: React/Next.js specific FormController
 *
 * Generates Server Actions and handles Next.js-specific routing concerns.
 */
export class ReactController<T extends FormShape = FormShape> extends SimpleFormController<T> {
  protected _options: ReactControllerOptions;
  protected _reactView: ReactView<T>;

  constructor(
    shape: T,
    mode: FormMode = 'edit',
    options: ReactControllerOptions = {}
  ) {
    super(shape, mode);
    this._options = options;
    this._reactView = new ReactView(this._model, mode);
  }

  // ============================================================
  // REACT VIEW ACCESS
  // ============================================================

  /**
   * Get the ReactView for this controller
   */
  get reactView(): ReactView<T> {
    return this._reactView;
  }

  /**
   * Set the mode on both view and controller
   */
  setMode(mode: FormMode): void {
    this._mode = mode;
    this._reactView.setMode(mode);
  }

  // ============================================================
  // SERVER ACTIONS
  // ============================================================

  /**
   * Create the main form submission Server Action
   */
  createSubmitAction(): ServerAction {
    return async (formData: FormData): Promise<ServerActionResult> => {
      // Parse form data
      const data = this.parseFormData(formData);

      // Update model with form data
      for (const [key, value] of Object.entries(data)) {
        this._model.setField(key, value);
      }

      // Execute the submit action
      const result = await this.executeAction('submit', data);

      return this.toServerActionResult(result);
    };
  }

  /**
   * Create a Server Action for a specific action ID
   */
  createAction(actionId: string): ServerAction {
    return async (formData: FormData): Promise<ServerActionResult> => {
      const data = this.parseFormData(formData);

      // Update model
      for (const [key, value] of Object.entries(data)) {
        this._model.setField(key, value);
      }

      const result = await this.executeAction(actionId, data);
      return this.toServerActionResult(result);
    };
  }

  /**
   * Create a cancel action (no form data needed)
   */
  createCancelAction(): () => Promise<ServerActionResult> {
    return async (): Promise<ServerActionResult> => {
      const result = await this.executeAction('cancel');
      return this.toServerActionResult(result, { redirect: this._options.redirectOnSuccess });
    };
  }

  /**
   * Create a delete action
   */
  createDeleteAction(): ServerAction {
    return async (formData: FormData): Promise<ServerActionResult> => {
      const id = formData.get('id') as string;
      const result = await this.executeAction('delete', { id });
      return this.toServerActionResult(result);
    };
  }

  // ============================================================
  // FORM HANDLER FOR REACT
  // ============================================================

  /**
   * Create a FormHandler configured for React/Server Actions
   */
  createReactHandler(): FormHandler {
    const submitAction = this.createSubmitAction();
    const cancelAction = this.createCancelAction();

    return {
      onSubmit: async (data: FormShape) => {
        // Convert to FormData for consistency
        const formData = new FormData();
        for (const field of data.fields) {
          if (field.value !== undefined) {
            formData.append(field.id, String(field.value));
          }
        }
        await submitAction(formData);
      },
      onCancel: async () => {
        await cancelAction();
      },
      onAction: async (actionId: string, data?: unknown) => {
        const action = this.createAction(actionId);
        const formData = new FormData();
        if (data && typeof data === 'object') {
          for (const [key, value] of Object.entries(data)) {
            formData.append(key, String(value));
          }
        }
        await action(formData);
      },
      onChange: (fieldId: string, value: unknown) => {
        this._model.setField(fieldId, value);
      },
    };
  }

  // ============================================================
  // HELPERS
  // ============================================================

  /**
   * Parse FormData to a plain object
   */
  protected parseFormData(formData: FormData): Record<string, unknown> {
    if (this._options.parseFormData) {
      return this._options.parseFormData(formData);
    }

    const data: Record<string, unknown> = {};
    for (const [key, value] of formData.entries()) {
      // Try to parse numbers
      if (typeof value === 'string' && !isNaN(Number(value)) && value.trim() !== '') {
        data[key] = Number(value);
      } else {
        data[key] = value;
      }
    }
    return data;
  }

  /**
   * Convert OperationResult to ServerActionResult
   */
  protected toServerActionResult(
    result: OperationResult<unknown>,
    overrides?: Partial<ServerActionResult>
  ): ServerActionResult {
    const base: ServerActionResult = {
      success: result.status === 'success',
      message: result.message,
      errors: result.errors,
    };

    // Add redirect/revalidate for successful mutations
    if (result.status === 'success') {
      if (this._options.redirectOnSuccess || overrides?.redirect) {
        base.redirect = overrides?.redirect || this._options.redirectOnSuccess;
      }
      if (this._options.revalidatePaths) {
        base.revalidate = this._options.revalidatePaths;
      }
    }

    return { ...base, ...overrides };
  }

  // ============================================================
  // STATIC FACTORY
  // ============================================================

  /**
   * Create a ReactController from a FormShape
   */
  static create<T extends FormShape>(
    shape: T,
    mode: FormMode = 'edit',
    options?: ReactControllerOptions
  ): ReactController<T> {
    return new ReactController(shape, mode, options);
  }

  /**
   * Create both ReactController and its Server Actions
   */
  static createWithActions<T extends FormShape>(
    shape: T,
    mode: FormMode = 'edit',
    options?: ReactControllerOptions
  ): {
    controller: ReactController<T>;
    submitAction: ServerAction;
    cancelAction: () => Promise<ServerActionResult>;
    deleteAction: ServerAction;
  } {
    const controller = new ReactController(shape, mode, options);
    return {
      controller,
      submitAction: controller.createSubmitAction(),
      cancelAction: controller.createCancelAction(),
      deleteAction: controller.createDeleteAction(),
    };
  }
}

// ============================================================
// HELPER FUNCTION FOR NEXT.JS
// ============================================================

/**
 * createFormActions: Quick factory for Next.js Server Actions
 *
 * Usage in Next.js:
 * ```tsx
 * // app/customer/[id]/actions.ts
 * 'use server';
 * import { createFormActions } from '@organon/model';
 * import { customerSchema } from './schema';
 *
 * export const { submitAction, cancelAction } = createFormActions(
 *   customerSchema,
 *   { redirectOnSuccess: '/customers', revalidatePaths: ['/customers'] }
 * );
 * ```
 */
export function createFormActions<T extends FormShape>(
  shape: T,
  options?: ReactControllerOptions
): {
  submitAction: ServerAction;
  cancelAction: () => Promise<ServerActionResult>;
  deleteAction: ServerAction;
  controller: ReactController<T>;
} {
  return ReactController.createWithActions(shape, 'edit', options);
}
