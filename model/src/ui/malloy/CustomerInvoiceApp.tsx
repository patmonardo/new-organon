import React, { useState } from 'react';
import { PolarsExecutionEngine } from '../../execution/polars-engine';
import { PolarsDataService } from '../../execution/polars-data-service';
import { Customer360Model, InvoiceModel, ecommerceMockData } from './domain/ecommerce';
import { useMalloyQuery } from '../react/sdsl/use-malloy-query';
import { MalloyDataGrid } from './components/MalloyDataGrid';
import { MalloyViewBuilder } from './components/MalloyViewBuilder';

// =============================================================================
// SERVER SIDE SIMULATION
// =============================================================================

// "Server-side" engine instance (Singleton-ish for this demo)
const serverEngine = new PolarsExecutionEngine(ecommerceMockData);
// "Aquifer" Service Layer
const dataService = new PolarsDataService(serverEngine);

// =============================================================================
// CLIENT APPLICATION
// =============================================================================

export const CustomerInvoiceApp: React.FC = () => {
  // State for "Navigation" (Tabs)
  const [activeTab, setActiveTab] = useState<'customers' | 'invoices'>('customers');

  return (
    <div className="customer-invoice-app min-h-screen bg-gray-100 font-sans text-gray-900">

      {/* App Header */}
      <header className="bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 bg-indigo-600 rounded-lg flex items-center justify-center text-white font-bold">
            M
          </div>
          <h1 className="text-xl font-bold tracking-tight">Malloy Insights</h1>
        </div>
        <div className="text-sm text-gray-500">
          Connected to: <span className="font-mono text-xs bg-gray-100 px-2 py-1 rounded">PolarsEngine (In-Memory)</span>
        </div>
      </header>

      {/* Main Content */}
      <main className="p-6 max-w-7xl mx-auto">

        {/* Navigation Tabs */}
        <div className="tabs mb-6 flex gap-1 bg-white p-1 rounded-lg border border-gray-200 w-fit">
          <button
            onClick={() => setActiveTab('customers')}
            className={`px-4 py-2 text-sm font-medium rounded-md transition-colors ${
              activeTab === 'customers'
                ? 'bg-indigo-50 text-indigo-700'
                : 'text-gray-600 hover:bg-gray-50'
            }`}
          >
            Customer 360
          </button>
          <button
            onClick={() => setActiveTab('invoices')}
            className={`px-4 py-2 text-sm font-medium rounded-md transition-colors ${
              activeTab === 'invoices'
                ? 'bg-indigo-50 text-indigo-700'
                : 'text-gray-600 hover:bg-gray-50'
            }`}
          >
            Invoices Analysis
          </button>
        </div>

        {/* Tab Content */}
        <div className="content">
          {activeTab === 'customers' && <CustomerView />}
          {activeTab === 'invoices' && <InvoiceView />}
        </div>

      </main>
    </div>
  );
};

// =============================================================================
// SUB-VIEWS (PAGES)
// =============================================================================

/**
 * CustomerView - The "Customer 360" Page
 */
