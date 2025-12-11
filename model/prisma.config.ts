import { defineConfig } from 'prisma/config';

export default defineConfig({
  schema: 'prisma/schema.prisma',
  datasource: {
    url: process.env.DATABASE_URL || "postgresql://dashboard_user:dashboard_pass@localhost:5432/dashboard_v3?schema=public"
  },
});
