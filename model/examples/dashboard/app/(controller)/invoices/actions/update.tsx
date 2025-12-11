"use server";

import { InvoiceController } from "@controller/invoice";

/**
 * Server action that proxies to the controller method for updating a invoice
 * @param id The invoice id to update
 * @param formData Form data with updated invoice information
 */
export default async function updateInvoice(id: string, formData: FormData) {
  return InvoiceController.updateInvoice(id, formData);
}
