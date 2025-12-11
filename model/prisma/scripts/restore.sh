#!/bin/bash
# Restore dashboard_v3 database from backup

set -e

DB_URL="${DATABASE_URL:-postgresql://dashboard_user:dashboard_pass@localhost:5432/dashboard_v3}"
BACKUP_DIR="$(dirname "$0")/../backups"

# Extract connection details from URL
DB_URL_NO_SCHEMA=$(echo "$DB_URL" | sed 's/?.*$//')
DB_NAME=$(echo "$DB_URL_NO_SCHEMA" | sed 's|.*/||')
DB_HOST=$(echo "$DB_URL_NO_SCHEMA" | sed 's|.*@||' | sed 's|:.*||')
DB_PORT=$(echo "$DB_URL_NO_SCHEMA" | sed 's|.*:||' | sed 's|/.*||')
DB_USER=$(echo "$DB_URL_NO_SCHEMA" | sed 's|postgresql://||' | sed 's|:.*||')
DB_PASS=$(echo "$DB_URL_NO_SCHEMA" | sed "s|postgresql://${DB_USER}:||" | sed 's|@.*||')

# If backup file provided, use it; otherwise use latest
if [ -n "$1" ]; then
  BACKUP_FILE="$1"
  if [ ! -f "$BACKUP_FILE" ]; then
    # Try relative to backup dir
    BACKUP_FILE="$BACKUP_DIR/$1"
  fi
else
  # Find latest backup
  BACKUP_FILE=$(ls -t "$BACKUP_DIR"/dashboard_v3_*.sql.gz 2>/dev/null | head -1)
  if [ -z "$BACKUP_FILE" ]; then
    echo "❌ No backup found in $BACKUP_DIR"
    exit 1
  fi
fi

if [ ! -f "$BACKUP_FILE" ]; then
  echo "❌ Backup file not found: $BACKUP_FILE"
  exit 1
fi

echo "⚠️  WARNING: This will DROP and recreate database: $DB_NAME"
echo "   Backup file: $BACKUP_FILE"
read -p "Continue? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
  echo "Cancelled"
  exit 0
fi

echo "Restoring database: $DB_NAME"

# Set PGPASSWORD for non-interactive auth
export PGPASSWORD="$DB_PASS"

# Decompress if needed
if [[ "$BACKUP_FILE" == *.gz ]]; then
  TEMP_SQL=$(mktemp)
  gunzip -c "$BACKUP_FILE" > "$TEMP_SQL"
  SQL_FILE="$TEMP_SQL"
else
  SQL_FILE="$BACKUP_FILE"
fi

# Restore
psql -h "$DB_HOST" -p "${DB_PORT:-5432}" -U "$DB_USER" -d postgres <<EOF
SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE datname = '$DB_NAME' AND pid <> pg_backend_pid();
DROP DATABASE IF EXISTS $DB_NAME;
CREATE DATABASE $DB_NAME;
EOF

psql -h "$DB_HOST" -p "${DB_PORT:-5432}" -U "$DB_USER" -d "$DB_NAME" -f "$SQL_FILE"

# Cleanup temp file if we created one
[ -n "$TEMP_SQL" ] && rm -f "$TEMP_SQL"

echo "✓ Database restored from: $BACKUP_FILE"
echo "  Run 'pnpm db:generate' to regenerate Prisma client"
