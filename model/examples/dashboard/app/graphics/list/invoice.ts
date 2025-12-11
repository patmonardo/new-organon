//@graphics/list/invoice.ts
import { Invoice } from "@/data/schema/invoice";
import { ListShape } from "@graphics/schema/list";

export function createInvoiceList(invoices: Invoice[]): ListShape {
  return {
    items: invoices.map(invoice => ({
      id: invoice.id,
      content: invoice,
      relations: [
        {
          label: "Edit",
          href: `/invoices/${invoice.id}/edit`,
          relation: "action",
          icon: "pencil"
        },
        {
          label: "Delete",
          href: `/invoices/${invoice.id}/delete`,
          relation: "action",
          icon: "trash"
        }
      ]
    })),
    layout: {
      type: "linear",
      spacing: "normal"
    },
    navigation: {
      search: true,
      pagination: true,
      creation: true
    },
    relations: [
      { label: "Add Invoice", href: "/invoices/create", relation: "action", icon: "plus" }
    ]
  };
}
