import { FormShape, FormField } from "@@model/src/schema/shape";
import { FormContext, FormExecutionContext } from "@../../model/src/schema/context";

/**
 * Type guard to check if a context is a FormContext
*/
export function isFormContext(context: any): context is FormContext {
  return context && typeof context === "object" && "form" in context;
}

export type { FormShape, FormField } from "@@model/src/schema/shape";
export type { FormContext, FormExecutionContext } from "@../../model/src/schema/context";
