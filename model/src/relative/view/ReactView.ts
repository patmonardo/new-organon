// model/src/view/react-form.ts
import { ReactNode } from "react";
import { FormView } from "./FormView";
import { Form, FormShape, FormMode, FormContent, FormHandler, OperationResult } from "@/logic/schema/morph";

export class ReactFormView<T extends FormShape> extends FormView<T> {
  /**
   * Display a form as a React/JSX component for GUI apps.
   */
  public async display(
    mode: FormMode,
    content: FormContent = "jsx",
    handler: FormHandler
  ): Promise<OperationResult<ReactNode>> {
    if (!this.form) {
      return {
        status: "error",
        data: null,
        message: "No form available for display",
      };
    }
    try {
      const rendered = await this.form.render(mode, content, handler);
      return {
        status: "success",
        data: rendered,
        message: "Form rendered successfully",
      };
    } catch (error) {
      return {
        status: "error",
        data: null,
        message: `Form render error: ${error}`,
      };
    }
  }
}

// Factory method (example)
export function createFormView<T extends FormShape>(
  form: Form<T>,
  platform: "react" | "nest"
): FormView<T> {
  if (platform === "react") {
    return new ReactFormView(form);
  }
  return new FormView(form);
}
