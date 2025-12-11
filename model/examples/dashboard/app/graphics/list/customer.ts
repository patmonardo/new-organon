//@graphics/list/customer.ts
import { Customer } from "@/data/schema/customer";
import { ListShape } from "@graphics/schema/list";

export function createCustomerList(customers: Customer[]): ListShape {
  return {
    items: customers.map(customer => ({
      id: customer.id,
      content: customer,
      relations: [
        {
          label: "Edit",
          href: `/customers/${customer.id}/edit`,
          relation: "action",
          icon: "pencil"
        },
        {
          label: "Delete",
          href: `/customers/${customer.id}/delete`,
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
      { label: "Add Customer", href: "/customers/create", relation: "action", icon: "plus" }
    ]
  };
}
