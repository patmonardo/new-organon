/**
 * OpenCypher AST: Intermediate Representation
 *
 * This module defines the parsed Cypher AST structures that serve as
 * the intermediate language between dialectical schema and Neo4j execution.
 *
 * GDSL uses these ASTs directly—no syntax parsing needed.
 * AI codegens dialectic states → Cypher AST patterns.
 */

/**
 * Cypher AST Node Types
 * Based on OpenCypher specification
 */
export type CypherASTNode =
  | CypherQuery
  | CypherPattern
  | CypherExpression
  | CypherClause;

/**
 * Cypher Query (root AST node)
 */
export interface CypherQuery {
  type: 'Query';
  clauses: CypherClause[];
  parameters?: Record<string, unknown>;
}

/**
 * Cypher Clauses
 */
export type CypherClause =
  | MatchClause
  | CreateClause
  | MergeClause
  | SetClause
  | WhereClause
  | ReturnClause
  | WithClause
  | DeleteClause
  | UnwindClause;

export interface MatchClause {
  type: 'Match';
  optional?: boolean;
  patterns: CypherPattern[];
  where?: CypherExpression;
}

export interface CreateClause {
  type: 'Create';
  patterns: CypherPattern[];
}

export interface MergeClause {
  type: 'Merge';
  patterns: CypherPattern[];
  onCreate?: SetClause;
  onMatch?: SetClause;
}

export interface SetClause {
  type: 'Set';
  items: SetItem[];
}

export interface SetItem {
  property: PropertyExpression;
  expression: CypherExpression;
}

export interface WhereClause {
  type: 'Where';
  expression: CypherExpression;
}

export interface ReturnClause {
  type: 'Return';
  distinct?: boolean;
  items: ReturnItem[];
  orderBy?: OrderByItem[];
  skip?: number;
  limit?: number;
}

export interface ReturnItem {
  expression: CypherExpression;
  alias?: string;
}

export interface WithClause {
  type: 'With';
  items: ReturnItem[];
  where?: CypherExpression;
}

export interface DeleteClause {
  type: 'Delete';
  detach?: boolean;
  expressions: CypherExpression[];
}

export interface UnwindClause {
  type: 'Unwind';
  expression: CypherExpression;
  variable: string;
}

/**
 * Cypher Patterns
 */
export type CypherPattern =
  | NodePattern
  | RelationshipPattern
  | PathPattern;

export interface NodePattern {
  type: 'NodePattern';
  variable?: string;
  labels: string[];
  properties?: Record<string, CypherExpression>;
}

export interface RelationshipPattern {
  type: 'RelationshipPattern';
  variable?: string;
  direction: 'left' | 'right' | 'both';
  types: string[];
  properties?: Record<string, CypherExpression>;
  length?: RelationshipLength;
}

export interface RelationshipLength {
  min?: number;
  max?: number;
}

export interface PathPattern {
  type: 'PathPattern';
  nodes: NodePattern[];
  relationships: RelationshipPattern[];
}

/**
 * Cypher Expressions
 */
export type CypherExpression =
  | LiteralExpression
  | VariableExpression
  | PropertyExpression
  | FunctionCallExpression
  | BinaryExpression
  | UnaryExpression
  | ListExpression
  | MapExpression
  | CaseExpression
  | PatternExpression;

export interface LiteralExpression {
  type: 'Literal';
  value: string | number | boolean | null;
}

export interface VariableExpression {
  type: 'Variable';
  name: string;
}

export interface PropertyExpression {
  type: 'Property';
  object: CypherExpression;
  property: string;
}

export interface FunctionCallExpression {
  type: 'FunctionCall';
  name: string;
  args: CypherExpression[];
}

export interface BinaryExpression {
  type: 'Binary';
  operator: BinaryOperator;
  left: CypherExpression;
  right: CypherExpression;
}

export type BinaryOperator =
  | '+'
  | '-'
  | '*'
  | '/'
  | '%'
  | '^'
  | '='
  | '<>'
  | '<'
  | '>'
  | '<='
  | '>='
  | 'AND'
  | 'OR'
  | 'XOR'
  | 'IN'
  | 'STARTS WITH'
  | 'ENDS WITH'
  | 'CONTAINS'
  | 'IS NULL'
  | 'IS NOT NULL';

export interface UnaryExpression {
  type: 'Unary';
  operator: UnaryOperator;
  expression: CypherExpression;
}

export type UnaryOperator = '+' | '-' | 'NOT';

export interface ListExpression {
  type: 'List';
  items: CypherExpression[];
}

export interface MapExpression {
  type: 'Map';
  entries: Record<string, CypherExpression>;
}

export interface CaseExpression {
  type: 'Case';
  input?: CypherExpression;
  when: { condition: CypherExpression; then: CypherExpression }[];
  default?: CypherExpression;
}

export interface PatternExpression {
  type: 'Pattern';
  pattern: CypherPattern;
}

export interface OrderByItem {
  expression: CypherExpression;
  direction?: 'ASC' | 'DESC';
}

/**
 * Helper: Convert dialectic state to Cypher pattern
 * This is where the magic happens—dialectic → Cypher AST
 */
export function dialecticStateToCypherPattern(
  state: import('../../../logic/src/schema/dialectic').DialecticState
): CypherPattern {
  // Map dialectic moments to Cypher nodes
  const nodes: NodePattern[] = state.moments.map((moment, idx) => ({
    type: 'NodePattern',
    variable: moment.name,
    labels: [state.concept, moment.type],
    properties: {
      definition: { type: 'Literal', value: moment.definition },
    },
  }));

  // Map transitions to Cypher relationships
  const relationships: RelationshipPattern[] =
    state.transitions?.map((trans) => ({
      type: 'RelationshipPattern',
      variable: trans.id,
      direction: 'right',
      types: [trans.mechanism],
      properties: trans.middleTerm
        ? {
            middleTerm: { type: 'Literal', value: trans.middleTerm },
          }
        : undefined,
    })) || [];

  // Build path pattern
  if (nodes.length === 1) {
    return nodes[0];
  }

  return {
    type: 'PathPattern',
    nodes,
    relationships,
  };
}