const CustomerView: React.FC = () => {
  // Initial "Server" Query state
  const { query, updateQuery, runQuery, result, isExecuting } = useMalloyQuery(Customer360Model, {
    group_by: ['customer_name', 'region'],
    aggregate: ['lifetime_value', 'invoice_count'],
    order_by: [{ field: 'lifetime_value', direction: 'desc' }],
    limit: 10,
  }, { service: dataService });

  // Auto-run on mount (simulating server fetch)
  React.useEffect(() => {
    runQuery();
  }, [runQuery]);

  return (
    <div className="customer-view grid grid-cols-1 lg:grid-cols-3 gap-6">

      {/* Sidebar: Controls */}
      <div className="controls col-span-1 space-y-6">
        <div className="bg-white p-4 rounded-xl border border-gray-200 shadow-sm">
          <h3 className="text-sm font-bold text-gray-900 uppercase tracking-wider mb-4">Analysis Controls</h3>

          <div className="space-y-4">
            <div>
              <label className="block text-xs font-medium text-gray-500 mb-1">Breakdown By</label>
              <div className="flex flex-wrap gap-2">
                 <button
                   onClick={() => updateQuery({ group_by: ['customer_name', 'region'] })}
                   className={`px-3 py-1 text-xs border rounded-full ${query.group_by?.includes('customer_name') ? 'bg-indigo-100 border-indigo-200 text-indigo-800' : 'bg-white border-gray-200 text-gray-600'}`}
                 >
                   Name & Region
                 </button>
                 <button
                   onClick={() => updateQuery({ group_by: ['region'] })}
                   className={`px-3 py-1 text-xs border rounded-full ${!query.group_by?.includes('customer_name') && query.group_by?.includes('region') ? 'bg-indigo-100 border-indigo-200 text-indigo-800' : 'bg-white border-gray-200 text-gray-600'}`}
                 >
                   Region Only
                 </button>
              </div>
            </div>

            <div>
               <label className="block text-xs font-medium text-gray-500 mb-1">Metrics</label>
               <div className="flex flex-wrap gap-2">
                 <button
                   onClick={() => updateQuery({ aggregate: ['lifetime_value', 'invoice_count'] })}
                   className="px-3 py-1 text-xs border border-gray-200 rounded-full hover:bg-gray-50"
                 >
                   LTV & Count
                 </button>
                 <button
                   onClick={() => updateQuery({ aggregate: ['avg_invoice'] })}
                   className="px-3 py-1 text-xs border border-gray-200 rounded-full hover:bg-gray-50"
                 >
                   Avg Order Value
                 </button>
               </div>
            </div>
          </div>

          <div className="mt-6 pt-4 border-t border-gray-100">
             <button
               onClick={runQuery}
               className="w-full py-2 bg-indigo-600 text-white rounded-lg text-sm font-bold shadow-sm hover:bg-indigo-700 transition-colors"
             >
               {isExecuting ? 'Refreshing...' : 'Update Analysis'}
             </button>
          </div>
        </div>

        {/* View Builder Preview */}
         <MalloyViewBuilder source={Customer360Model} query={query} result={result} />
      </div>

      {/* Main: Data Grid */}
      <div className="results col-span-1 lg:col-span-2 space-y-4">
        <h2 className="text-lg font-bold text-gray-900">Top Customers</h2>
        <MalloyDataGrid result={result} />
      </div>

    </div>
  );
};

/**
 * InvoiceView - The "Invoices Analysis" Page
 */
const InvoiceView: React.FC = () => {
    const { query, runQuery, result } = useMalloyQuery(InvoiceModel, {
      group_by: ['status'],
      aggregate: ['total_amount', 'count'],
    }, { service: dataService });

    React.useEffect(() => { runQuery(); }, [runQuery]);

    return (
      <div className="invoice-view space-y-6">
        <div className="bg-white p-6 rounded-xl border border-gray-200 shadow-sm flex items-center justify-between">
            <div>
                <h2 className="text-lg font-bold text-gray-900">Invoice Status Distribution</h2>
                <p className="text-sm text-gray-500">Breakdown of invoice revenue by status.</p>
            </div>
            <button onClick={runQuery} className="px-4 py-2 bg-white border border-gray-300 rounded-md text-sm font-medium hover:bg-gray-50">
                Refresh
            </button>
        </div>

        <MalloyDataGrid result={result} />

        <div className="grid grid-cols-2 gap-6">
             <MalloyViewBuilder source={InvoiceModel} query={query} result={result} />
             <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg text-yellow-800 text-sm">
                 <strong>Insight:</strong> "Void" invoices are excluded from revenue aggregations in the raw data, but displayed here for audit.
             </div>
        </div>
      </div>
    );
  };
