//@view/invoice.ts
import { ReactNode } from "react";
import Image from "next/image";
import { InvoiceWithCustomer } from "@/data/schema/invoice";
import {
  TableColumn,
  TableLayout,
  TableShape,
} from "@graphics/schema/table";
import { defineTable } from "@graphics/schema/table";
import { ButtonRenderer } from "@/graphics/button/renderer";
import { Table } from "./table";
import InvoiceStatus from "@/(controller)/invoices/status";

const DEFAULT_IMAGE = "/icons/favicon.ico";

export class InvoiceTable extends Table<TableShape> {
  constructor(
    invoices: InvoiceWithCustomer[],
    options?: { overrideLayout?: Partial<TableLayout> }
  ) {
    super(invoices);

    // Apply layout overrides if provided using the new transformation pattern
    if (options?.overrideLayout) {
      this.withTransformations((shape) => {
        shape.layout = {
          ...shape.layout,
          ...options.overrideLayout,
        };
      });
    }
  }

  protected defineShape(): TableShape {
    return defineTable({
      columns: [
        {
          key: "customer",
          label: "Customer",
          className: "text-left",
          width: "auto",
          sortable: true,
          filterable: true,
        },
        {
          key: "email",
          label: "Email",
          className: "text-left",
          width: "auto",
          sortable: true,
          filterable: true,
        },
        {
          key: "date",
          label: "Date",
          className: "text-center",
          width: "auto",
          sortable: true,
          filterable: true,
        },
        {
          key: "amount",
          label: "Amount",
          className: "text-right",
          width: "auto",
          sortable: true,
          filterable: true,
        },
        {
          key: "status",
          label: "Status",
          className: "text-left",
          width: "auto",
          sortable: true,
          filterable: true,
        },
      ],
      layout: {
        title: "Invoices",
        responsive: true,
        searchable: true,
        paginated: true,
        striped: false,
        hoverable: true,
        addButton: {
          label: "Create Invoice",
          href: "/invoices/create",
          icon: "plus",
        },
      },
      state: {
        status: "idle",
        page: 1,
        totalPages: 1,
        message: undefined,
        sortColumn: undefined,
        sortDirection: undefined,
      },
      actions: [
        { id: "edit", type: "edit", label: "Edit", icon: "pencil" },
        { id: "delete", type: "delete", label: "Delete", icon: "trash" },
      ],
    });
  }

  // Override to provide custom cell rendering
  public renderCell(
    column: TableColumn,
    invoice: InvoiceWithCustomer
  ): ReactNode {
    // console.log("Rendering cell for column:", column.key);
    switch (column.key) {
      case "customer":
        return (
          <div className="flex items-center gap-3">
            <Image
              src={invoice.customer.imageUrl || DEFAULT_IMAGE}
              className="rounded-full"
              width={28}
              height={28}
              alt={`${invoice.customer.name}'s profile picture`}
            />
            <p>{invoice.customer.name}</p>
          </div>
        );
      case "email":
        return invoice.customer.email;
      case "amount":
        return this.formatCurrency(invoice.amount);
      case "date":
        return this.formatDate(invoice.date);
      case "status":
        return <InvoiceStatus status={invoice.status} />;
      default:
        return super.renderCell(column, invoice);
    }
  }

  // Update the end of invoice.tsx with a proper renderActions method
  public renderActions(
    invoice: InvoiceWithCustomer,
    actions: any[]
  ): ReactNode {
    return (
      <>
        {actions.map((action, index) => {
          // Generate URL based on action type
          let href = "#";
          if (action.type === "edit") {
            href = `/invoices/${invoice.id}/edit`;
          } else if (action.type === "delete") {
            href = `/invoices//${invoice.id}/delete`;
          } else if (action.type === "view") {
            href = `/invoices/${invoice.id}`;
          }
          return (
            <ButtonRenderer
              key={index}
              shape={{
                variant: action.variant || "secondary",
                label: action.label,
                icon: action.icon,
                href: href,
                srOnly: true,
                disabled: false,
                onClick: undefined,
                refreshAfterAction: false,
                confirmMessage: undefined,
                customClass: undefined,
                size: "md",
                iconSource: "heroicons",
              }}
            />
          );
        })}
      </>
    );
  }

  // Override to provide custom mobile card rendering
  public renderMobileCard(invoice: InvoiceWithCustomer): ReactNode {
    return (
      <>
        <div className="flex items-center justify-between border-b pb-4">
          <div>
            <div className="mb-2 flex items-center">
              <Image
                src={invoice.customer.imageUrl || DEFAULT_IMAGE}
                className="mr-2 rounded-full"
                width={28}
                height={28}
                alt={`${invoice.customer.name}'s profile picture`}
              />
              <p>{invoice.customer.name}</p>
            </div>
            <p className="text-sm text-gray-500">{invoice.customer.email}</p>
          </div>
          <InvoiceStatus status={invoice.status} />
        </div>
        <div className="flex w-full items-center justify-between pt-4">
          <div>
            <p className="text-xl font-medium">
              {this.formatCurrency(invoice.amount)}
            </p>
            <p>{this.formatDate(invoice.date)}</p>
          </div>
        </div>
      </>
    );
  }

  formatCurrency(amount: number): React.ReactNode {
    const formatter = new Intl.NumberFormat("en-US", {
      style: "currency",
      currency: "USD",
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    });

    // Format the amount but we'll use it for display only
    const formatted = formatter.format(amount);

    // Split the formatted string at the decimal point
    const [dollars, cents] = formatted.split(".");

    // Return a structured format for better alignment
    return (
      <div className="tabular-nums text-right">
        <span className="dollars">{dollars}</span>
        <span className="cents text-gray-500">.{cents}</span>
      </div>
    );
  }

  // Add this method to your InvoiceTable class
  formatDate(date: Date): React.ReactNode {
    // Get year, month, day with proper padding
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");

    return (
      <div className="tabular-nums text-right">
        <span className="month-day">
          {month}/{day}
        </span>
        <span className="year text-gray-500">/{year}</span>
      </div>
    );
  }
}
