"use server";

import { CustomerController } from "@controller/customer";

/**
 * Server action that proxies to the controller method for updating a customer
 * @param formData Form data with updated customer information
 */
export default async function createCustomer(formData: FormData) {
  return CustomerController.createCustomer(formData);
}
