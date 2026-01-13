import { describe, it, expect, beforeAll } from 'vitest';
import { defaultConnection } from '../../src/repository/neo4j-client';
import { createFormDb } from './createFormDb';
import { FormShapeRepository } from '../../src/repository/form';
import { EntityShapeRepository } from '../../src/repository/entity';
import { ContextRepository } from '../../src/repository/context';
import { PropertyShapeRepository } from '../../src/repository/property';

const neo4jOk = await defaultConnection.verifyConnectivity();

describe.skipIf(!neo4jOk)('Integration Tests with Seeded Data', () => {
  const { shapeEngine, entityEngine, formShapeRepo, entityShapeRepo } =
    createFormDb({
      connection: defaultConnection,
    });

  const formRepo = formShapeRepo;
  const entityRepo = entityShapeRepo;
  const contextRepo = new ContextRepository(defaultConnection);
  const propertyRepo = new PropertyShapeRepository(defaultConnection);

  describe('Forms (Seeded)', () => {
    it('should load invoice-form from seeded data', async () => {
      const form = await formRepo.getFormById('invoice-form');
      expect(form).toBeTruthy();
      expect(form?.id).toBe('invoice-form');
      expect(form?.name).toBe('InvoiceForm');
      expect(form?.title).toBe('Invoice');
      expect(form?.fields).toBeDefined();
      expect(form?.fields?.length).toBeGreaterThan(0);
    });

    it('should load customer-form from seeded data', async () => {
      const form = await formRepo.getFormById('customer-form');
      expect(form).toBeTruthy();
      expect(form?.id).toBe('customer-form');
      expect(form?.name).toBe('CustomerForm');
      expect(form?.fields).toBeDefined();
    });

    it('should load product-form from seeded data', async () => {
      const form = await formRepo.getFormById('product-form');
      expect(form).toBeTruthy();
      expect(form?.id).toBe('product-form');
      expect(form?.name).toBe('ProductForm');
    });

    it('should find all forms via repository', async () => {
      const forms = await formRepo.findForms({});
      expect(forms.length).toBeGreaterThanOrEqual(3);
      const formIds = forms.map((f) => f.id);
      expect(formIds).toContain('invoice-form');
      expect(formIds).toContain('customer-form');
      expect(formIds).toContain('product-form');
    });

    it('should find forms by tags', async () => {
      const billingForms = await formRepo.findForms({ tags: ['billing'] });
      expect(billingForms.length).toBeGreaterThan(0);
      expect(billingForms.some((f) => f.id === 'invoice-form')).toBe(true);
    });
  });

  describe('Entities (Seeded)', () => {
    it('should load customer entities from seeded data', async () => {
      const customer1 = await entityRepo.getEntityById('customer-1');
      expect(customer1).toBeTruthy();
      expect(customer1?.id).toBe('customer-1');
      expect(customer1?.formId).toBe('customer-form');
      expect(customer1?.name).toBe('Acme Corporation');
    });

    it('should load product entities from seeded data', async () => {
      const product1 = await entityRepo.getEntityById('product-1');
      expect(product1).toBeTruthy();
      expect(product1?.id).toBe('product-1');
      expect(product1?.formId).toBe('product-form');
      expect(product1?.name).toBe('Laptop Pro 15');
    });

    it('should load invoice entities from seeded data', async () => {
      const invoice1 = await entityRepo.getEntityById('invoice-1');
      expect(invoice1).toBeTruthy();
      expect(invoice1?.id).toBe('invoice-1');
      expect(invoice1?.formId).toBe('invoice-form');
      expect(invoice1?.status).toBe('PAID');
    });

    it('should find entities by formId', async () => {
      const customers = await entityRepo.findEntities({
        type: 'system.Entity',
      });
      const customerEntities = customers.filter(
        (e) => e.formId === 'customer-form',
      );
      expect(customerEntities.length).toBeGreaterThanOrEqual(3);
    });

    it('should find entities by tags', async () => {
      const customers = await entityRepo.findEntities({ tags: ['customer'] });
      expect(customers.length).toBeGreaterThanOrEqual(3);
    });
  });

  describe('Relationships (Seeded)', () => {
    it('should verify invoice-customer relationships exist', async () => {
      const session = defaultConnection.getSession();
      try {
        const result = await session.run(`
          MATCH (inv:Entity {id: 'invoice-1'})-[r:BELONGS_TO]->(cust:Entity {id: 'customer-1'})
          RETURN count(r) as count
        `);
        const count = result.records[0]?.get('count');
        expect(count?.toNumber?.() ?? count).toBeGreaterThan(0);
      } finally {
        await session.close();
      }
    });

    it('should find all invoices for a customer', async () => {
      const session = defaultConnection.getSession();
      try {
        const result = await session.run(`
          MATCH (cust:Entity {id: 'customer-1'})<-[:BELONGS_TO]-(inv:Entity)
          RETURN inv.id as invoiceId
          ORDER BY invoiceId
        `);
        const invoiceIds = result.records.map((r) => r.get('invoiceId'));
        expect(invoiceIds.length).toBeGreaterThanOrEqual(2);
        expect(invoiceIds).toContain('invoice-1');
        expect(invoiceIds).toContain('invoice-2');
      } finally {
        await session.close();
      }
    });
  });

  describe('Engines with Seeded Data', () => {
    it('should load form via ShapeEngine', async () => {
      const form = await shapeEngine.getShape('invoice-form');
      expect(form).toBeTruthy();
      expect(form?.id).toBe('invoice-form');
    });

    it('should load entity via EntityEngine', async () => {
      const entity = await entityEngine.getEntity('customer-1');
      expect(entity).toBeTruthy();
      expect(entity?.id).toBe('customer-1');
    });

    it('should update entity via EntityEngine', async () => {
      const original = await entityEngine.getEntity('customer-1');
      expect(original).toBeTruthy();

      await entityEngine.handle({
        kind: 'entity.setCore',
        payload: { id: 'customer-1', name: 'Acme Corporation Updated' },
      } as any);

      const updated = await entityEngine.getEntity('customer-1');
      expect(updated?.name).toBe('Acme Corporation Updated');

      // Restore original
      await entityEngine.handle({
        kind: 'entity.setCore',
        payload: { id: 'customer-1', name: 'Acme Corporation' },
      } as any);
    });
  });

  describe('Context & Property (Seeded - Optional)', () => {
    it('should load invoice management context if seeded', async () => {
      const context = await contextRepo.getContextById(
        'context-invoice-management',
      );
      if (context) {
        expect(context.id).toBe('context-invoice-management');
        expect(context.name).toBe('Invoice Management Context');
        expect((context.entities ?? []).length).toBeGreaterThan(0);
      } else {
        console.log('⚠️  Context not seeded - run pnpm seed:reflective first');
      }
    });

    it('should load product catalog context if seeded', async () => {
      const context = await contextRepo.getContextById(
        'context-product-catalog',
      );
      if (context) {
        expect(context.name).toBe('Product Catalog Context');
      }
    });

    it('should load invoice-customer relationship property if seeded', async () => {
      const property = await propertyRepo.getPropertyById(
        'property-invoice-customer-law',
      );
      if (property) {
        expect(property.id).toBe('property-invoice-customer-law');
        expect(property.facets?.law).toBeDefined();
        expect(property.facets?.law?.invariants?.length).toBeGreaterThan(0);
      }
    });

    it('should verify property reflects on context if seeded', async () => {
      const property = await propertyRepo.getPropertyById(
        'property-invoice-customer-law',
      );
      if (property) {
        expect(property.facets?.context?.contextId).toBe(
          'context-invoice-management',
        );
      }
    });
  });

  describe('Full Stack Integration', () => {
    it('should demonstrate Form → Entity flow (always works)', async () => {
      // 1. Load Form (Rational structure)
      const form = await formRepo.getFormById('invoice-form');
      expect(form).toBeTruthy();

      // 2. Load Entity (Empirical instance)
      const entity = await entityRepo.getEntityById('invoice-1');
      expect(entity).toBeTruthy();
      expect(entity?.formId).toBe(form?.id);

      // This demonstrates the horizontal dialectic:
      // Form (Rational) → Entity (Empirical)
    });

    it('should demonstrate Form → Entity → Context → Property flow (if reflective system seeded)', async () => {
      // 1. Load Form (Rational structure)
      const form = await formRepo.getFormById('invoice-form');
      expect(form).toBeTruthy();

      // 2. Load Entity (Empirical instance)
      const entity = await entityRepo.getEntityById('invoice-1');
      expect(entity).toBeTruthy();
      expect(entity?.formId).toBe(form?.id);

      // 3. Load Context (Reflection on Form/Entity) - optional
      const context = await contextRepo.getContextById(
        'context-invoice-management',
      );
      if (context) {
        expect((context.entities ?? []).some((e) => e.id === entity?.id)).toBe(
          true,
        );

        // 4. Load Property (Grounding laws) - optional
        const property = await propertyRepo.getPropertyById(
          'property-invoice-customer-law',
        );
        if (property) {
          expect(property.facets?.context?.contextId).toBe(context.id);

          // This demonstrates the full Organic Unity:
          // Form (Rational) → Entity (Empirical) → Context (Reflection) → Property (Grounding)
        }
      } else {
        console.log(
          '⚠️  Reflective system not seeded - run pnpm seed:reflective to test full flow',
        );
      }
    });
  });
});
