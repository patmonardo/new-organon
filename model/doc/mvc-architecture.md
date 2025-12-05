# MVC Architecture: General Data Models + Middleware

## Overview

**MVC (SDSL) is oriented toward:**
- **General Data Models** - Any data modeling domain
- **Middleware** - Business logic, transformations, orchestration
- **Special Domain Calculators** - Domain-specific calculation engines

**Not just Personal Data Science** - but a general-purpose data modeling and middleware platform.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│              MVC (SDSL) Platform                        │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ Data Models │  │ Middleware   │  │ Calculators  │ │
│  │ (General)   │  │ (Business)   │  │ (Domain)     │ │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘ │
│         │                 │                  │         │
│         └─────────┬───────┴──────────────────┘         │
│                   │                                     │
│         ┌─────────▼──────────┐                         │
│         │  MVC Layer         │                         │
│         │  (Controllers)     │                         │
│         └─────────┬──────────┘                         │
│                   │                                     │
│         ┌─────────▼──────────┐                         │
│         │  Execution Layer   │                         │
│         │  Polars/Arrow      │                         │
│         └─────────────────────┘                         │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

## General Data Models

### Any Domain

MVC supports **general data modeling** - not limited to specific domains:

```typescript
// E-commerce domain
export const ProductModel = defineModel({ ... });

// Healthcare domain
export const PatientModel = defineModel({ ... });

// Finance domain
export const TransactionModel = defineModel({ ... });

// Education domain
export const CourseModel = defineModel({ ... });
```

**Malloy-inspired semantic modeling** works for any domain.

## Middleware

### Business Logic Layer

**Middleware** handles:
- Business logic
- Data transformations
- Orchestration
- Integration
- Validation
- Caching

```typescript
// Middleware example
class CustomerMiddleware {
  async transformCustomerData(rawData: unknown): Promise<Customer> {
    // Business logic transformation
    // Validation
    // Enrichment
    return transformed;
  }
  
  async orchestrateWorkflow(customerId: string): Promise<void> {
    // Coordinate multiple operations
    // Handle dependencies
    // Manage state
  }
}
```

## Special Domain Calculators

### Domain-Specific Calculation Engines

**Special Domain Calculators** provide:
- Domain-specific calculations
- Specialized algorithms
- Custom aggregations
- Business rules

```typescript
// Financial calculator
class FinancialCalculator {
  calculateInterest(principal: number, rate: number, time: number): number {
    // Financial calculation logic
  }
  
  calculateAmortization(loan: Loan): AmortizationSchedule {
    // Amortization calculation
  }
}

// Healthcare calculator
class HealthcareCalculator {
  calculateBMI(weight: number, height: number): number {
    // BMI calculation
  }
  
  calculateRiskScore(patient: Patient): RiskScore {
    // Risk assessment
  }
}
```

## Comparison: GDS vs MVC

| Aspect | GDS (GDSL) | MVC (SDSL) |
|--------|------------|------------|
| **Orientation** | ML, Logical Forms | General Data Models, Middleware |
| **Domain** | Advanced ML, Logical processing | Any domain, Business logic |
| **Scale** | Enterprise, Big Data | Personal/Team, General purpose |
| **Language** | Rust (OpenCypher) | TypeScript (Malloy) |
| **Execution** | GDS Kernel | Polars/Arrow |
| **Use Cases** | ML training, Logical forms | Data modeling, Middleware, Calculators |

## MVC Stack

**Powerful stack for General Data Modeling:**

1. **Malloy** - Semantic modeling (any domain)
2. **Polars** - DataFrame execution
3. **Apache Arrow** - Columnar memory
4. **TypeScript** - Business logic, middleware
5. **React** - UI components
6. **Radix + Material** - Rich components
7. **Domain Calculators** - Special-purpose engines

## Integration with GDS

**MVC can call GDS when needed:**

```typescript
// MVC middleware that uses GDS for ML operations
class AdvancedAnalyticsMiddleware {
  async analyzeWithML(data: DataModel): Promise<MLResult> {
    // Use GDS for ML operations
    const mlResult = await gds.executeML({
      model: 'recommendation-model',
      data: data
    });
    return mlResult;
  }
}
```

**But MVC is primarily:**
- General data modeling
- Middleware and business logic
- Domain calculators
- Form applications

## Key Principles

1. **MVC = General Purpose** - Not limited to specific domains
2. **Middleware Layer** - Business logic, transformations
3. **Domain Calculators** - Special-purpose calculation engines
4. **GDS = ML/Logical Forms** - Advanced operations when needed
5. **Clear Separation** - MVC for general, GDS for ML/Logical

## Next Steps

- [ ] Document general data modeling patterns
- [ ] Design middleware architecture
- [ ] Create domain calculator framework
- [ ] Document MVC → GDS integration points
- [ ] Clarify when to use MVC vs GDS

