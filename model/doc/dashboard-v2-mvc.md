# Dashboard V2 – MVC Mapping and Layout Plan

## Quick comparison
- **dashboard controllers** (`model/examples/dashboard/controller/{customer,invoice}.tsx`): Next entrypoints. Fetch via `CustomerModel`/`InvoiceModel`, call `View.display(...)`, wire server actions (`create/update/cancel`) and use `notFound/redirect/revalidatePath`. Return `ReactNode` for App Router pages.
- **customer demo**: removed/archived; previously a Radix/SDSL client-heavy demo.

## Fixed layout for the example app (no mainline impact)
- Location: `model/examples/dashboard/` is a self-contained Next App Router demo. Mainline `src/*` remains untouched.
- App tree (`model/examples/dashboard/app/`):
  - `(controller)/...` routes + server actions
  - `controller/` classes (imported by routes)
  - `data/` adapters/services
  - `model/` domain models
  - `schema/` Zod schemas
  - `view/` view helpers
  - `layout.tsx`, `page.tsx` (redirect), `globals.css`
- Assets/config: `public/`, `tailwind.config.ts`, `postcss.config.js`, `next.config.js`, `tsconfig.json`, `vercel.json`, `package.json` are scoped to this example.

## Minimal flow (CRUD/list)
1) Page (App Router) calls a controller method.  
2) Controller hits model/data, then passes results to the view (`View.display(...)`).  
3) View returns JSX for forms/tables/cards; server actions are injected via `FormHandler`.  
4) Server actions live under `app/(controller)/.../actions/*.ts` and call models, then `revalidatePath`/`redirect`.

## Recharts + dashboard shell
- Keep chart/layout logic where it already lives in the example; add wrappers locally if needed.
- Mock data can stay in the example model/data layer until Arrow/Polars/DuckDB is wired.


