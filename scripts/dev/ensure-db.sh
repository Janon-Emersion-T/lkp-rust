#!/usr/bin/env bash
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$PROJECT_ROOT"

if [ -f ".env" ]; then
    set -a
    # shellcheck disable=SC1091
    source ".env"
    set +a
fi

if [ -z "${DATABASE_URL:-}" ]; then
    echo "❌ DATABASE_URL is missing."
    echo "Create a .env file with DATABASE_URL=postgres://user:password@localhost:5432/database_name"
    exit 1
fi

DB_URL="$DATABASE_URL"

DB_NAME="$(echo "$DB_URL" | sed -E 's#(postgres|postgresql)://[^/]+/([^?]+).*#\2#')"
DB_USER="$(echo "$DB_URL" | sed -E 's#(postgres|postgresql)://([^:/@]+).*#\2#')"

if [ -z "$DB_NAME" ] || [ -z "$DB_USER" ] || [ "$DB_NAME" = "$DB_URL" ] || [ "$DB_USER" = "$DB_URL" ]; then
    echo "❌ Could not parse DATABASE_URL."
    echo "Current DATABASE_URL: $DB_URL"
    exit 1
fi

echo "🗄 Database: $DB_NAME"
echo "👤 Owner: $DB_USER"

if ! command -v pg_isready >/dev/null 2>&1; then
    echo "❌ PostgreSQL client tools are missing."
    echo "Install them with:"
    echo "sudo apt install postgresql-client"
    exit 1
fi

if ! pg_isready >/dev/null 2>&1; then
    echo "❌ PostgreSQL is not running."
    echo "Start it with:"
    echo "sudo systemctl start postgresql"
    exit 1
fi

echo "🔎 Ensuring database exists..."

if sudo -u postgres createdb -O "$DB_USER" "$DB_NAME" 2>/tmp/lkp_createdb_error.log; then
    echo "✅ Database created."
else
    if grep -qi "already exists" /tmp/lkp_createdb_error.log; then
        echo "✅ Database already exists."
    else
        echo "❌ Could not create database."
        cat /tmp/lkp_createdb_error.log
        exit 1
    fi
fi

rm -f /tmp/lkp_createdb_error.log

echo "🧱 Running migrations..."
sqlx migrate run

echo "✅ Database is ready."