//@/(controller)/invoices/status.tsx
import clsx from 'clsx';

export default function InvoiceStatus({ status }: { status: string }) {
  return (
    <span
      className={clsx(
        'inline-flex items-center rounded-full px-2 py-1 text-xs font-medium',
        {
          'bg-green-100 text-green-700': status === 'PAID',
          'bg-yellow-100 text-yellow-700': status === 'PENDING',
          'bg-gray-100 text-gray-700': status === 'DRAFT',
        },
      )}
    >
      {status.charAt(0).toUpperCase() + status.slice(1)}
    </span>
  );
}
