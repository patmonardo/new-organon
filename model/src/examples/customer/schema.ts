/**
 * Customer Schema - MVC SDSL Design Template
 * 
 * This defines the Customer FormShape - the Property schema for Customer forms.
 * Property.schema() → Codegen (this file)
 * Property.values() → Eval (CustomerModel)
 */

import { z } from "zod";
import type { FormShape, FormField } from "../../sdsl/types";

// =============================================================================
// CUSTOMER DATA SCHEMA (What gets stored)
// =============================================================================

export const CustomerDataSchema = z.object({
  id: z.string().optional(),
  name: z.string().min(1, { error: "Name is required" }),
  email: z.string().email({ error: "Invalid email address" }),
  imageUrl: z.string().url().optional().nullable(),
  createdAt: z.date().optional(),
  updatedAt: z.date().optional(),
});

export type CustomerData = z.infer<typeof CustomerDataSchema>;

// =============================================================================
// CUSTOMER FORM FIELDS (FormShape.fields)
// =============================================================================

export const customerFields: FormField[] = [
  {
    id: "name",
    type: "text",
    label: "Name",
    placeholder: "Enter customer name",
    required: true,
    disabled: false,
    validation: {
      required: true,
      minLength: 1,
      message: "Name is required",
    },
  },
  {
    id: "email",
    type: "email",
    label: "Email",
    placeholder: "Enter email address",
    required: true,
    disabled: false,
    validation: {
      required: true,
      pattern: "^[^@]+@[^@]+\\.[^@]+$",
      message: "Valid email is required",
    },
  },
  {
    id: "imageUrl",
    type: "url",
    label: "Profile Image URL",
    placeholder: "https://example.com/image.png",
    required: false,
    disabled: false,
  },
];

// =============================================================================
// CUSTOMER FORM SHAPE (The complete Property schema)
// =============================================================================

export const CustomerFormShape: FormShape = {
  id: "customer-form",
  name: "customer",
  title: "Customer",
  description: "Customer information form",
  fields: customerFields,
  meta: {
    sections: [
      {
        id: "basic-info",
        title: "Basic Information",
        fields: ["name", "email"],
      },
      {
        id: "profile",
        title: "Profile",
        fields: ["imageUrl"],
      },
    ],
    actions: [
      {
        id: "submit",
        type: "submit",
        label: "Save Customer",
        primary: true,
      },
      {
        id: "cancel",
        type: "button",
        label: "Cancel",
      },
    ],
  },
};

// =============================================================================
// CREATE/UPDATE SCHEMAS (For validation)
// =============================================================================

export const CreateCustomerSchema = CustomerDataSchema.omit({
  id: true,
  createdAt: true,
  updatedAt: true,
});

export type CreateCustomer = z.infer<typeof CreateCustomerSchema>;

export const UpdateCustomerSchema = CustomerDataSchema.partial().omit({
  id: true,
  createdAt: true,
  updatedAt: true,
});

export type UpdateCustomer = z.infer<typeof UpdateCustomerSchema>;

// =============================================================================
// OPERATION RESULT (Standard response type)
// =============================================================================

export interface OperationResult<T> {
  data: T | null;
  status: "success" | "error";
  message?: string;
}

