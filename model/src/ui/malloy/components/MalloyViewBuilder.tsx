import React from 'react';
import { DataModel, ViewQuery } from '../../../data/sdsl';
import { SemanticResult } from '../../../execution/semantic-hydrator';

interface MalloyViewBuilderProps {
  source: DataModel;
  query: ViewQuery;
  onQueryChange?: (query: ViewQuery) => void;
  result?: SemanticResult | null;
}

export const MalloyViewBuilder: React.FC<MalloyViewBuilderProps> = ({ source, query, result }) => {
  return (
    <div className="malloy-view p-4 border rounded-lg bg-white shadow-sm">
      <div className="header mb-4 flex justify-between items-center">
        <h3 className="text-lg font-bold text-gray-900">View Definition</h3>
        <span className="text-sm text-gray-500">Source: {source.config.name}</span>
      </div>

      <div className="query-structure space-y-4">
        {/* Group By Section */}
        {query.group_by && (
          <div className="section group-by">
            <h4 className="text-xs font-uppercase text-gray-500 font-semibold tracking-wider">GROUP BY</h4>
            <div className="flex flex-wrap gap-2 mt-1">
              {query.group_by.map((field) => (
                <span key={field} className="px-2 py-1 bg-blue-100 text-blue-800 rounded text-sm font-medium">
                  {field}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Aggregate Section */}
        {query.aggregate && (
          <div className="section aggregate">
            <h4 className="text-xs font-uppercase text-gray-500 font-semibold tracking-wider">AGGREGATE</h4>
            <div className="flex flex-wrap gap-2 mt-1">
              {query.aggregate.map((field) => (
                <span key={field} className="px-2 py-1 bg-green-100 text-green-800 rounded text-sm font-medium">
                  {field}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Filters Section */}
        {query.where && (
          <div className="section where">
            <h4 className="text-xs font-uppercase text-gray-500 font-semibold tracking-wider">WHERE</h4>
            <div className="space-y-1 mt-1">
              {query.where.map((filter, idx) => (
                <div key={idx} className="flex items-center text-sm text-gray-700">
                  <span className="font-medium mr-2">{filter.field}</span>
                  <span className="text-gray-500 mr-2">{filter.operator}</span>
                  <span className="font-mono bg-gray-100 px-1 rounded">{String(filter.value)}</span>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Visual Separation for Plan */}
      <div className="mt-6 pt-4 border-t border-gray-100">
        <h4 className="text-xs font-uppercase text-gray-400 font-semibold tracking-wider mb-2">EXECUTION PLAN</h4>
        <pre className="text-xs bg-gray-900 text-green-400 p-3 rounded font-mono overflow-x-auto whitespace-pre-wrap">
          {result ? result.plan : source.view(query).toPlan()}
        </pre>
      </div>
    </div>
  );
};
