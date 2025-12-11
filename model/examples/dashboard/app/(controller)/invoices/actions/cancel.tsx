"use server";

import { InvoiceController } from "@controller/invoice";

/**
 * Server action that proxies to the controller method for canceling operations
 */
export default async function cancel() {
  return InvoiceController.cancelInvoice();
}
