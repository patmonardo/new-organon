import type { DataView, MeasureDefinition } from '../data/sdsl';

export interface SqlQuery {
  text: string;
  params: unknown[];
}

export class SqlEngine {
  toSelect(view: DataView): SqlQuery {
    const { model, query } = view;
    const selections: string[] = [];

    // Grouped fields
    const groups = query.group_by ?? [];
    selections.push(...groups);

    // Measures
    for (const measureName of query.aggregate ?? []) {
      const def = model.config.measures?.[measureName];
      if (!def) {
        throw new Error(`Unknown measure '${measureName}' in model '${model.config.name}'`);
      }
      selections.push(`${this.measureToSql(def)} AS "${measureName}"`);
    }

    if (selections.length === 0) {
      selections.push('*');
    }

    const textParts: string[] = [`SELECT ${selections.join(', ')}`, `FROM ${model.config.source}`];
    const params: unknown[] = [];

    if (query.filter) {
      const filters = Object.entries(query.filter).map(([field, value], idx) => {
        params.push(value);
        return `${field} = $${idx + 1}`;
      });
      if (filters.length > 0) {
        textParts.push(`WHERE ${filters.join(' AND ')}`);
      }
    }

    if (groups.length > 0) {
      textParts.push(`GROUP BY ${groups.join(', ')}`);
    }

    if (query.limit) {
      textParts.push(`LIMIT ${query.limit}`);
    }

    return {
      text: textParts.join(' '),
      params,
    };
  }

  private measureToSql(def: MeasureDefinition): string {
    if (def.sql) return def.sql;

    const field = def.field ?? '*';
    switch (def.type) {
      case 'sum':
        return `SUM(${field})`;
      case 'avg':
        return `AVG(${field})`;
      case 'min':
        return `MIN(${field})`;
      case 'max':
        return `MAX(${field})`;
      case 'count':
      default:
        return `COUNT(${field === '*' ? '*' : field})`;
    }
  }
}
