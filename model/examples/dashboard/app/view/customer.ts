//@view/customer.ts7
import { ReactNode } from "react";
import { OperationResult } from "@/data/schema/base";
import { Customer } from "@/data/schema/customer";
import { CustomerFormShape } from "@graphics/schema/customer";
import { CustomerForm } from "@graphics/form/customer";
import { CustomerTable } from "@graphics/table/customer";
import { FormView } from "./form";

export class CustomerView extends FormView<CustomerFormShape> {
  constructor(private readonly customer?: Customer) {
    super(new CustomerForm(customer));
  }

  public async displayTable(
    customers: Customer[],
    totalPages = 1
  ): Promise<OperationResult<ReactNode>> {
    // Create the specialized table
    const table = new CustomerTable(customers);

    // Set pagination in the shape
    table.withTransformations((shape) => {
      shape.state.totalPages = totalPages;
      shape.state.page = 1; // Default to first page
    });

    // Direct rendering - no dependency on parent class
    return {
      status: "success",
      data: await table.render(),
      message: "Table rendered successfully",
    };
  }
}
