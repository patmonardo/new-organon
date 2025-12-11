//@/data/client.ts
import { PrismaClient } from '@prisma/client'
import { PrismaPg } from '@prisma/adapter-pg'

declare global {
  var prisma: PrismaClient | undefined
}

// Ensure DATABASE_URL is set for Prisma Client
const databaseUrl = process.env.DATABASE_URL || "postgresql://dashboard_user:dashboard_pass@localhost:5432/dashboard_v3?schema=public"

// Create the adapter with connection string
const adapter = new PrismaPg({ connectionString: databaseUrl })

const prisma = global.prisma || new PrismaClient({ adapter })

if (process.env.NODE_ENV !== 'production') {
  global.prisma = prisma
}

export { prisma }
