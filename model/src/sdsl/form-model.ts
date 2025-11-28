/**
 * FormModel - The Model in MVC SDSL
 *
 * State : Structure dyad
 *
 * FormModel receives FormShape from the FactStore (First Speaker)
 * and provides data access operations (CRUD).
 *
 * The Model is responsible for:
 * - Holding the current state (data)
 * - Defining the structure (schema)
 * - Performing data operations
 * - Validating data integrity
 */

import type { FormShape, FormField, OperationResult } from './types';

/**
 * FormModel: Abstract base class for all Models in MVC SDSL
 *
 * Extend this class to create domain-specific models:
 * - CustomerModel extends FormModel
 * - InvoiceModel extends FormModel
 * - DashboardModel extends FormModel
 */
export abstract class FormModel<T extends FormShape = FormShape> {
  protected _shape: T;
  protected _dirty: boolean = false;

  constructor(shape: T) {
    this._shape = shape;
  }

  // ============================================================
  // STATE - The current data
  // ============================================================

  /**
   * Get the current shape (read-only)
   */
  get shape(): T {
    return this._shape;
  }

  /**
   * Get a specific field value
   */
  getField(fieldId: string): unknown {
    const field = this._shape.fields.find(f => f.id === fieldId);
    return field?.value;
  }

  /**
   * Set a field value (marks model as dirty)
   */
  setField(fieldId: string, value: unknown): void {
    const fieldIndex = this._shape.fields.findIndex(f => f.id === fieldId);
    if (fieldIndex >= 0) {
      this._shape.fields[fieldIndex] = {
        ...this._shape.fields[fieldIndex],
        value
      };
      this._dirty = true;
    }
  }

  /**
   * Check if model has unsaved changes
   */
  get isDirty(): boolean {
    return this._dirty;
  }

  /**
   * Mark model as clean (after save)
   */
  markClean(): void {
    this._dirty = false;
  }

  // ============================================================
  // STRUCTURE - The schema/definition
  // ============================================================

  /**
   * Get field definitions
   */
  getFields(): FormField[] {
    return this._shape.fields;
  }

  /**
   * Get field definition by id
   */
  getFieldDef(fieldId: string): FormField | undefined {
    return this._shape.fields.find(f => f.id === fieldId);
  }

  // ============================================================
  // VALIDATION - Data integrity
  // ============================================================

  /**
   * Validate the current state
   * Override in subclass for custom validation
   */
  validate(): OperationResult<boolean> {
    const errors: Record<string, string[]> = {};

    for (const field of this._shape.fields) {
      if (field.required && (field.value === undefined || field.value === '')) {
        errors[field.id] = [`${field.label || field.id} is required`];
      }
    }

    if (Object.keys(errors).length > 0) {
      return {
        status: 'error',
        data: false,
        message: 'Validation failed',
        errors
      };
    }

    return {
      status: 'success',
      data: true,
      message: 'Validation passed'
    };
  }

  // ============================================================
  // CRUD - Data operations (abstract - implement in subclass)
  // ============================================================

  /**
   * Load model data from source
   */
  abstract load(id?: string): Promise<OperationResult<T>>;

  /**
   * Save model data to source
   */
  abstract save(): Promise<OperationResult<T>>;

  /**
   * Delete model data from source
   */
  abstract delete(): Promise<OperationResult<boolean>>;

  // ============================================================
  // SERIALIZATION
  // ============================================================

  /**
   * Convert to plain object
   */
  toJSON(): T {
    return { ...this._shape };
  }

  /**
   * Create from plain object
   */
  static fromJSON<T extends FormShape>(data: T): FormModel<T> {
    throw new Error('fromJSON must be implemented in subclass');
  }
}

/**
 * SimpleFormModel: Concrete implementation for simple use cases
 * Uses in-memory storage (for testing/prototyping)
 */
export class SimpleFormModel<T extends FormShape = FormShape> extends FormModel<T> {
  private static storage: Map<string, FormShape> = new Map();

  async load(id?: string): Promise<OperationResult<T>> {
    const loadId = id || this._shape.id;
    const data = SimpleFormModel.storage.get(loadId);

    if (data) {
      this._shape = data as T;
      this._dirty = false;
      return {
        status: 'success',
        data: this._shape,
        message: 'Loaded successfully'
      };
    }

    return {
      status: 'error',
      data: null,
      message: `Not found: ${loadId}`
    };
  }

  async save(): Promise<OperationResult<T>> {
    const validation = this.validate();
    if (validation.status === 'error') {
      return {
        status: 'error',
        data: null,
        message: validation.message,
        errors: validation.errors
      };
    }

    SimpleFormModel.storage.set(this._shape.id, this._shape);
    this._dirty = false;

    return {
      status: 'success',
      data: this._shape,
      message: 'Saved successfully'
    };
  }

  async delete(): Promise<OperationResult<boolean>> {
    const deleted = SimpleFormModel.storage.delete(this._shape.id);

    return {
      status: deleted ? 'success' : 'error',
      data: deleted,
      message: deleted ? 'Deleted successfully' : 'Not found'
    };
  }

  // Test helper
  static clearStorage(): void {
    SimpleFormModel.storage.clear();
  }
}
