import { LatestInvoiceDisplay } from '@graphics/schema/invoice';
import { ContainerCard } from '@/graphics/card/card';

interface LatestInvoicesCardProps {
  invoices: LatestInvoiceDisplay[];
  title?: string;
}

export function LatestInvoicesCard({
  invoices,
  title = "Latest Invoices"
}: LatestInvoicesCardProps) {
  return (
    <ContainerCard className="h-full">
      <h3 className="font-medium text-gray-900 mb-4">{title}</h3>
      <div className="flex flex-col space-y-4">
        {invoices.map((invoice) => (
          <div
            key={invoice.id}
            className="flex items-center justify-between rounded-lg bg-gray-50 p-3"
          >
            <div className="flex items-center">
              <div>
                <p className="text-sm font-medium">{invoice.customer.name}</p>
                <p className="text-xs text-gray-500">{invoice.customer.email}</p>
              </div>
            </div>
            <div className="flex flex-col items-end">
              <p className="text-sm font-medium">{invoice.formattedAmount}</p>
              <div className="flex items-center">
                <p className="text-xs text-gray-500">{invoice.formattedDate}</p>
                <div className={`ml-2 h-2 w-2 rounded-full ${invoice.statusColor}`} />
              </div>
            </div>
          </div>
        ))}
      </div>
    </ContainerCard>
  );
}
