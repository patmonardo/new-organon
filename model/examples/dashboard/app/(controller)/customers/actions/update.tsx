"use server";

import { CustomerController } from "@controller/customer";

/**
 * Server action that proxies to the controller method for updating a customer
 * @param id The customer id to update
 * @param formData Form data with updated customer information
 */
export default async function updateCustomer(id: string, formData: FormData) {
  return CustomerController.updateCustomer(id, formData);
}
