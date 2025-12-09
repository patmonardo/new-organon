import React from 'react';
import { defineModel, sum, count, dimension, DataModel } from '../../data/sdsl';
import { MalloyModelVisualizer } from './components/MalloyModelVisualizer';
import { MalloyViewBuilder } from './components/MalloyViewBuilder';
import { useMalloyQuery } from '../react/sdsl/use-malloy-query';
import { PolarsExecutionEngine, PolarsDataset } from '../../execution/polars-engine';
import { PolarsDataService } from '../../execution/polars-data-service';
import { MalloyDataGrid } from './components/MalloyDataGrid';

// Define a sample model
const salesModel = defineModel({
  name: 'ecommerce_sales',
  source: 'bigquery.table.sales',
  dimensions: {
    order_date: dimension('order_date', 'day'),
    product_category: dimension('product_category'),
    customer_region: dimension('customer_region'),
    status: dimension('status'),
  },
  measures: {
    total_revenue: sum('sale_amount'),
    order_count: count(),
    avg_order_value: { type: 'avg', field: 'sale_amount', label: 'AOV' },
  },
});

// Define mock data
const mockData: PolarsDataset = {
  customers: [
    { id: '1', name: 'Alice', email: 'alice@example.com', region: 'West', createdAt: '2023-01-01' },
    { id: '2', name: 'Bob', email: 'bob@example.com', region: 'East', createdAt: '2023-01-02' },
    { id: '3', name: 'Charlie', email: 'charlie@example.com', region: 'West', createdAt: '2023-01-03' },
    { id: '4', name: 'David', email: 'david@example.com', region: 'East', createdAt: '2023-01-04' },
  ],
  invoices: [
    { id: '101', customerId: '1', amount: 100, status: 'paid', date: '2023-01-10' },
    { id: '102', customerId: '1', amount: 200, status: 'pending', date: '2023-01-15' },
    { id: '103', customerId: '2', amount: 150, status: 'paid', date: '2023-01-12' },
    { id: '104', customerId: '3', amount: 300, status: 'paid', date: '2023-01-20' },
  ],
};

// Create engine instance
const engine = new PolarsExecutionEngine(mockData);
const dataService = new PolarsDataService(engine);

export const MalloySandbox: React.FC = () => {
  // Use the hook to manage query state, passing the service
  const { query, updateQuery, reset, runQuery, result, isExecuting, error } = useMalloyQuery(salesModel, {
    group_by: ['product_category'],
    aggregate: ['total_revenue', 'order_count'],
    limit: 10,
  }, { service: dataService });

  return (
    <div className="p-8 max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-8 font-sans">

      {/* Left Column: Model Explorer */}
      <div className="model-explorer space-y-8">
        <div>
          <h2 className="text-2xl font-bold mb-4 text-gray-800">1. Explore Model</h2>
          <MalloyModelVisualizer model={salesModel} />
        </div>

        <div className="p-4 bg-blue-50 rounded-lg border border-blue-100">
          <h3 className="text-sm font-bold text-blue-900 mb-2">QUICK ACTIONS</h3>
          <div className="flex flex-wrap gap-2">
            <button
              onClick={() => updateQuery({ group_by: ['customer_region'], aggregate: ['total_revenue'] })}
              className="px-3 py-1 bg-white border border-blue-200 text-blue-700 rounded-md text-sm hover:bg-blue-50"
            >
              Analyze by Region
            </button>
            <button
              onClick={() => updateQuery({ group_by: ['status'], aggregate: ['count'] })}
              className="px-3 py-1 bg-white border border-blue-200 text-blue-700 rounded-md text-sm hover:bg-blue-50"
            >
              Analyze by Status
            </button>
            <button
              onClick={reset}
              className="px-3 py-1 bg-gray-100 border border-gray-200 text-gray-700 rounded-md text-sm hover:bg-gray-200"
            >
              Reset
            </button>
          </div>
        </div>
      </div>

      {/* Right Column: Query & View */}
      <div className="view-builder space-y-8">
        <div>
          <h2 className="text-2xl font-bold mb-4 text-gray-800">2. Build View</h2>
          <MalloyViewBuilder source={salesModel} query={query} result={result} />
        </div>

        {/* Execution Area */}
        <div className="execution-area">
          <div className="flex justify-between items-center mb-4">
             <h3 className="text-lg font-bold text-gray-800">3. Results</h3>
             <button
               onClick={runQuery}
               disabled={isExecuting}
               className={`
                 px-4 py-2 rounded-md font-bold text-white shadow-sm transition-colors
                 ${isExecuting ? 'bg-gray-400 cursor-wait' : 'bg-green-600 hover:bg-green-700'}
               `}
             >
               {isExecuting ? 'Running...' : 'Run Query'}
             </button>
          </div>

          {error && (
            <div className="p-4 mb-4 bg-red-50 text-red-700 border border-red-200 rounded-md">
              Error: {error.message}
            </div>
          )}

          <MalloyDataGrid result={result} dense={true} />
        </div>
      </div>

    </div>
  );
};
