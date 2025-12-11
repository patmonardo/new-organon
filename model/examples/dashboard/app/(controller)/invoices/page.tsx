import { Suspense } from "react";
import { InvoiceController } from "@controller/invoice";

export default async function Page(props: {
  searchParams?: Promise<{
      query?: string;
      page?: string;
  }>;
}) {
  const searchParams = await props.searchParams;
  const query = searchParams?.query || '';
  const page = Number(searchParams?.page) || 1;
  return (
    <main className="max-w-7xl mx-auto p-4">
      <Suspense key={query + page} fallback={<div>Loading...</div>}>
        {await InvoiceController.listInvoices(query, page)}
      </Suspense>
    </main>
  );
}
