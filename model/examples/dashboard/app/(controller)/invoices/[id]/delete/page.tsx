//@/(controller)/invoices/[id]/delete/page.tsx
import { InvoiceController } from "@controller/invoice";

export default async function Page(props: { params: Promise<{ id: string }> }) {
  const params = await props.params;
  const id = params.id;

  // Call delete method on controller
  await InvoiceController.deleteInvoice(id);
}
