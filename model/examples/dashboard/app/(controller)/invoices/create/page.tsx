import { InvoiceController } from "@controller/invoice";

export default async function CreateInvoicePage() {
  return (
    <main className="max-w-4xl mx-auto p-4">
      {await InvoiceController.createForm()}
    </main>
  );
}
