import React from 'react';
import { SemanticResult } from '../../../execution/semantic-hydrator'; // Use semantic result

interface MalloyDataGridProps {
  result: SemanticResult | null;
  className?: string;
  dense?: boolean;
}

export const MalloyDataGrid: React.FC<MalloyDataGridProps> = ({ result, className = '', dense = true }) => {
  if (!result) {
    return (
      <div className={`malloy-data-grid empty p-4 text-center text-gray-400 bg-gray-50 rounded-lg ${className}`}>
        No data collected
      </div>
    );
  }

  const { rows } = result;

  if (rows.length === 0) {
    return (
      <div className={`malloy-data-grid empty p-4 text-center text-gray-500 bg-white border border-gray-200 rounded-lg ${className}`}>
        Query returned 0 rows
      </div>
    );
  }

  const columns = Object.keys(rows[0]);

  return (
    <div className={`malloy-data-grid overflow-x-auto border border-gray-200 rounded-lg shadow-sm ${className}`}>
      <table className="min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          <tr>
            {columns.map((col) => (
              <th
                key={col}
                scope="col"
                className={`
                  text-left text-xs font-medium text-gray-500 uppercase tracking-wider
                  ${dense ? 'px-3 py-2' : 'px-6 py-3'}
                `}
              >
                {col}
              </th>
            ))}
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {rows.map((row, idx) => (
            <tr key={idx} className={idx % 2 === 0 ? 'bg-white' : 'bg-gray-50'}>
              {columns.map((col) => {
                const val = row[col];
                const displayVal = typeof val === 'object' && val !== null ? JSON.stringify(val) : String(val);
                return (
                  <td
                    key={col}
                    className={`
                      text-sm text-gray-900 whitespace-nowrap
                      ${dense ? 'px-3 py-1' : 'px-6 py-4'}
                    `}
                  >
                    {displayVal}
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
      <div className="footer bg-gray-50 px-4 py-2 border-t border-gray-200 text-xs text-gray-500 flex justify-between">
        <span>{rows.length} rows</span>
        {result.meta && (
            <span>
                Time: {result.meta?.executionTime ? `${result.meta.executionTime}ms` : 'N/A'} | Bytes: {result.meta?.arrowBytes ? String(result.meta.arrowBytes) : '0'}
            </span>
        )}
      </div>
    </div>
  );
};
