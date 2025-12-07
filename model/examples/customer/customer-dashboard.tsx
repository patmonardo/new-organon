/**
 * CustomerDashboard - Working Radix Dashboard for Customer Profile
 *
 * This is the working Customer Dashboard that demonstrates:
 * - Radix primitives for dashboard UI
 * - Semantic data hydration from Polars
 * - Metrics, tables, profile headers
 */

import React from 'react';
import type { Customer, Invoice } from './customer';
import type { HydratorSnapshot } from '../../src/data/semantic-hydrator';
import {
  RadixPage,
  RadixPageHeader,
  RadixGrid,
  RadixStack,
  RadixCard,
  RadixCardHeader,
  RadixCardTitle,
  RadixCardContent,
  RadixMetricCard,
  RadixProfileHeader,
  RadixTable,
  RadixStatusBadge,
  RadixButton,
  RadixEmptyState,
  RadixBadge,
} from '../../src/sdsl/radix-primitives';

// =============================================================================
// TYPES
// =============================================================================

export interface CustomerDashboardProps {
  /** Customer data (from semantic hydration) */
  customer: Customer;
  /** Invoice list */
  invoices: Invoice[];
  /** Computed metrics */
  metrics: {
    invoiceCount: number;
    totalRevenue: number;
    averageInvoice: number;
  };
  /** Optional: raw hydrator snapshot for debugging */
  snapshot?: HydratorSnapshot;
  /** Action handlers */
  onEdit?: () => void;
  onDelete?: () => void;
  onViewInvoice?: (invoice: Invoice) => void;
  onCreateInvoice?: () => void;
}

// =============================================================================
// HELPERS
// =============================================================================

function formatCurrency(cents: number): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
  }).format(cents / 100);
}

function formatDate(isoDate: string): string {
  return new Date(isoDate).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
}

// =============================================================================
// METRIC ICONS (SVG)
// =============================================================================

const InvoiceIcon = () => (
  <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
    <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
  </svg>
);

const RevenueIcon = () => (
  <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
    <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v12m-3-2.818l.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
  </svg>
);

const AverageIcon = () => (
  <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
    <path strokeLinecap="round" strokeLinejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" />
  </svg>
);

// =============================================================================
// MAIN COMPONENT
// =============================================================================

export function CustomerDashboard({
  customer,
  invoices,
  metrics,
  snapshot,
  onEdit,
  onDelete,
  onViewInvoice,
  onCreateInvoice,
}: CustomerDashboardProps) {
  // Table columns for invoices
  const invoiceColumns = [
    { id: 'id', label: 'Invoice' },
    { id: 'amount', label: 'Amount' },
    { id: 'status', label: 'Status' },
    { id: 'date', label: 'Date' },
  ];

  // Format invoice rows for table
  const invoiceRows = invoices.map(inv => ({
    id: inv.id,
    amount: formatCurrency(inv.amount),
    status: <RadixStatusBadge status={inv.status} />,
    date: formatDate(inv.date),
  }));

  return (
    <RadixPage>
      {/* Page Header */}
      <RadixPageHeader
        title="Customer Dashboard"
        subtitle="View and manage customer profile and invoices"
      />

      <RadixStack gap={6}>
        {/* Metrics Row */}
        <RadixGrid columns={3}>
          <RadixMetricCard
            label="Total Invoices"
            value={metrics.invoiceCount}
            icon={<InvoiceIcon />}
          />
          <RadixMetricCard
            label="Total Revenue"
            value={formatCurrency(metrics.totalRevenue)}
            icon={<RevenueIcon />}
            trend="up"
            delta={12}
          />
          <RadixMetricCard
            label="Average Invoice"
            value={formatCurrency(metrics.averageInvoice)}
            icon={<AverageIcon />}
          />
        </RadixGrid>

        {/* Profile Header */}
        <RadixProfileHeader
          name={customer.name}
          email={customer.email}
          imageUrl={customer.imageUrl}
          badge={customer.region?.toUpperCase()}
          badgeColor="blue"
        >
          <div className="mt-4 flex gap-2">
            {onEdit && (
              <RadixButton variant="secondary" onClick={onEdit}>
                Edit Profile
              </RadixButton>
            )}
            {onDelete && (
              <RadixButton variant="ghost" onClick={onDelete}>
                Delete
              </RadixButton>
            )}
          </div>
        </RadixProfileHeader>

        {/* Invoices Table */}
        <RadixCard>
          <RadixCardHeader>
            <RadixCardTitle>Recent Invoices</RadixCardTitle>
            {onCreateInvoice && (
              <RadixButton onClick={onCreateInvoice}>
                New Invoice
              </RadixButton>
            )}
          </RadixCardHeader>
          <RadixCardContent>
            {invoices.length > 0 ? (
              <RadixTable
                columns={invoiceColumns}
                rows={invoiceRows}
              />
            ) : (
              <RadixEmptyState
                title="No invoices yet"
                description="Create your first invoice to get started"
                action={
                  onCreateInvoice && (
                    <RadixButton onClick={onCreateInvoice}>
                      Create Invoice
                    </RadixButton>
                  )
                }
              />
            )}
          </RadixCardContent>
        </RadixCard>

        {/* Debug Panel (optional) */}
        {snapshot && process.env.NODE_ENV === 'development' && (
          <RadixCard className="bg-slate-900 text-slate-100">
            <RadixCardHeader>
              <RadixCardTitle className="text-slate-100">Semantic Plan (Debug)</RadixCardTitle>
              <RadixBadge color="slate">DEV</RadixBadge>
            </RadixCardHeader>
            <RadixCardContent>
              <pre className="overflow-auto text-xs">
                {snapshot.plan}
              </pre>
            </RadixCardContent>
          </RadixCard>
        )}
      </RadixStack>
    </RadixPage>
  );
}

// =============================================================================
// STATIC FACTORY (for use without controller)
// =============================================================================

export function createCustomerDashboard(props: CustomerDashboardProps): React.ReactElement {
  return <CustomerDashboard {...props} />;
}

