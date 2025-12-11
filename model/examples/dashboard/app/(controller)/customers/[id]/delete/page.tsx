import { CustomerController } from "@controller/customer";

export default async function Page(props: { params: Promise<{ id: string }> }) {
  const params = await props.params;
  const id = params.id;

  // Call delete method on controller
  await CustomerController.deleteCustomer(id);
}
