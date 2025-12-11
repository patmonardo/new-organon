//@/(controller)/dashboard/page.tsx
import { Suspense } from "react";
import { Card } from "@/graphics/card/card";
import { InvoiceController } from "@controller/invoice";
import { RevenueController } from "@controller/revenue";
import { DashboardModel } from "@model/dashboard";
import {
  CardSkeleton,
  RevenueChartSkeleton,
  LatestInvoicesSkeleton,
} from "@/graphics/style/skeletons";

export default async function Page(props: {
  searchParams?: Promise<{
    query?: string;
    page?: string;
  }>;
}) {
  const searchParams = await props.searchParams;
  const query = searchParams?.query || "";
  const page = Number(searchParams?.page) || 1;
  // Fetch card data using the model
  const data = await DashboardModel.getCardData();
  return (
    <main className="p-6">
      <h1 className="mb-6 text-2xl font-bold">Dashboard</h1>

      {/* Cards row */}
      <div className="grid gap-6 grid-cols-1 md:grid-cols-4 mb-6">
        <Suspense fallback={<CardSkeleton />}>
          <Card
            title="Total Customers"
            value={data.numberOfCustomers.toLocaleString()}
            type="customers"
          />
        </Suspense>
        <Suspense fallback={<CardSkeleton />}>
          <Card
            title="Pending"
            value={data.totalPendingInvoices.toString()}
            type="pending"
          />
        </Suspense>
        <Suspense fallback={<CardSkeleton />}>
          <Card
            title="Total Invoices"
            value={data.numberOfInvoices.toLocaleString()}
            type="invoices"
          />
        </Suspense>
        <Suspense fallback={<CardSkeleton />}>
          <Card
            title="Total Revenue"
            value={data.totalPaidInvoices.toString()}
            type="collected"
          />
        </Suspense>
      </div>

      {/* Charts and tables */}
      <div className="grid gap-6 grid-cols-1 md:grid-cols-2 mb-6">
        <Suspense fallback={<RevenueChartSkeleton />}>
          <RevenueController.displayChart />
        </Suspense>
        <Suspense fallback={<LatestInvoicesSkeleton />}>
          <InvoiceController.latestInvoices />
        </Suspense>
      </div>
    </main>
  );
}
