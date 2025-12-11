# Dashboard Example Architecture

This application serves as a reference implementation (Exemplar) for auto-generated Data Model Apps. It demonstrates a clean separation of concerns based on a "Pure Formal Ontology" mapping to modern web technologies.

## 5-Layer Architecture

The application is structured into 5 distinct layers:

1.  **`(controller)`** (`app/(controller)`): **Routing Layer**

    - Next.js App Router file-system based routing.
    - Responsible for URL mapping and delegation to the Logic Layer.
    - Key file: `page.tsx`

2.  **`controller`** (`app/controller`): **Logic Layer**

    - Orchestration classes.
    - Handles business logic flow, data fetching via Model, and selection of View.
    - Example: `CustomerController`

3.  **`view`** (`app/view`): **Presentation Layer**

    - View Logic classes.
    - Responsible for preparing data for display (formatting, shaping) and selecting the concrete UI Strategy.
    - Example: `CustomerView`

4.  **`graphics`** (`app/graphics`): **UI Layer**

    - Pure React Components.
    - Reusable UI elements (Forms, Tables, Charts).
    - "Dumb" components that render data shapes.

5.  **`model`** (`app/model`): **Data Layer**
    - Data Access and Schema validation.
    - Extends `BaseModel` for consistent behavior.
    - Wraps Zod Schemas (`shape`) and Prisma Client (`data`).
    - Example: `CustomerModel`

## Pure Formal Ontology

This structure is designed to support a "System of Given Forms" generated from a "Rational System of Pure Forms".

- **Model**: Represents the **Substance** (Data/Being).
- **View**: Represents the **Manifestation** (Appearance/Essence).
- **Controller**: Represents the **Mediation** (Concept/Notion).

The Code Generator can synthesize these layers from a high-level Logic Model, ensuring that the implementation strictly follows the formal definitions of the entities.

| Layer     | Dashboard Component | Pure Form Primitive         | Role                     |
| :-------- | :------------------ | :-------------------------- | :----------------------- |
| **Logic** | `controller/*`      | `model/sdsl/FormController` | Mediation (Concept)      |
| **View**  | `view/*`            | `model/sdsl/FormView`       | Representation (Essence) |
| **Model** | `model/*`           | `logic/schema/FormShape`    | Substance (Being)        |
| **Data**  | `prisma/*`          | `logic/relative/FormEntity` | Storage                  |
| **UI**    | `graphics/*`        | -                           | Manifestation            |

## Reverse Engineered Codegen Architecture

This exemplar serves as the blueprint for the "Zod First" generation pipeline.

### The "Zod First" Pipeline

1.  **Input**: Pure Form Logic Definition (e.g., `Customer` Entity).
2.  **Transformation**:
    - Generate Zod Schemas (`FormShape`) -> `app/data/schema/*.ts`
    - Generate Prisma Models (`FormEntity`) -> `app/data/prisma/schema.prisma`
3.  **Synthesis**:
    - Generate `Model` class extending `BaseModel`.
    - Generate `View` class extending `FormView`.
    - Generate `Controller` class extending `FormController` (conceptually).
    - Generate Next.js Route Handlers (`app/(controller)`).

### Template Skeleton

The Code Generator targets the following "Skeleton" structure:

```text
app/
├── (controller)/
│   └── __entity_plural__/      # Next.js Route Handlers
│       ├── page.tsx            # List View
│       ├── create/page.tsx     # Create View
│       └── [id]/
│           └── edit/page.tsx   # Edit View
├── controller/
│   └── __entity__.tsx          # Orchestration Logic
├── view/
│   └── __entity__.ts           # Presentation Logic
├── model/
│   └── __entity__.ts           # Data Logic
├── graphics/
│   ├── form/                   # UI Components
│   └── schema/                 # Display Schemas
└── data/
    └── schema/
        └── __entity__.ts       # Zod Definitions
```
