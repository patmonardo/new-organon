"use server";

import { CustomerController } from "@controller/customer";

/**
 * Server action that proxies to the controller method for deleting a customer
 */
export default async function deleteCustomer(id: string) {
  return CustomerController.deleteCustomer(id);
}
