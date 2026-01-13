import { defaultConnection } from '../neo4j-client';
import { ContextRepository } from '../context';
import { PropertyShapeRepository } from '../property';
import { ContextShapeRepo } from '@schema/context';
import { PropertyShapeRepo } from '@schema/property';
import neo4j from 'neo4j-driver';

/**
 * Helper to convert Neo4j Integer to number
 */
function toNumber(value: any): number {
  if (value === null || value === undefined) return 0;
  if (neo4j.isInt(value)) {
    return value.toNumber();
  }
  return Number(value) || 0;
}

/**
 * Seed Context and Property - Reflective System
 *
 * This demonstrates the vertical dialectic:
 * - Context reflects on Form/Entity (content)
 * - Property reflects on Context (grounds laws/invariants)
 *
 * Building on the Form:Entity base from seed-enhanced
 */
export async function seedContextProperty() {
  console.log('🔮 Starting Context & Property Seed (Reflective System)...');

  const connected = await defaultConnection.verifyConnectivity();
  if (!connected) {
    console.error('❌ Failed to connect to Neo4j. Exiting.');
    process.exit(1);
  }

  const contextRepo = new ContextRepository(defaultConnection);
  const propertyRepo = new PropertyShapeRepository(defaultConnection);

  const session = defaultConnection.getSession();
  try {
    // Check if we have Forms/Entities to reflect upon
    const formCheck = await session.run(
      'MATCH (f:FormShape) RETURN count(f) as count',
    );
    const entityCheck = await session.run(
      'MATCH (e:Entity) RETURN count(e) as count',
    );
    const formCount = toNumber(formCheck.records[0]?.get('count'));
    const entityCount = toNumber(entityCheck.records[0]?.get('count'));

    if (formCount === 0 || entityCount === 0) {
      console.log('⚠️  No Forms/Entities found. Run seed:enhanced first.');
      return;
    }

    console.log(
      `   Found ${formCount} forms and ${entityCount} entities to reflect upon\n`,
    );

    // === 1. Create Contexts (Reflection on Form/Entity) ===
    console.log('📐 Creating Contexts (Reflection on Content)...');

    // Context for Invoice Management
    const invoiceContext: ContextShapeRepo = {
      id: 'context-invoice-management',
      type: 'context.business',
      name: 'Invoice Management Context',
      description: 'Context for managing invoices and billing operations',
      state: {
        status: 'active',
      },
      entities: [
        { id: 'invoice-1', type: 'system.Entity' },
        { id: 'invoice-2', type: 'system.Entity' },
        { id: 'invoice-3', type: 'system.Entity' },
        { id: 'invoice-4', type: 'system.Entity' },
        { id: 'customer-1', type: 'system.Entity' },
        { id: 'customer-2', type: 'system.Entity' },
        { id: 'customer-3', type: 'system.Entity' },
      ],
      relations: [],
      facets: {
        presuppositions: [
          {
            name: 'Invoice Form Exists',
            definition: 'Invoice form structure is available',
            posited: true,
          },
          {
            name: 'Customer Form Exists',
            definition: 'Customer form structure is available',
            posited: true,
          },
        ],
        scope: {
          modal: 'actual',
          domain: ['invoice-form', 'customer-form'],
          phase: 'runtime',
        },
        conditions: [
          {
            id: 'inv-1',
            constraint: 'invoice.customerId must reference valid customer',
            predicate: 'exists(customer)',
          },
          {
            id: 'inv-2',
            constraint: 'invoice.amount > 0',
            predicate: 'amount > 0',
          },
        ],
      },
      createdAt: Date.now(),
      updatedAt: Date.now(),
    };

    await contextRepo.saveContext(invoiceContext);
    console.log(`   ✅ Created Invoice Management Context`);

    // Context for Product Catalog
    const productContext: ContextShapeRepo = {
      id: 'context-product-catalog',
      type: 'context.business',
      name: 'Product Catalog Context',
      description: 'Context for managing product catalog and inventory',
      state: {
        status: 'active',
      },
      entities: [
        { id: 'product-1', type: 'system.Entity' },
        { id: 'product-2', type: 'system.Entity' },
        { id: 'product-3', type: 'system.Entity' },
        { id: 'product-4', type: 'system.Entity' },
      ],
      relations: [],
      facets: {
        presuppositions: [
          {
            name: 'Product Form Exists',
            definition: 'Product form structure is available',
            posited: true,
          },
        ],
        scope: {
          modal: 'actual',
          domain: ['product-form'],
          phase: 'runtime',
        },
        conditions: [
          {
            id: 'prod-1',
            constraint: 'product.sku must be unique',
            predicate: 'unique(sku)',
          },
          {
            id: 'prod-2',
            constraint: 'product.stock >= 0',
            predicate: 'stock >= 0',
          },
        ],
      },
      createdAt: Date.now(),
      updatedAt: Date.now(),
    };

    await contextRepo.saveContext(productContext);
    console.log(`   ✅ Created Product Catalog Context`);

    // === 2. Create Properties (Reflection on Context) ===
    console.log('\n⚖️  Creating Properties (Grounding Laws/Invariants)...');

    // Property: Invoice-Customer Relationship Law
    const invoiceCustomerProperty: PropertyShapeRepo = {
      id: 'property-invoice-customer-law',
      type: 'property.business',
      name: 'Invoice-Customer Relationship Law',
      state: {
        status: 'active',
        contextId: 'context-invoice-management',
      },
      signature: {},
      facets: {
        law: {
          invariants: [
            {
              id: 'inv-1',
              constraint: 'Every invoice must belong to exactly one customer',
              predicate: 'count(invoice.customerId) == 1',
              universality: 'necessary',
            },
            {
              id: 'inv-2',
              constraint: 'Customer must exist before invoice creation',
              predicate:
                'exists(customer) && customer.createdAt < invoice.createdAt',
              universality: 'necessary',
            },
          ],
          universality: 'necessary',
        },
        facticity: {
          grounds: ['invoice-form', 'customer-form'],
          conditions: ['context-invoice-management'],
          evidence: [
            {
              name: 'BELONGS_TO relationship',
              definition:
                'Invoice entities link to Customer entities via BELONGS_TO',
              type: 'relationship',
            },
          ],
        },
        mediates: {
          fromEntities: ['invoice-1', 'invoice-2', 'invoice-3', 'invoice-4'],
          toAspects: [], // Will be populated when Aspects are created
        },
        context: {
          contextId: 'context-invoice-management',
          version: 1,
        },
      },
      meta: { seed: 'context-property' },
      tags: ['business', 'relationship', 'invariant'],
    };

    await propertyRepo.saveProperty(invoiceCustomerProperty);
    console.log(`   ✅ Created Invoice-Customer Relationship Law`);

    // Property: Product Inventory Law
    const productInventoryProperty: PropertyShapeRepo = {
      id: 'property-product-inventory-law',
      type: 'property.business',
      name: 'Product Inventory Law',
      state: {
        status: 'active',
        contextId: 'context-product-catalog',
      },
      signature: {},
      facets: {
        law: {
          invariants: [
            {
              id: 'inv-1',
              constraint: 'Product stock cannot be negative',
              predicate: 'product.stock >= 0',
              universality: 'necessary',
            },
            {
              id: 'inv-2',
              constraint: 'Product SKU must be unique across all products',
              predicate: 'unique(product.sku)',
              universality: 'necessary',
            },
            {
              id: 'inv-3',
              constraint: 'Product price must be positive',
              predicate: 'product.price > 0',
              universality: 'necessary',
            },
          ],
          universality: 'necessary',
        },
        facticity: {
          grounds: ['product-form'],
          conditions: ['context-product-catalog'],
          evidence: [
            {
              name: 'Product entities',
              definition:
                'Product entities maintain stock and pricing information',
              type: 'entity',
            },
          ],
        },
        mediates: {
          fromEntities: ['product-1', 'product-2', 'product-3', 'product-4'],
          toAspects: [],
        },
        context: {
          contextId: 'context-product-catalog',
          version: 1,
        },
      },
      meta: { seed: 'context-property' },
      tags: ['business', 'inventory', 'invariant'],
    };

    await propertyRepo.saveProperty(productInventoryProperty);
    console.log(`   ✅ Created Product Inventory Law`);

    // Property: Invoice Status Transition Law
    const invoiceStatusProperty: PropertyShapeRepo = {
      id: 'property-invoice-status-law',
      type: 'property.business',
      name: 'Invoice Status Transition Law',
      state: {
        status: 'active',
        contextId: 'context-invoice-management',
      },
      signature: {},
      facets: {
        law: {
          invariants: [
            {
              id: 'inv-1',
              constraint:
                'Invoice status transitions: DRAFT → PENDING → PAID/OVERDUE',
              predicate: 'validTransition(status, nextStatus)',
              universality: 'conditional',
            },
            {
              id: 'inv-2',
              constraint: 'Overdue invoices must have date < today',
              predicate: "status == 'OVERDUE' => date < now()",
              universality: 'conditional',
            },
          ],
          universality: 'conditional',
        },
        facticity: {
          grounds: ['invoice-form'],
          conditions: ['context-invoice-management'],
          evidence: [
            {
              name: 'Invoice status field',
              definition: 'Invoice entities have status field with enum values',
              type: 'field',
            },
          ],
        },
        mediates: {
          fromEntities: ['invoice-1', 'invoice-2', 'invoice-3', 'invoice-4'],
          toAspects: [],
        },
        context: {
          contextId: 'context-invoice-management',
          version: 1,
        },
      },
      meta: { seed: 'context-property' },
      tags: ['business', 'workflow', 'state-machine'],
    };

    await propertyRepo.saveProperty(invoiceStatusProperty);
    console.log(`   ✅ Created Invoice Status Transition Law`);

    // === 3. Summary ===
    console.log('\n📊 Reflective System Summary:');
    const contextSummary = await session.run(`
      MATCH (c:Context)
      RETURN count(c) as count
    `);
    const propertySummary = await session.run(`
      MATCH (p:Property)
      RETURN count(p) as count
    `);

    const contextCount = toNumber(contextSummary.records[0]?.get('count'));
    const propertyCount = toNumber(propertySummary.records[0]?.get('count'));

    console.log(`   Contexts: ${contextCount} (reflecting on Form/Entity)`);
    console.log(`   Properties: ${propertyCount} (grounding laws/invariants)`);

    console.log('\n✨ Context & Property seed completed successfully!');
    console.log('\n📐 Architecture:');
    console.log('   Horizontal: Form (Rational) ↔ Entity (Empirical)');
    console.log('   Vertical:   Context (Reflection) ↔ Property (Grounding)');
    console.log('   Unity:      Entity.facets bridges both dimensions');
  } catch (error) {
    console.error('❌ Error during context/property seed:', error);
    throw error;
  } finally {
    await session.close();
    await defaultConnection.close();
  }
}
