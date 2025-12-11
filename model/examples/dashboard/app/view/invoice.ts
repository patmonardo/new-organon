//@view/invoice.ts
import { ReactNode } from "react";
import { OperationResult } from "@/data/schema/base";
import { Invoice, InvoiceWithCustomer } from "@/data/schema/invoice";
import { InvoiceFormShape } from "@graphics/schema/invoice";
import { InvoiceForm } from "@graphics/form/invoice";
import { InvoiceTable } from "@graphics/table/invoice";
import { FormView } from "./form";

export class InvoiceView extends FormView<InvoiceFormShape> {
  constructor(private readonly invoice?: Invoice) {
    super(new InvoiceForm(invoice));
  }

  public async displayTable(
    invoices: InvoiceWithCustomer[],
    totalPages = 1
  ): Promise<OperationResult<ReactNode>> {
    // Create the specialized table
    const table = new InvoiceTable(invoices);

    // Set pagination in the shape
    table.withTransformations((shape) => {
      shape.state.totalPages = totalPages;
      shape.state.page = 1; // Default to first page
    });

    // Use parent implementation to render
    return {
      status: "success",
      data: await table.render(),
      message: "Table rendered successfully",
    };
  }

  public async displayLatest(
    invoices: InvoiceWithCustomer[]
  ): Promise<OperationResult<ReactNode>> {
    // Create the table
    const table = new InvoiceTable(invoices);

    // Apply transformations to the shape - all in one transformation for clarity
    table.withTransformations((shape) => {
      // Layout changes
      shape.layout.searchable = false;
      shape.layout.paginated = false;
      shape.layout.addButton = undefined;
      shape.layout.title = "Latest Invoices";

      // State changes
      shape.state.totalPages = 1;
      shape.state.page = 1;
    });
    // Direct rendering, consistent with displayTable method
    return {
      status: "success",
      data: await table.render(),
      message: "Table rendered successfully",
    };
  }
}
