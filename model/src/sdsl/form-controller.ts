/**
 * FormController - The Controller in MVC SDSL
 *
 * Action : Rule dyad
 *
 * FormController orchestrates the Model→View pipeline.
 * It sublates Model and View within itself, producing
 * Generic Display Language that Adapters consume.
 *
 * The Controller is responsible for:
 * - Orchestrating Model and View
 * - Handling actions (submit, cancel, etc.)
 * - Applying rules (validation, authorization)
 * - Producing output for Adapters
 */

import type {
  FormShape,
  FormMode,
  FormContent,
  FormHandler,
  OperationResult,
  DisplayDocument
} from './types';
import { FormModel, SimpleFormModel } from './form-model';
import { FormView, SimpleFormView } from './form-view';

/**
 * ControllerAction: An action the controller can perform
 */
export interface ControllerAction {
  id: string;
  name: string;
  handler: (data?: unknown) => Promise<OperationResult<unknown>>;
  rules?: string[];  // Rule IDs that must pass
}

/**
 * ControllerRule: A rule that governs actions
 */
export interface ControllerRule {
  id: string;
  name: string;
  check: (context: ControllerContext) => boolean;
  message?: string;
}

/**
 * ControllerContext: Context for rule evaluation
 */
export interface ControllerContext {
  mode: FormMode;
  model: FormModel;
  user?: { id: string; roles?: string[] };
}

/**
 * FormController: Abstract base class for all Controllers in MVC SDSL
 *
 * Extend this class to create domain-specific controllers:
 * - CustomerController extends FormController
 * - InvoiceController extends FormController
 * - DashboardController extends FormController
 */
export abstract class FormController<T extends FormShape = FormShape> {
  protected _model: FormModel<T>;
  protected _view: FormView<T>;
  protected _mode: FormMode;
  protected _actions: Map<string, ControllerAction> = new Map();
  protected _rules: Map<string, ControllerRule> = new Map();

  constructor(model: FormModel<T>, view: FormView<T>, mode: FormMode = 'view') {
    this._model = model;
    this._view = view;
    this._mode = mode;

    // Register default actions
    this.registerDefaultActions();
  }

  // ============================================================
  // ORCHESTRATION - Model→View pipeline
  // ============================================================

  /**
   * Get the current mode
   */
  get mode(): FormMode {
    return this._mode;
  }

  /**
   * Set mode (updates view as well)
   */
  setMode(mode: FormMode): void {
    this._mode = mode;
    this._view.setMode(mode);
  }

  /**
   * Get the model
   */
  get model(): FormModel<T> {
    return this._model;
  }

  /**
   * Get the view
   */
  get view(): FormView<T> {
    return this._view;
  }

  /**
   * Produce DisplayDocument by running the pipeline
   * This is the main output method
   */
  display(): DisplayDocument {
    return this._view.render();
  }

  /**
   * Serialize to a specific content format
   */
  serialize(content: FormContent): OperationResult<string | object> {
    return this._view.serialize(content);
  }

  // ============================================================
  // ACTIONS - What the controller can do
  // ============================================================

  /**
   * Register an action
   */
  registerAction(action: ControllerAction): void {
    this._actions.set(action.id, action);
  }

  /**
   * Execute an action
   */
  async executeAction(actionId: string, data?: unknown): Promise<OperationResult<unknown>> {
    const action = this._actions.get(actionId);

    if (!action) {
      return {
        status: 'error',
        data: null,
        message: `Unknown action: ${actionId}`
      };
    }

    // Check rules if any
    if (action.rules && action.rules.length > 0) {
      const context: ControllerContext = {
        mode: this._mode,
        model: this._model
      };

      for (const ruleId of action.rules) {
        const rule = this._rules.get(ruleId);
        if (rule && !rule.check(context)) {
          return {
            status: 'error',
            data: null,
            message: rule.message || `Rule failed: ${ruleId}`
          };
        }
      }
    }

    // Execute the action
    return action.handler(data);
  }

  /**
   * Create a FormHandler that routes to controller actions
   */
  createHandler(): FormHandler {
    return {
      onSubmit: async (data: FormShape) => {
        await this.executeAction('submit', data);
      },
      onCancel: () => {
        this.executeAction('cancel');
      },
      onAction: async (actionId: string, data?: unknown) => {
        await this.executeAction(actionId, data);
      },
      onChange: (fieldId: string, value: unknown) => {
        this._model.setField(fieldId, value);
      }
    };
  }

  /**
   * Register default actions (submit, cancel, etc.)
   */
  protected registerDefaultActions(): void {
    this.registerAction({
      id: 'submit',
      name: 'Submit',
      rules: ['valid'],
      handler: async () => {
        return this._model.save();
      }
    });

    this.registerAction({
      id: 'cancel',
      name: 'Cancel',
      handler: async () => {
        return {
          status: 'success' as const,
          data: null,
          message: 'Cancelled'
        };
      }
    });

    this.registerAction({
      id: 'delete',
      name: 'Delete',
      handler: async () => {
        return this._model.delete();
      }
    });

    // Register default validation rule
    this._rules.set('valid', {
      id: 'valid',
      name: 'Validation',
      check: (ctx) => ctx.model.validate().status === 'success',
      message: 'Validation failed'
    });
  }

  // ============================================================
  // RULES - What governs actions
  // ============================================================

  /**
   * Register a rule
   */
  registerRule(rule: ControllerRule): void {
    this._rules.set(rule.id, rule);
  }

  /**
   * Check if a rule passes
   */
  checkRule(ruleId: string): boolean {
    const rule = this._rules.get(ruleId);
    if (!rule) return true;

    const context: ControllerContext = {
      mode: this._mode,
      model: this._model
    };

    return rule.check(context);
  }
}

/**
 * SimpleFormController: Concrete implementation for simple use cases
 */
export class SimpleFormController<T extends FormShape = FormShape> extends FormController<T> {
  constructor(shape: T, mode: FormMode = 'view') {
    const model = new SimpleFormModel(shape);
    const view = new SimpleFormView(model, mode);
    super(model, view, mode);
  }
}

/**
 * Factory function to create a complete MVC stack
 */
export function createFormMVC<T extends FormShape>(
  shape: T,
  mode: FormMode = 'view'
): SimpleFormController<T> {
  return new SimpleFormController(shape, mode);
}
