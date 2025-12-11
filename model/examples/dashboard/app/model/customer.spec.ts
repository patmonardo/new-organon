import { describe, it, expect } from 'vitest';
import { CustomerModel } from './customer';
import { CreateCustomer, UpdateCustomer } from '../data/schema/customer';
import { prisma } from '../data/client';
import { v4 as uuidv4 } from 'uuid';

describe('CustomerModel (Database)', () => {
    it('should create and delete multiple customers', async () => {
        const createdCustomerIds: string[] = [];

        try {
            // Create multiple customers
            for (let i = 0; i < 3; i++) {
                const createCustomerData: CreateCustomer = {
                    name: `John Doe ${i}`,
                    email: `john.doe${i}@example.com`,
                    imageUrl: 'https://example.com/image.jpg',
                };

                const result = await CustomerModel.create(createCustomerData);

                expect(result.status).toBe('success');
                expect(result.data).toBeDefined();
                expect(result.data?.name).toBe(createCustomerData.name);

                createdCustomerIds.push(result.data!.id);
            }
        } finally {
            // Clean up all created customers
            await Promise.all(
                createdCustomerIds.map(async (customerId) => {
                    try {
                        await prisma.customer.delete({ where: { id: customerId } });
                    } catch (error) {
                        console.error(`Failed to delete customer ${customerId}:`, error);
                    }
                })
            );
        }
    });

    it('should find a customer by id', async () => {
        const createCustomerData: CreateCustomer = {
            name: 'John Doe',
            email: 'john.doe@example.com',
            imageUrl: 'https://example.com/image.jpg',
        };

        let customerId: string | undefined;
        try {
            const createResult = await CustomerModel.create(createCustomerData);
            expect(createResult.status).toBe('success');
            expect(createResult.data).toBeDefined();

            customerId = createResult.data?.id;

            const foundCustomer = await CustomerModel.findById(customerId!);

            expect(foundCustomer?.status).toBe('success');
            expect(foundCustomer?.data).toBeDefined();
            expect(foundCustomer?.data?.name).toBe(createCustomerData.name);
            expect(foundCustomer?.data?.email).toBe(createCustomerData.email);
            expect(foundCustomer?.data?.imageUrl).toBe(createCustomerData.imageUrl);

        } finally {
            if (customerId) {
                await prisma.customer.delete({ where: { id: customerId } });
            }
        }
    });

    it('should update a customer', async () => {
        const createCustomerData: CreateCustomer = {
            name: 'Original Name',
            email: 'original.email@example.com',
            imageUrl: 'https://example.com/original-image.jpg',
        };

        let customerId: string | undefined;
        try {
            const createResult = await CustomerModel.create(createCustomerData);
            expect(createResult.status).toBe('success');
            expect(createResult.data).toBeDefined();

            customerId = createResult.data?.id;

            const updateCustomerData: UpdateCustomer = {
                name: 'Updated Name',
                email: 'updated.email@example.com',
                imageUrl: 'https://example.com/updated-image.jpg',
            };

            const updateResult = await CustomerModel.update(customerId!, updateCustomerData);

            expect(updateResult?.status).toBe('success');
            expect(updateResult?.data).toBeDefined();
            expect(updateResult?.data?.name).toBe(updateCustomerData.name);
            expect(updateResult?.data?.email).toBe(updateCustomerData.email);
            expect(updateResult?.data?.imageUrl).toBe(updateCustomerData.imageUrl);

        } finally {
            if (customerId) {
                await prisma.customer.delete({ where: { id: customerId } });
            }
        }
    });

});
