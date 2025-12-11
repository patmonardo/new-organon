"use server";

import { CustomerController } from "@controller/customer";

/**
 * Server action that proxies to the controller method for canceling operations
 */
export default async function cancel() {
  return CustomerController.cancelCustomer();
}
