"use server";

import { InvoiceController } from "@controller/invoice";

/**
 * Server action that proxies to the controller method for deleting a customer
 */
export default async function deleteInvoice(id: string) {
  return InvoiceController.deleteInvoice(id);
}
