/**
 * Customer MVC Example - Entry Point
 * 
 * This is the MVC SDSL Design Template for building business forms.
 * 
 * Architecture:
 * 
 * 1. Schema (schema.ts)
 *    - Define FormShape with fields, layout, actions
 *    - Define data validation schemas (Create, Update)
 * 
 * 2. Model (model.ts)
 *    - Static CRUD methods (data access layer)
 *    - Uses FactStore for persistence
 * 
 * 3. FormModel (form-model.ts)
 *    - Extends FormModel for form state
 *    - Handles validation and persistence
 * 
 * 4. FormView (form-view.ts)
 *    - Extends FormView for rendering
 *    - Produces DisplayDocument
 * 
 * 5. Controller (controller.ts)
 *    - Extends FormController
 *    - Transport-agnostic business logic
 *    - Uses CustomerFormModel and CustomerFormView
 * 
 * 6. View (view.tsx) [Optional]
 *    - React-specific rendering
 *    - For direct JSX output
 * 
 * Property as Center:
 * - Schema = Property.schema() (Codegen)
 * - Model values = Property.values() (Eval)
 */

// Schema exports
export {
  CustomerDataSchema,
  CreateCustomerSchema,
  UpdateCustomerSchema,
  CustomerFormShape,
  customerFields,
  type CustomerData,
  type CreateCustomer,
  type UpdateCustomer,
} from "./schema";

// Data access layer (CRUD)
export {
  CustomerModel,
  type CustomerModelState,
} from "./model";

// Prisma-backed data access (for production)
export { CustomerPrismaModel } from "./prisma-model";

// Form model (extends FormModel)
export { CustomerFormModel } from "./form-model";

// Form view (extends FormView)
export { CustomerFormView } from "./form-view";

// React view (direct JSX rendering)
export { CustomerView, type FormMode } from "./view";

// Controller (extends FormController)
export {
  CustomerController,
  createCustomerAction,
  updateCustomerAction,
  deleteCustomerAction,
  customerTrpcRouter,
  customerRestHandlers,
} from "./controller";

// Re-export types from sdsl for convenience
export type {
  ControllerResult,
  FormDefinition,
  ListResult,
  OperationResult,
} from "../../sdsl/types";
