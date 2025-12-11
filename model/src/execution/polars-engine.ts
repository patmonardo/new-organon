// Analytics excluded - GDS Kernel handles this
// import pl from 'nodejs-polars';
// import type { DataFrame } from 'nodejs-polars';
// import type { Connection } from 'duckdb';
import { tableFromJSON } from 'apache-arrow';
import type { DataView } from '../data/sdsl';

// Stub types for excluded analytics
type DataFrame = any;
type Connection = any;
const pl = {
  DataFrame: class { constructor() {} },
  readJSON: () => ({})
} as any;

export interface ExecutionOptions {
  limit?: number;
  dataset?: PolarsDataset;
}

export interface ExecutionResult {
  plan: string;
  rows: Array<Record<string, unknown>>;
  meta: Record<string, unknown>;
}

export interface PolarsDataset {
  customers: CustomerRow[];
  invoices: InvoiceRow[];
}

export interface CustomerRow {
  id: string;
  name: string;
  email: string;
  imageUrl?: string;
  region?: string;
  createdAt?: string;
}

export interface InvoiceRow {
  id: string;
  customerId: string;
  amount: number;
  status: string;
  date: string;
}

/**
 * PolarsExecutionEngine
 * ---------------------
 * Compiles a DataView into Polars DataFrame operations backed by
 * Apache Arrow buffers. A lightweight DuckDB connection is used to
 * provide an EXPLAIN plan for observability.
 */
export class PolarsExecutionEngine {
  private duckdbModule: Promise<DuckDbModule | null> | null = null;

  constructor(private dataset?: PolarsDataset) {}

  setDataset(dataset: PolarsDataset): void {
    this.dataset = dataset;
  }

  async execute(view: DataView, options: ExecutionOptions = {}): Promise<ExecutionResult> {
    const dataset = options.dataset ?? this.dataset;
    if (!dataset) {
      throw new Error('PolarsExecutionEngine requires a dataset to execute');
    }

    const limit = options.limit ?? view.query.limit;
    const invoiceMap = this.buildInvoiceMap(dataset.invoices);
    const customerFrame = this.createFrame(dataset.customers as unknown as Record<string, unknown>[], 'customers');
    const invoiceFrame = this.createFrame(dataset.invoices as unknown as Record<string, unknown>[], 'invoices');

    const filteredCustomers = this.applyFilter(customerFrame, view.query.filter ?? {});
    const filteredInvoices = this.applyInvoiceFilter(invoiceFrame, view.query.filter ?? {});
    const aggregates = this.buildInvoiceAggregates(filteredInvoices);
    let enriched = this.joinCustomersWithMetrics(filteredCustomers, aggregates);
    if (limit && limit > 0) {
      enriched = enriched.head(limit);
    }

    const arrowBuffer = enriched.writeIPC();
    const rows = this.toRows(enriched).map(row => this.enrichRow(row, invoiceMap));
    const duckPlan = await this.buildDuckDbPlan(view);
    const logicalPlan = this.safeParsePlan(view.toPlan());

    return {
      plan: JSON.stringify({
        view: logicalPlan,
        duckdb: duckPlan,
      }, null, 2),
      rows,
      meta: {
        engine: 'polars',
        rowCount: rows.length,
        arrowBytes: arrowBuffer.byteLength,
        limit: limit ?? null,
      },
    };
  }

  private createFrame(rows: Array<Record<string, unknown>>, name: string): DataFrame {
    if (!rows.length) {
      return pl.DataFrame({ __empty: [] as number[] }).drop('__empty');
    }
    try {
      const table = tableFromJSON(rows);
      return pl.DataFrame(table as unknown as Record<string, unknown[]>);
    } catch {
      try {
        return pl.DataFrame(rows as unknown as Record<string, unknown[]>);
      } catch (inner) {
        throw new Error(`Failed to build Polars DataFrame for ${name}: ${(inner as Error).message}`);
      }
    }
  }

  private applyFilter(frame: DataFrame, filter: Record<string, unknown>): DataFrame {
    if (!filter || Object.keys(filter).length === 0) {
      return frame;
    }
    let current = frame;
    if (filter.id) {
      current = current.filter(pl.col('id').eq(pl.lit(String(filter.id))));
    }
    if (filter.region) {
      current = current.filter(pl.col('region').eq(pl.lit(String(filter.region))));
    }
    return current;
  }

  private applyInvoiceFilter(frame: DataFrame, filter: Record<string, unknown>): DataFrame {
    if (!filter || Object.keys(filter).length === 0) {
      return frame;
    }
    let current = frame;
    if (filter.id) {
      current = current.filter(pl.col('customerId').eq(pl.lit(String(filter.id))));
    }
    return current;
  }

  private buildInvoiceAggregates(frame: DataFrame): DataFrame {
    if (!frame.height) {
      return pl.DataFrame({
        customerId: [] as string[],
        totalRevenue: [] as number[],
        averageInvoice: [] as number[],
        invoiceCount: [] as number[],
      });
    }

    return frame.groupBy('customerId').agg(
      pl.col('amount').sum().alias('totalRevenue'),
      pl.col('amount').mean().alias('averageInvoice'),
      pl.col('amount').count().alias('invoiceCount'),
    );
  }

