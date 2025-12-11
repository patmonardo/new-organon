import { ReactNode } from "react";
import Image from "next/image";
import { Customer } from "@/data/schema/customer";
import { TableShape, TableColumn, TableLayout } from "@graphics/schema/table";
import { defineTable } from "@graphics/schema/table";
import { ButtonRenderer } from "@/graphics/button/renderer";
import { Table } from "./table";

const DEFAULT_IMAGE = "/icons/favicon.ico";
export class CustomerTable extends Table<TableShape> {
  constructor(
    customers: Customer[],
    options?: { overrideLayout?: Partial<TableLayout> }
  ) {
    super(customers);

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

  // Change to defineShape instead of getTableShape and make it protected
  protected defineShape(): TableShape {
    return defineTable({
      columns: [
        {
          key: "name",
          label: "Name",
          width: "auto",
          className: "text-left",
          sortable: true,
          filterable: true,
        },
        {
          key: "email",
          label: "Email",
          width: "auto",
          className: "text-left",
          sortable: true,
          filterable: true,
        },
      ],
      layout: {
        title: "Customers",
        responsive: true,
        searchable: true,
        paginated: true,
        striped: false,
        hoverable: false,
        addButton: {
          label: "Add Customer",
          href: "/customers/create",
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
  public renderCell(column: TableColumn, customer: Customer): ReactNode {
    switch (column.key) {
      case "name":
        return (
          <div className="flex items-center gap-3">
            <Image
              src={customer.imageUrl || DEFAULT_IMAGE}
              className="rounded-full"
              width={28}
              height={28}
              alt={`${customer.name}'s profile picture`}
            />
            <p>{customer.name}</p>
          </div>
        );
      case "email":
        return (
          <div className="flex items-center gap-3">
            <p>{customer.email}</p>
          </div>
        );
      default:
        return super.renderCell(column, customer);
    }
  }

  // In customer.tsx
  public renderActions(item: Customer, actions: any[]): ReactNode {
    return (
      <>
        {actions.map((action, index) => {
          // Generate URL based on action type
          let href = "#";
          if (action.type === "edit") {
            href = `/customers/${item.id}/edit`;
          } else if (action.type === "delete") {
            href = `/customers/${item.id}/delete`;
          } else if (action.type === "view") {
            href = `/customers/${item.id}/view`;
          }
          return (
            <ButtonRenderer
              key={index}
              shape={{
                variant: action.variant || "secondary",
                label: action.label,
                icon: action.icon,
                href: href,
                disabled: false,
                onClick: undefined,
                refreshAfterAction: false,
                confirmMessage: undefined,
                srOnly: true,
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
  public renderMobileCard(customer: Customer): ReactNode {
    return (
      <>
        <div className="flex items-center justify-between border-b pb-4">
          <div className="flex items-center">
            <Image
              src={customer.imageUrl || DEFAULT_IMAGE}
              alt={`${customer.name}'s profile picture`}
              className="mr-2 rounded-full"
              width={28}
              height={28}
            />
            <div>
              <p className="font-medium">{customer.name}</p>
              <p className="text-sm text-gray-500">{customer.email}</p>
            </div>
          </div>
        </div>
      </>
    );
  }
}
