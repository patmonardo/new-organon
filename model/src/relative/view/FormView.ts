// model/src/view/form.ts
export class FormView<T extends FormShape> {
  constructor(protected readonly form?: Form<T>) {}

  /**
   * Display a form in a generic (server/API) context.
   * Returns a string or a simple object for API responses.
   */
  public async display(
    mode: FormMode,
    content: FormContent = "json",
    handler: FormHandler
  ): Promise<OperationResult<string | object>> {
    if (!this.form) {
      return {
        status: "error",
        data: null,
        message: "No form available for display",
      };
    }
    try {
      // Server-side rendering or API serialization
      const rendered = await this.form.serialize(mode, content, handler);
      return {
        status: "success",
        data: rendered,
        message: "Form serialized successfully",
      };
    } catch (error) {
      return {
        status: "error",
        data: null,
        message: `Form serialization error: ${error}`,
      };
    }
  }
}
