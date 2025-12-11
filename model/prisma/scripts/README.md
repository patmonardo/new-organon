# Database Backup & Restore

Quick backup and restore for dashboard_v3 database during development.

## Backup

Create a timestamped backup:

```bash
pnpm db:backup
```

Backups are saved to `prisma/backups/` as `dashboard_v3_YYYYMMDD_HHMMSS.sql.gz`

- Automatically compresses backups
- Keeps last 10 backups (deletes older ones)
- Uses `DATABASE_URL` from env or defaults to dashboard_v3

## Restore

Restore from latest backup:

```bash
pnpm db:restore
```

Restore from specific backup:

```bash
pnpm db:restore dashboard_v3_20241210_143022.sql.gz
```

**⚠️ WARNING**: Restore will:
- Drop the entire database
- Recreate it
- Restore from backup

Always backup before making schema changes!

## Workflow

1. **Before making changes:**
   ```bash
   pnpm db:backup
   ```

2. **Make your changes** (schema, migrations, etc.)

3. **If something breaks:**
   ```bash
   pnpm db:restore
   pnpm db:generate  # Regenerate Prisma client
   ```

## Manual Commands

You can also run scripts directly:

```bash
./prisma/scripts/backup.sh
./prisma/scripts/restore.sh [backup-file]
```

## Environment

Scripts use `DATABASE_URL` from environment, or default to:
`postgresql://dashboard_user:dashboard_pass@localhost:5432/dashboard_v3`

Set `DATABASE_URL` to use a different database.
