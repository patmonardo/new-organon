"use client";

import { ReactNode } from "react";
import Link from "next/link";
import { PlusIcon } from "@heroicons/react/24/outline";
import { TableShape, TableColumn } from "@/model/schema/table";
import { SearchRenderer } from "../search/search";
import { PaginationRenderer } from "../list/pagination";

/**
 * TableRenderer - renders a TableShape to React table element
 */
export function TableRenderer({
  shape,
  data,
  renderCell,
  renderActions,
}: {
  shape: TableShape;
  data: Record<string, unknown>[];
  renderCell?: (column: TableColumn, item: Record<string, unknown>) => ReactNode;
  renderActions?: (item: Record<string, unknown>, actions: any[]) => ReactNode;
}): ReactNode {
  const layout = shape.layout || {};
  const title = layout.title || "Table";
  const state = shape.state || { status: "idle", page: 1, totalPages: 1 };
  const columns = shape.columns || [];
  const actions = shape.actions || [];

  // Default cell renderer
  const defaultRenderCell = (column: TableColumn, item: any): ReactNode => {
    const value = item[column.key];
    return value !== undefined && value !== null ? String(value) : "";
  };

  const cellRenderer = renderCell || defaultRenderCell;

  return (
    <div className="flex flex-col w-full">
      {/* Header section */}
      <div className="flex w-full items-center justify-between">
        <h1 className="text-2xl">{title}</h1>
        {layout.addButton && (
          <Link
            href={layout.addButton.href || "#"}
            className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
          >
            <span>{layout.addButton.label || "Add New"}</span>
            <PlusIcon className="h-5 w-5" />
          </Link>
        )}
      </div>

      {/* Search */}
      {layout.searchable && (
        <div className="mt-4 mb-8">
          <SearchRenderer placeholder={`Search ${title.toLowerCase()}...`} />
        </div>
      )}

      {/* Table */}
      {data.length === 0 ? (
        <div className="mt-6 text-center py-10 bg-gray-50 rounded-md">
          <p className="text-gray-500">No data available</p>
        </div>
      ) : (
        <div className="mt-6 flow-root">
          <div className="overflow-x-auto">
            <table
              className={`min-w-full divide-y divide-gray-300 ${
                layout.striped ? "table-striped" : ""
              }`}
            >
              {/* Table Header */}
              <thead>
                <tr>
                  {columns.map((column: TableColumn, i: number) => (
                    <th
                      key={i}
                      className={`px-3 py-3.5 text-sm font-semibold text-gray-900 ${
                        column.className || "text-left"
                      }`}
                    >
                      {column.label}
                    </th>
                  ))}
                  {actions.length > 0 && (
                    <th className="relative py-3.5 pl-3 pr-4 sm:pr-6">
                      <span className="sr-only">Actions</span>
                    </th>
                  )}
                </tr>
              </thead>

              {/* Table Body */}
              <tbody className="divide-y divide-gray-200 bg-white">
                {data.map((item: Record<string, unknown>, rowIndex: number) => (
                  <tr
                    key={rowIndex}
                    className={layout.hoverable ? "hover:bg-gray-50" : ""}
                  >
                    {columns.map((column: TableColumn, colIndex: number) => (
                      <td
                        key={colIndex}
                        className="whitespace-nowrap px-3 py-4 text-sm text-gray-500"
                      >
                        {cellRenderer(column, item)}
                      </td>
                    ))}

                    {actions.length > 0 && renderActions && (
                      <td className="whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                        <div className="flex justify-end gap-3">
                          {renderActions(item, actions)}
                        </div>
                      </td>
                    )}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* Pagination */}
      {layout.paginated && state.totalPages > 0 && (
        <div className="mt-5 flex w-full justify-center">
          <PaginationRenderer totalPages={state.totalPages} />
        </div>
      )}
    </div>
  );
}