  private joinCustomersWithMetrics(customers: DataFrame, aggregates: DataFrame): DataFrame {
    if (!aggregates.height) {
      return customers.withColumns(
        pl.lit(0).alias('invoiceCount'),
        pl.lit(0).alias('totalRevenue'),
        pl.lit(0).alias('averageInvoice'),
      );
    }

    return customers.join(aggregates, {
      leftOn: 'id',
      rightOn: 'customerId',
      how: 'left',
    }).withColumns(
      pl.col('invoiceCount').fillNull(0),
      pl.col('totalRevenue').fillNull(0),
      pl.col('averageInvoice').fillNull(0),
    );
  }

  private toRows(frame: DataFrame): Array<Record<string, unknown>> {
    const buffer = frame.writeJSON({ format: 'json' });
    return JSON.parse(buffer.toString());
  }

  private enrichRow(row: Record<string, unknown>, invoiceMap: Map<string, InvoiceRow[]>): Record<string, unknown> {
    const id = row.id ? String(row.id) : undefined;
    if (!id) {
      return row;
    }

    const invoices = invoiceMap.get(id) ?? [];
    const totalRevenue = this.asNumber(row.totalRevenue) ?? this.sumAmounts(invoices);
    const invoiceCount = this.asNumber(row.invoiceCount) ?? invoices.length;
    const averageInvoice = invoiceCount > 0 ? Math.round(totalRevenue / invoiceCount) : 0;

    return {
      ...row,
      invoices,
      metrics: {
        invoiceCount,
        totalRevenue,
        averageInvoice,
      },
    };
  }

  private buildInvoiceMap(invoices: InvoiceRow[]): Map<string, InvoiceRow[]> {
    const map = new Map<string, InvoiceRow[]>();
    for (const invoice of invoices) {
      const customerId = invoice.customerId;
      if (!map.has(customerId)) {
        map.set(customerId, []);
      }
      map.get(customerId)?.push({ ...invoice });
    }
    return map;
  }

  private sumAmounts(invoices: InvoiceRow[]): number {
    return invoices.reduce((total, invoice) => total + (invoice.amount ?? 0), 0);
  }

  private asNumber(value: unknown): number | undefined {
    if (typeof value === 'number') {
      return value;
    }
    if (typeof value === 'string' && value.trim() !== '') {
      const parsed = Number(value);
      return Number.isFinite(parsed) ? parsed : undefined;
    }
    return undefined;
  }

  private async buildDuckDbPlan(view: DataView): Promise<string | undefined> {
    try {
      const sql = this.toSql(view);
      if (!sql) {
        return undefined;
      }

      const duckdb = await this.getDuckDb();
      if (!duckdb) {
        return 'duckdb-unavailable';
      }

      const DuckDbDatabase = (duckdb as any).default ?? duckdb;
      const db = new DuckDbDatabase(':memory:');
      const connection = db.connect();
      await this.bootstrapDuckDbSchema(connection);
      const plan = await new Promise<string>((resolve, reject) => {
        connection.all(`EXPLAIN ${sql}`, (err: Error | null, rows: Record<string, unknown>[]) => {
          connection.close(() => {
            if (err) {
              reject(err);
              return;
            }
            const serialized = rows
              .map((row: Record<string, unknown>) => Object.values(row).join(' '))
              .join('\n');
            resolve(serialized);
          });
        });
      });
      return plan;
    } catch (err) {
      return `duckdb-plan-error: ${(err as Error).message}`;
    }
  }

  private async getDuckDb(): Promise<DuckDbModule | null> {
    // Stub - analytics excluded, GDS Kernel handles this
    return null;
  }

  private async bootstrapDuckDbSchema(connection: Connection): Promise<void> {
    const statements = [
      `CREATE TABLE customers (id VARCHAR, name VARCHAR, email VARCHAR, imageUrl VARCHAR, region VARCHAR, createdAt VARCHAR);`,
      `CREATE TABLE invoices (id VARCHAR, customerId VARCHAR, amount DOUBLE, status VARCHAR, date VARCHAR);`,
    ];
    await Promise.all(statements.map(sql => this.runDuckDb(connection, sql)));
  }

  private runDuckDb(connection: Connection, sql: string): Promise<void> {
    return new Promise((resolve, reject) => {
      // Stub - analytics excluded
      const err: any = null;
      if (err) {
        reject(err);
      } else {
        resolve();
      }
    });
  }

  private toSql(view: DataView): string {
    const selectColumns = [
      'c.id',
      'c.name',
      'c.email',
      'c.imageUrl',
      'c.region',
      'c.createdAt',
    ];

    const metricSelects = [
      'SUM(i.amount) AS totalRevenue',
      'AVG(i.amount) AS averageInvoice',
      'COUNT(i.id) AS invoiceCount',
    ];

    const filterClauses: string[] = [];
    if (view.query.filter?.id) {
      filterClauses.push(`c.id = '${this.escapeSql(String(view.query.filter.id))}'`);
    }
    if (view.query.filter?.region) {
      filterClauses.push(`c.region = '${this.escapeSql(String(view.query.filter.region))}'`);
    }

    const whereClause = filterClauses.length ? `WHERE ${filterClauses.join(' AND ')}` : '';
    const limitClause = view.query.limit ? `LIMIT ${view.query.limit}` : '';

    return `SELECT ${[...selectColumns, ...metricSelects].join(', ')}
FROM customers c
LEFT JOIN invoices i ON c.id = i.customerId
${whereClause}
GROUP BY ${selectColumns.join(', ')}
${limitClause}`;
  }

  private escapeSql(value: string): string {
    return value.replace(/'/g, "''");
  }

  private safeParsePlan(plan: string): unknown {
    try {
      return JSON.parse(plan);
    } catch {
      return plan;
    }
  }
}

// Stub type - analytics excluded
type DuckDbModule = any;
