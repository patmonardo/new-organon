/**
 * Prisma Client - Database connection for @model
 * 
 * Provides a singleton PrismaClient instance.
 * In development, prevents multiple instances during hot-reload.
 */

import { PrismaClient } from "@prisma/client";

// Global declaration for development hot-reload
declare global {
  // eslint-disable-next-line no-var
  var __prisma: PrismaClient | undefined;
}

/**
 * Create or reuse PrismaClient instance
 * 
 * In development: reuses global instance to prevent connection exhaustion
 * In production: creates fresh instance
 */
function createPrismaClient(): PrismaClient {
  if (process.env.NODE_ENV === "production") {
    return new PrismaClient();
  }

  // Development: use global to survive hot-reload
  if (!global.__prisma) {
    global.__prisma = new PrismaClient({
      log: process.env.DEBUG ? ["query", "info", "warn", "error"] : ["error"],
    });
  }

  return global.__prisma;
}

/**
 * Singleton Prisma client instance
 */
export const prisma = createPrismaClient();

/**
 * Disconnect from database (for cleanup)
 */
export async function disconnect(): Promise<void> {
  await prisma.$disconnect();
}

/**
 * Check if database is connected
 */
export async function isConnected(): Promise<boolean> {
  try {
    await prisma.$queryRaw`SELECT 1`;
    return true;
  } catch {
    return false;
  }
}

