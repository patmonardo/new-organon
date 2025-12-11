# Prisma Schema - Dashboard V4

This directory contains the Prisma schema and migrations for this self-contained dashboard application.

## Structure

```
app/data/prisma/
├── schema.prisma      # Database schema definition
├── migrations/        # Database migration history (created on first migrate)
└── README.md          # This file
```

## Commands

All Prisma commands are configured in `package.json` to use this local schema:

```bash
# Generate Prisma Client
pnpm generate

# Open Prisma Studio
pnpm studio

# Create a migration
pnpm migrate-dev

# Deploy migrations
pnpm migrate
```

## Database URL

Configure your database connection in `.env`:

```env
DATABASE_URL="postgresql://user:pass@localhost:5432/dashboard_v4?schema=public"
```

Or it will use the default from `prisma.config.ts`.

## Design Philosophy

Each generated app has its own Prisma schema and migration history. This makes the app:
- **Self-contained**: All data definitions live with the app
- **Portable**: Easy to move or deploy independently
- **Isolated**: No shared database concerns
- **Clear**: Schema and migrations co-located with data access code
