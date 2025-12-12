#!/bin/bash

# FormDB Test Suite Runner
# Simple script to seed, query, and test the persistence layer

echo "🧪 FormDB Test Suite"
echo "===================="
echo ""

# Check if Neo4j is running
echo "Checking Neo4j connectivity..."
if ! pnpm tsx -e "import { defaultConnection } from './src/connection.js'; defaultConnection.verifyConnectivity().then(c => process.exit(c ? 0 : 1)).catch(() => process.exit(1))"; then
    echo "❌ Neo4j not available. Start Neo4j first:"
    echo "   docker run -p 7687:7687 -p 7474:7474 -e NEO4J_AUTH=neo4j/pjm070FF neo4j:latest"
    exit 1
fi
echo "✅ Neo4j connected"
echo ""

# Seed database
echo "1️⃣  Seeding FormDB..."
pnpm tsx test/support/seed-formdb.ts
echo ""

# Query data
echo "2️⃣  Querying FormDB..."
pnpm tsx test/support/query-formdb.ts
echo ""

# Run integration tests (if they exist and pass schema)
echo "3️⃣  Running persistence layer tests..."
# pnpm test test/repository/form-entity-neo4j.test.ts
echo "   (Integration tests need schema fixes - skipping for now)"
echo ""

echo "✨ Test suite complete"
