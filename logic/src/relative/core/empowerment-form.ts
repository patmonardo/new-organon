import { EmpowermentSchema, type Empowerment } from '@schema';

/**
 * Thin Form wrapper for Empowerment schema.
 * - preserves schema invariants via EmpowermentSchema.parse()
 * - exposes ergonomic getters and toSchema()
 * - provides a static combine() used by the processor
 */
export default class FormEmpowerment {
  private _schema: Empowerment;

  constructor(data: Partial<Empowerment> | Empowerment) {
    // parse/normalize immediately to preserve canonical invariants
    this._schema = EmpowermentSchema.parse({
      ...data,
    } as Empowerment);
  }

  static create(data: Partial<Empowerment> | Empowerment) {
    return new FormEmpowerment(data);
  }

  toSchema(): Empowerment {
    return this._schema;
  }

  toJSON() {
    return this.toSchema();
  }

  get id(): string {
    return this._schema.id;
  }

  get subject(): string {
    return this._schema.subject;
  }

  get weight(): number {
    return typeof this._schema.weight === 'number' ? this._schema.weight : 0;
  }

  get certainty(): number {
    return typeof this._schema.certainty === 'number' ? this._schema.certainty : 1;
  }

  get provenance() {
    return this._schema.provenance;
  }

  // Extract numeric score for a raw empowerment-like object
  static scoreOf(obj: any): number {
    if (!obj) return 0;
    const weight =
      typeof obj.weight === 'number' ? obj.weight :
      typeof obj.w === 'number' ? obj.w : 0;
    const certainty =
      typeof obj.certainty === 'number' ? obj.certainty :
      typeof obj.c === 'number' ? obj.c : 1;
    return weight * certainty;
  }

  /**
   * Combine multiple empowerments into canonical result:
   * { total, scores: [{ id, subject, score, provenance? }] }
   *
   * Accepts FormEmpowerment instances or raw objects conforming to schema.
   */
  static combine(
    tokens: Array<FormEmpowerment | Partial<Empowerment> | any>,
    root?: FormEmpowerment | Partial<Empowerment> | any,
    opts?: { privilegeBoost?: number },
  ) {
    const boost = typeof opts?.privilegeBoost === 'number' ? opts.privilegeBoost : 1;

    const normalize = (t: any) => {
      if (!t) return { id: 'unknown', subject: 'unknown', score: 0, provenance: undefined };
      const schema = typeof t.toSchema === 'function' ? t.toSchema() : t;
      const id = schema?.id ?? String(Math.random()).slice(2);
      const subject = schema?.subject ?? schema?.entity ?? 'unknown';
      const provenance = schema?.provenance;
      const score = FormEmpowerment.scoreOf(schema);
      return { id, subject, score: score * boost, provenance };
    };

    const scores = (tokens || []).map(normalize);

    if (root) {
      const rootNorm = normalize(root);
      // if root not duplicated, include it
      if (!scores.some(s => s.id === rootNorm.id)) {
        scores.push(rootNorm);
      }
    }

    const total = scores.reduce((acc, it) => acc + (typeof it.score === 'number' ? it.score : 0), 0);
    return { total, scores };
  }
}
