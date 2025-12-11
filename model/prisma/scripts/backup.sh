#!/bin/bash
# Backup dashboard_v3 database

set -e

DB_URL="${DATABASE_URL:-postgresql://dashboard_user:dashboard_pass@localhost:5432/dashboard_v3}"
BACKUP_DIR="$(dirname "$0")/../backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/dashboard_v3_${TIMESTAMP}.sql"

# Create backups directory if it doesn't exist
mkdir -p "$BACKUP_DIR"

# Extract connection details from URL
# Format: postgresql://user:pass@host:port/dbname
DB_URL_NO_SCHEMA=$(echo "$DB_URL" | sed 's/?.*$//')
DB_NAME=$(echo "$DB_URL_NO_SCHEMA" | sed 's|.*/||')
DB_HOST=$(echo "$DB_URL_NO_SCHEMA" | sed 's|.*@||' | sed 's|:.*||')
DB_PORT=$(echo "$DB_URL_NO_SCHEMA" | sed 's|.*:||' | sed 's|/.*||')
DB_USER=$(echo "$DB_URL_NO_SCHEMA" | sed 's|postgresql://||' | sed 's|:.*||')
DB_PASS=$(echo "$DB_URL_NO_SCHEMA" | sed "s|postgresql://${DB_USER}:||" | sed 's|@.*||')

echo "Backing up database: $DB_NAME"
echo "Backup file: $BACKUP_FILE"

# Set PGPASSWORD for non-interactive auth
export PGPASSWORD="$DB_PASS"

# Create backup
pg_dump -h "$DB_HOST" -p "${DB_PORT:-5432}" -U "$DB_USER" -d "$DB_NAME" \
  --clean --if-exists --no-owner --no-acl \
  -f "$BACKUP_FILE"

# Compress backup
gzip -f "$BACKUP_FILE"
BACKUP_FILE="${BACKUP_FILE}.gz"

echo "✓ Backup created: $BACKUP_FILE"
echo "  Size: $(du -h "$BACKUP_FILE" | cut -f1)"

# Keep only last 10 backups
cd "$BACKUP_DIR"
ls -t dashboard_v3_*.sql.gz 2>/dev/null | tail -n +11 | xargs -r rm -f

echo "✓ Backup complete"
