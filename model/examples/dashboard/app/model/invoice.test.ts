import { describe, it, expect } from 'vitest';
import { InvoiceModel } from './invoice';
import { CreateInvoice, UpdateInvoice } from '../data/schema/invoice';
import { CustomerModel } from './customer';
import { CreateCustomer } from '../data/schema/customer';
import { prisma } from '../data/client';
import { v4 as uuidv4 } from 'uuid';

describe('InvoiceModel (Database)', () => {
    it('should create and delete an invoice', async () => {
        // First, create a customer to associate with the invoice
        const createCustomerData: CreateCustomer = {
            name: 'John Doe',
            email: 'john.doe@example.com',
            imageUrl: '/icons/favicon.ico',
        };

        let customerId: string | undefined;
        let invoiceId: string | undefined;

        try {
            const customerResult = await CustomerModel.create(createCustomerData);
            expect(customerResult.status).toBe('success');
            expect(customerResult.data).toBeDefined();
            customerId = customerResult.data?.id;

            // Now create the invoice
            const createInvoiceData: CreateInvoice = {
                customerId: customerId!,
                amount: 100,
                status: 'PENDING',
                date: new Date(),
            };

            const invoiceResult = await InvoiceModel.create(createInvoiceData);
            expect(invoiceResult.status).toBe('success');
            expect(invoiceResult.data).toBeDefined();
            expect(invoiceResult.data?.amount).toBe(createInvoiceData.amount);

            invoiceId = invoiceResult.data?.id;

        } finally {
            // Clean up: delete the invoice first, then the customer
            if (invoiceId) {
                await prisma.invoice.delete({ where: { id: invoiceId } });
            }
            if (customerId) {
                await prisma.customer.delete({ where: { id: customerId } });
            }
        }
    });

    it('should find an invoice by id', async () => {
        // First, create a customer
        const createCustomerData: CreateCustomer = {
            name: 'Jane Smith',
            email: 'jane.smith@example.com',
            imageUrl: 'https://example.com/jane.jpg',
        };

        let customerId: string | undefined;
        let invoiceId: string | undefined;

        try {
            const customerResult = await CustomerModel.create(createCustomerData);
            expect(customerResult.status).toBe('success');
            expect(customerResult.data).toBeDefined();
            customerId = customerResult.data?.id;

            // Create an invoice
            const createInvoiceData: CreateInvoice = {
                customerId: customerId!,
                amount: 200,
                status: 'PAID',
                date: new Date(),
            };

            const invoiceResult = await InvoiceModel.create(createInvoiceData);
            expect(invoiceResult.status).toBe('success');
            expect(invoiceResult.data).toBeDefined();
            invoiceId = invoiceResult.data?.id;

            // Find the invoice
            const foundInvoiceResult = await InvoiceModel.findById(invoiceId!);
            expect(foundInvoiceResult?.status).toBe('success');
            expect(foundInvoiceResult?.data).toBeDefined();
            expect(foundInvoiceResult?.data?.amount).toBe(createInvoiceData.amount);
            expect(foundInvoiceResult?.data?.customerId).toBe(customerId);

        } finally {
            // Clean up
            if (invoiceId) {
                await prisma.invoice.delete({ where: { id: invoiceId } });
            }
            if (customerId) {
                await prisma.customer.delete({ where: { id: customerId } });
            }
        }
    });

    it('should update an invoice', async () => {
        // Create a customer
        const createCustomerData: CreateCustomer = {
            name: 'Alice Johnson',
            email: 'alice.johnson@example.com',
            imageUrl: 'https://example.com/alice.jpg',
        };

        let customerId: string | undefined;
        let invoiceId: string | undefined;

        try {
            const customerResult = await CustomerModel.create(createCustomerData);
            expect(customerResult.status).toBe('success');
            expect(customerResult.data).toBeDefined();
            customerId = customerResult.data?.id;

            // Create an invoice
            const createInvoiceData: CreateInvoice = {
                customerId: customerId!,
                amount: 300,
                status: 'OVERDUE',
                date: new Date(),
            };

            const invoiceResult = await InvoiceModel.create(createInvoiceData);
            expect(invoiceResult.status).toBe('success');
            expect(invoiceResult.data).toBeDefined();
            invoiceId = invoiceResult.data?.id;

            // Update the invoice
            const updateInvoiceData: UpdateInvoice = {
                amount: 400,
                status: 'PAID',
            };

            const updateResult = await InvoiceModel.update(invoiceId!, updateInvoiceData);
            expect(updateResult?.status).toBe('success');
            expect(updateResult?.data).toBeDefined();
            expect(updateResult?.data?.amount).toBe(updateInvoiceData.amount);
            expect(updateResult?.data?.status).toBe(updateInvoiceData.status);

        } finally {
            // Clean up
            if (invoiceId) {
                await prisma.invoice.delete({ where: { id: invoiceId } });
            }
            if (customerId) {
                await prisma.customer.delete({ where: { id: customerId } });
            }
        }
    });

    it('should delete an invoice', async () => {
      // Create a customer
      const createCustomerData: CreateCustomer = {
          name: 'Delete Test Customer',
          email: 'delete.test@example.com',
          imageUrl: 'https://example.com/delete.jpg',
      };

      let customerId: string | undefined;
      let invoiceId: string | undefined;

      try {
          const customerResult = await CustomerModel.create(createCustomerData);
          expect(customerResult.status).toBe('success');
          expect(customerResult.data).toBeDefined();
          customerId = customerResult.data?.id;

          // Create an invoice
          const createInvoiceData: CreateInvoice = {
              customerId: customerId!,
              amount: 500,
              status: 'PENDING',
              date: new Date(),
          };

          const invoiceResult = await InvoiceModel.create(createInvoiceData);
          expect(invoiceResult.status).toBe('success');
          expect(invoiceResult.data).toBeDefined();
          invoiceId = invoiceResult.data?.id;

          // Delete the invoice
          const deleteResult = await InvoiceModel.delete(invoiceId!);
          expect(deleteResult.status).toBe('success');

          // Verify that the invoice is deleted
          const foundInvoiceResult = await InvoiceModel.findById(invoiceId!);
          expect(foundInvoiceResult?.status).toBe('error'); // Expect an error status
          expect(foundInvoiceResult?.data).toBeNull(); // Expect no data

      } finally {
          // Clean up (customer only, as invoice should be deleted)
          if (customerId) {
              await prisma.customer.delete({ where: { id: customerId } });
          }
      }
  });
});
