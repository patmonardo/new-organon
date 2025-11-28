/**
 * Dashboard Service
 *
 * CRUD operations for Dashboard model.
 * Manages dashboard configurations and components.
 */

import type { DashboardShape, DashboardComponent } from '../schema/dashboard';

/**
 * DashboardInput: Data for creating/updating a dashboard
 */
export interface DashboardInput {
  name: string;
  title?: string;
  description?: string;
  config?: Partial<DashboardShape>;
  isDefault?: boolean;
}

/**
 * DashboardFilter: Criteria for querying dashboards
 */
export interface DashboardFilter {
  id?: string;
  name?: string;
  isDefault?: boolean;
}

/**
 * StoredDashboard: Dashboard as stored in database
 */
export interface StoredDashboard {
  id: string;
  name: string;
  title?: string;
  description?: string;
  config: DashboardShape;
  isDefault: boolean;
  createdAt: Date;
  updatedAt: Date;
}

/**
 * DashboardService interface
 * Can be implemented with any persistence layer or mocked for testing
 */
export interface DashboardService {
  create(input: DashboardInput): Promise<StoredDashboard>;
  findById(id: string): Promise<StoredDashboard | null>;
  findMany(filter?: DashboardFilter): Promise<StoredDashboard[]>;
  findDefault(): Promise<StoredDashboard | null>;
  update(id: string, input: Partial<DashboardInput>): Promise<StoredDashboard>;
  delete(id: string): Promise<boolean>;
  setDefault(id: string): Promise<StoredDashboard>;
}

/**
 * Mock DashboardService for testing
 */
export class MockDashboardService implements DashboardService {
  private dashboards: Map<string, StoredDashboard> = new Map();
  private idCounter = 1;

  async create(input: DashboardInput): Promise<StoredDashboard> {
    const dashboard: StoredDashboard = {
      id: `dashboard-${this.idCounter++}`,
      name: input.name,
      title: input.title,
      description: input.description,
      config: {
        id: `dashboard-${this.idCounter}`,
        name: input.name,
        type: 'dashboard',
        fields: [],
        layout: {
          title: input.title || input.name,
          gridColumns: 12,
        },
        components: [],
        ...input.config,
      } as DashboardShape,
      isDefault: input.isDefault ?? false,
      createdAt: new Date(),
      updatedAt: new Date(),
    };
    this.dashboards.set(dashboard.id, dashboard);
    return dashboard;
  }

  async findById(id: string): Promise<StoredDashboard | null> {
    return this.dashboards.get(id) || null;
  }

  async findMany(filter?: DashboardFilter): Promise<StoredDashboard[]> {
    let results = Array.from(this.dashboards.values());

    if (filter?.id) {
      results = results.filter(d => d.id === filter.id);
    }
    if (filter?.name) {
      results = results.filter(d => d.name === filter.name);
    }
    if (filter?.isDefault !== undefined) {
      results = results.filter(d => d.isDefault === filter.isDefault);
    }

    return results;
  }

  async findDefault(): Promise<StoredDashboard | null> {
    const defaults = await this.findMany({ isDefault: true });
    return defaults[0] || null;
  }

  async update(id: string, input: Partial<DashboardInput>): Promise<StoredDashboard> {
    const dashboard = this.dashboards.get(id);
    if (!dashboard) {
      throw new Error(`Dashboard not found: ${id}`);
    }

    const updated: StoredDashboard = {
      ...dashboard,
      name: input.name ?? dashboard.name,
      title: input.title ?? dashboard.title,
      description: input.description ?? dashboard.description,
      isDefault: input.isDefault ?? dashboard.isDefault,
      config: {
        ...dashboard.config,
        ...input.config,
      } as DashboardShape,
      updatedAt: new Date(),
    };
    this.dashboards.set(id, updated);
    return updated;
  }

  async delete(id: string): Promise<boolean> {
    return this.dashboards.delete(id);
  }

  async setDefault(id: string): Promise<StoredDashboard> {
    // Clear existing defaults
    for (const [dashId, dash] of this.dashboards) {
      if (dash.isDefault) {
        this.dashboards.set(dashId, { ...dash, isDefault: false });
      }
    }

    // Set new default
    return this.update(id, { isDefault: true });
  }

  // Test helpers
  clear(): void {
    this.dashboards.clear();
    this.idCounter = 1;
  }

  seed(dashboards: StoredDashboard[]): void {
    for (const dashboard of dashboards) {
      this.dashboards.set(dashboard.id, dashboard);
    }
  }
}

/**
 * Default mock service instance for development/testing
 */
export const mockDashboardService = new MockDashboardService();
