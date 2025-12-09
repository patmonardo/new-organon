# Dashboard Example (Next App)

This folder contains an isolated Next App Router demo for the dashboard UI. It stays separate from the main model package.

## Run
```bash
cd model/examples/dashboard
pnpm install
pnpm dev
```
Then open http://localhost:3000 — the root redirects to `/dashboard`.

## Structure
- `app/` — Next App Router pages/layout (moved from the earlier `(controller)` group).
- `controller/` — controller classes used by the pages.
- `model/`, `view/` — example data models and views for the demo.
- `public/` — static assets (favicon, images, manifest).
- `tailwind.config.ts`, `postcss.config.js`, `app/globals.css` — Tailwind setup scoped to this example.

## Notes
- Keep changes contained here; the model package remains framework-agnostic.
- If you need additional routes, add them under `app/` and reuse the controllers/views from this example.

