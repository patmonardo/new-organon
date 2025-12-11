import { InvoiceController } from "@controller/invoice";

export default async function Page(props: { params: Promise<{ id: string }> }) {
  const params = await props.params;
  const id = params.id;
  return (
    <main className="max-w-4xl mx-auto p-4">
      {await InvoiceController.editForm(params.id)}
    </main>
  );
}
