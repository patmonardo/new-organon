/**
 * Customer View - MVC SDSL Design Template
 * 
 * The View handles:
 * - Rendering forms from FormShape
 * - Rendering tables/lists
 * - Producing JSX output
 * 
 * Pattern: View renders FormShape fields directly to JSX
 */

import React from "react";
import type { FormHandler, OperationResult, FormField } from "../../sdsl/types";
import { CustomerFormShape, customerFields, type CustomerData } from "./schema";

// =============================================================================
// FORM MODE
// =============================================================================

export type FormMode = "create" | "edit" | "view";

// =============================================================================
// CUSTOMER VIEW CLASS
// =============================================================================

/**
 * CustomerView - Renders Customer forms and tables
 * 
 * Pattern: View renders FormShape fields to JSX
 */
export class CustomerView {
  private customer?: CustomerData;
  private fields: FormField[];

  constructor(customer?: CustomerData) {
    this.customer = customer;
    this.fields = customerFields;
  }

  // ---------------------------------------------------------------------------
  // Form Display
  // ---------------------------------------------------------------------------

  /**
   * Display the customer form
   * @param mode - create, edit, or view
   * @param format - output format (jsx, html, json)
   * @param handlers - form action handlers
   */
  async display(
    mode: FormMode,
    _format: "jsx" | "html" | "json" = "jsx",
    handlers?: FormHandler
  ): Promise<OperationResult<React.ReactNode>> {
    try {
      const jsx = this.renderFormJsx(mode, handlers);
      return {
        data: jsx,
        status: "success",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: `Failed to render form: ${error}`,
      };
    }
  }

  /**
   * Render form as JSX
   */
  private renderFormJsx(
    mode: FormMode,
    handlers?: FormHandler
  ): React.ReactNode {
    const isReadOnly = mode === "view";
    const title = mode === "create" 
      ? "Create Customer" 
      : mode === "edit" 
        ? "Edit Customer" 
        : "Customer Details";

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
      e.preventDefault();
      if (handlers?.onSubmit) {
        const formData = new FormData(e.currentTarget);
        const data = Object.fromEntries(formData.entries());
        await handlers.onSubmit({ 
          ...CustomerFormShape, 
          ...data 
        } as typeof CustomerFormShape);
      }
    };

    return (
      <div className="max-w-2xl mx-auto p-6">
        {/* Breadcrumbs */}
        <nav className="mb-4 text-sm text-gray-500">
          <a href="/customers" className="hover:underline">Customers</a>
          <span className="mx-2">/</span>
          <span>{title}</span>
        </nav>

        {/* Form */}
        <form onSubmit={handleSubmit}>
          <div className="bg-white shadow rounded-lg p-6">
            <h1 className="text-2xl font-bold mb-6">{title}</h1>

            {/* Render fields */}
            <div className="space-y-4">
              {this.fields.map((field) => (
                <div key={field.id}>
                  {this.renderField(field, isReadOnly)}
                </div>
              ))}
            </div>

            {/* Actions */}
            {!isReadOnly && (
              <div className="flex gap-4 mt-6 pt-6 border-t">
                <button
                  type="submit"
                  className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                >
                  {mode === "create" ? "Create Customer" : "Save Changes"}
                </button>
                
                {handlers?.onCancel && (
                  <button
                    type="button"
                    onClick={handlers.onCancel}
                    className="px-4 py-2 bg-gray-200 text-gray-700 rounded hover:bg-gray-300"
                  >
                    Cancel
                  </button>
                )}
              </div>
            )}
          </div>
        </form>
      </div>
    );
  }

  /**
   * Render a single form field
   */
  private renderField(
    field: FormField,
    isReadOnly: boolean
  ): React.ReactNode {
    const { id, type, label, placeholder, required } = field;
    const value = this.customer?.[id as keyof CustomerData] ?? "";

    const inputClasses = `
      w-full px-3 py-2 border rounded-md
      ${isReadOnly ? "bg-gray-100" : "bg-white"}
      focus:outline-none focus:ring-2 focus:ring-blue-500
    `;

    return (
      <div>
        <label htmlFor={id} className="block text-sm font-medium text-gray-700 mb-1">
          {label || id}
          {required && <span className="text-red-500 ml-1">*</span>}
        </label>
        
        <input
          id={id}
          name={id}
          type={type === "email" || type === "url" ? type : "text"}
          defaultValue={value as string}
          placeholder={placeholder}
          required={required}
          readOnly={isReadOnly}
          className={inputClasses}
        />
      </div>
    );
  }

  // ---------------------------------------------------------------------------
  // Table Display
  // ---------------------------------------------------------------------------

  /**
   * Display customers as a table
   */
  async displayTable(
    customers: CustomerData[],
    totalPages: number = 1
  ): Promise<OperationResult<React.ReactNode>> {
    try {
      const jsx = this.renderTableJsx(customers, totalPages);
      return {
        data: jsx,
        status: "success",
      };
    } catch (error) {
      return {
        data: null,
        status: "error",
        message: `Failed to render table: ${error}`,
      };
    }
  }

  /**
   * Render customers table as JSX
   */
  private renderTableJsx(
    customers: CustomerData[],
    _totalPages: number
  ): React.ReactNode {
    return (
      <div className="bg-white shadow rounded-lg overflow-hidden">
        {/* Header */}
        <div className="px-6 py-4 border-b flex justify-between items-center">
          <h1 className="text-2xl font-bold">Customers</h1>
          <a
            href="/customers/create"
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            Add Customer
          </a>
        </div>

        {/* Table */}
        <table className="w-full">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Name
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Email
              </th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                Actions
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {customers.length === 0 ? (
              <tr>
                <td colSpan={3} className="px-6 py-8 text-center text-gray-500">
                  No customers found
                </td>
              </tr>
            ) : (
              customers.map((customer) => (
                <tr key={customer.id} className="hover:bg-gray-50">
                  <td className="px-6 py-4">
                    <div className="flex items-center">
                      {customer.imageUrl && (
                        <img
                          src={customer.imageUrl}
                          alt={customer.name}
                          className="w-8 h-8 rounded-full mr-3"
                        />
                      )}
                      <span className="font-medium">{customer.name}</span>
                    </div>
                  </td>
                  <td className="px-6 py-4 text-gray-600">
                    {customer.email}
                  </td>
                  <td className="px-6 py-4 text-right">
                    <a
                      href={`/customers/${customer.id}/edit`}
                      className="text-blue-600 hover:underline mr-4"
                    >
                      Edit
                    </a>
                    <a
                      href={`/customers/${customer.id}/delete`}
                      className="text-red-600 hover:underline"
                    >
                      Delete
                    </a>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>

        {/* Pagination placeholder */}
        <div className="px-6 py-4 border-t">
          {/* Pagination component would go here */}
        </div>
      </div>
    );
  }
}

