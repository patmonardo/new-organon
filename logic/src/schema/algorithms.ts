import { z } from 'zod';

import { GdsGraphNameSchema } from './common';
import { gdsApplicationBase } from './application';

/**
 * Algorithms facade.
 *
 * This models the Rust Applications / Control Logic algorithm dispatcher.
 */
export const GdsAlgorithmsFacadeSchema = z.literal('algorithms');
export type GdsAlgorithmsFacade = z.infer<typeof GdsAlgorithmsFacadeSchema>;

const AlgorithmsBase = gdsApplicationBase(GdsAlgorithmsFacadeSchema);

/**
 * Algorithm execution mode (Java GDS parity).
 * - stream: Returns individual result rows
 * - stats: Returns aggregated statistics only
 * - mutate: Writes results to in-memory graph projection
 * - write: Writes results back to database
 * - estimate: Returns memory estimation without executing
 */
export const AlgorithmModeSchema = z
  .enum(['stream', 'stats', 'mutate', 'write', 'estimate', 'train'])
  .default('stream');
export type AlgorithmMode = z.infer<typeof AlgorithmModeSchema>;

/**
 * Submode for estimate operations.
 * - memory: Memory usage estimation
 */
export const EstimateSubmodeSchema = z.enum(['memory']).optional();
export type EstimateSubmode = z.infer<typeof EstimateSubmodeSchema>;

// NOTE: This union is intentionally large; it can be split further by Java package
// (centrality/community/pathfinding/embeddings/...) once the op surface is stable.
export const GdsAlgorithmsCallSchema = z.discriminatedUnion('op', [
  // ==========================================================================
  // Pathfinding Algorithms (unified with mode parameter)
  // ==========================================================================
  AlgorithmsBase.extend({
    op: z.literal('bfs'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    sourceNode: z.number().int().nonnegative(),
    targetNodes: z.array(z.number().int().nonnegative()).optional(),
    maxDepth: z.number().int().positive().optional(),
    trackPaths: z.boolean().optional(),
    concurrency: z.number().int().positive().optional(),
    delta: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('dfs'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    sourceNode: z.number().int().nonnegative(),
    targetNodes: z.array(z.number().int().nonnegative()).optional(),
    maxDepth: z.number().int().positive().optional(),
    trackPaths: z.boolean().optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('dijkstra'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    sourceNode: z.number().int().nonnegative(),
    targetNode: z.number().int().nonnegative().optional(),
    targetNodes: z.array(z.number().int().nonnegative()).optional(),
    weightProperty: z.string().min(1).optional(),
    direction: z.enum(['incoming', 'outgoing', 'both']).optional(),
    trackRelationships: z.boolean().optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('bellman_ford'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    sourceNode: z.number().int().nonnegative(),
    weightProperty: z.string().min(1).optional(),
    direction: z.enum(['incoming', 'outgoing']).optional(),
    relationshipTypes: z.array(z.string().min(1)).optional(),
    trackNegativeCycles: z.boolean().optional(),
    trackPaths: z.boolean().optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('astar'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    sourceNode: z.number().int().nonnegative(),
    targetNode: z.number().int().nonnegative().optional(),
    targetNodes: z.array(z.number().int().nonnegative()).optional(),
    weightProperty: z.string().min(1).optional(),
    direction: z.enum(['incoming', 'outgoing']).optional(),
    relationshipTypes: z.array(z.string().min(1)).optional(),
    heuristic: z.enum(['manhattan', 'euclidean', 'haversine']).optional(),
    concurrency: z.number().int().positive().optional(),
  }),
  AlgorithmsBase.extend({
    op: z.literal('delta_stepping'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    sourceNode: z.number().int().nonnegative(),
    delta: z.number().positive().optional(),
    weightProperty: z.string().min(1).optional(),
    direction: z.enum(['incoming', 'outgoing']).optional(),
    relationshipTypes: z.array(z.string().min(1)).optional(),
    concurrency: z.number().int().positive().optional(),
  }),
  AlgorithmsBase.extend({
    op: z.literal('yens'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    sourceNode: z.number().int().nonnegative(),
    targetNode: z.number().int().nonnegative(),
    k: z.number().int().positive().optional(),
    weightProperty: z.string().min(1).optional(),
    direction: z.enum(['incoming', 'outgoing']).optional(),
    relationshipTypes: z.array(z.string().min(1)).optional(),
    concurrency: z.number().int().positive().optional(),
  }),
  AlgorithmsBase.extend({
    op: z.literal('all_shortest_paths'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    weighted: z.boolean().optional(),
    weightProperty: z.string().min(1).optional(),
    direction: z.enum(['incoming', 'outgoing', 'undirected']).optional(),
    relationshipTypes: z.array(z.string().min(1)).optional(),
    maxResults: z.number().int().positive().optional(),
    concurrency: z.number().int().positive().optional(),
  }),
  AlgorithmsBase.extend({
    op: z.literal('spanning_tree'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    startNode: z.number().int().nonnegative().optional(),
    computeMinimum: z.boolean().optional(),
    weightProperty: z.string().min(1).optional(),
    direction: z.enum(['incoming', 'outgoing', 'undirected']).optional(),
    relationshipTypes: z.array(z.string().min(1)).optional(),
    concurrency: z.number().int().positive().optional(),
  }),
  AlgorithmsBase.extend({
    op: z.literal('topological_sort'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    computeMaxDistance: z.boolean().optional(),
    concurrency: z.number().int().positive().optional(),
  }),
  AlgorithmsBase.extend({
    op: z.literal('random_walk'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    walksPerNode: z.number().int().positive().optional(),
    walkLength: z.number().int().positive().optional(),
    returnFactor: z.number().positive().optional(),
    inOutFactor: z.number().positive().optional(),
    sourceNodes: z.array(z.number().int().nonnegative()).optional(),
    randomSeed: z.number().int().nonnegative().optional(),
    concurrency: z.number().int().positive().optional(),
  }),
  AlgorithmsBase.extend({
    op: z.literal('dag_longest_path'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    concurrency: z.number().int().positive().optional(),
  }),
  AlgorithmsBase.extend({
    op: z.literal('steiner_tree'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    sourceNodes: z.array(z.number().int().nonnegative()).optional(),
    weightProperty: z.string().min(1).optional(),
    direction: z.enum(['incoming', 'outgoing', 'undirected']).optional(),
    relationshipTypes: z.array(z.string().min(1)).optional(),
    concurrency: z.number().int().positive().optional(),
  }),
  AlgorithmsBase.extend({
    op: z.literal('prize_collecting_steiner_tree'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    prizes: z.array(z.number()),
    weightProperty: z.string().min(1).optional(),
    relationshipTypes: z.array(z.string().min(1)).optional(),
    concurrency: z.number().int().positive().optional(),
  }),
  // ==========================================================================
  // Centrality Algorithms
  // ==========================================================================
  AlgorithmsBase.extend({
    op: z.literal('degree_centrality'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    normalize: z.boolean().optional(),
    orientation: z.enum(['incoming', 'outgoing', 'undirected']).optional(),
    weighted: z.boolean().optional(),
    weightProperty: z.string().min(1).optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('pagerank'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    direction: z.enum(['incoming', 'outgoing', 'both']).optional(),
    iterations: z.number().int().positive().optional(),
    dampingFactor: z.number().min(0).max(1).optional(),
    tolerance: z.number().positive().optional(),
    sourceNodes: z.array(z.number().int().nonnegative()).optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('betweenness'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    direction: z.enum(['incoming', 'outgoing', 'both']).optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('closeness'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    direction: z.enum(['incoming', 'outgoing', 'both']).optional(),
    useWasserman: z.boolean().optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('harmonic'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    direction: z.enum(['incoming', 'outgoing', 'both']).optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('hits'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    direction: z.enum(['incoming', 'outgoing', 'both']).optional(),
    iterations: z.number().int().positive().optional(),
    tolerance: z.number().positive().optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('articulation_points'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('bridges'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('celf'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    seedSetSize: z.number().int().positive().optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  // ==========================================================================
  // Similarity Algorithms
  // ==========================================================================
  AlgorithmsBase.extend({
    op: z.literal('knn'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    nodeProperties: z.array(z.string().min(1)),
    topK: z.number().int().positive(),
    sampleRate: z.number().min(0).max(1).optional(),
    perturbationRate: z.number().min(0).max(1).optional(),
    maxIterations: z.number().int().positive().optional(),
    similarityCutoff: z.number().min(0).max(1).optional(),
    degreeCutoff: z.number().int().nonnegative().optional(),
    randomSeed: z.number().int().nonnegative().optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('node_similarity'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    degreeCutoff: z.number().int().nonnegative().optional(),
    similarityCutoff: z.number().min(0).max(1).optional(),
    upperDegreeCutoff: z.number().int().nonnegative().optional(),
    lowerDegreeCutoff: z.number().int().nonnegative().optional(),
    topK: z.number().int().positive().optional(),
    bottomK: z.number().int().positive().optional(),
    topN: z.number().int().positive().optional(),
    bottomN: z.number().int().positive().optional(),
    concurrency: z.number().int().positive().optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('filtered_knn'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    nodeProperties: z.array(z.string().min(1)),
    topK: z.number().int().positive(),
    sampleRate: z.number().min(0).max(1).optional(),
    perturbationRate: z.number().min(0).max(1).optional(),
    maxIterations: z.number().int().positive().optional(),
    similarityCutoff: z.number().min(0).max(1).optional(),
    degreeCutoff: z.number().int().nonnegative().optional(),
    randomSeed: z.number().int().nonnegative().optional(),
    concurrency: z.number().int().positive().optional(),
    sourceNodeLabel: z.string().min(1).optional(),
    targetNodeLabel: z.string().min(1).optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('filtered_node_similarity'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    degreeCutoff: z.number().int().nonnegative().optional(),
    similarityCutoff: z.number().min(0).max(1).optional(),
    upperDegreeCutoff: z.number().int().nonnegative().optional(),
    lowerDegreeCutoff: z.number().int().nonnegative().optional(),
    topK: z.number().int().positive().optional(),
    bottomK: z.number().int().positive().optional(),
    topN: z.number().int().positive().optional(),
    bottomN: z.number().int().positive().optional(),
    concurrency: z.number().int().positive().optional(),
    sourceNodeLabel: z.string().min(1).optional(),
    targetNodeLabel: z.string().min(1).optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  // ==========================================================================
  // Node Embeddings Algorithms
  // ==========================================================================
  AlgorithmsBase.extend({
    op: z.literal('graph_sage'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    nodeProperties: z.array(z.string().min(1)).optional(),
    embeddingDimension: z.number().int().positive(),
    aggregator: z.enum(['mean', 'pool', 'lstm']).optional(),
    activationFunction: z.enum(['relu', 'sigmoid', 'tanh']).optional(),
    batchSize: z.number().int().positive().optional(),
    epochs: z.number().int().positive().optional(),
    learningRate: z.number().positive().optional(),
    sampleSizes: z.array(z.number().int().positive()).optional(),
    negativeSampleWeight: z.number().positive().optional(),
    randomSeed: z.number().int().nonnegative().optional(),
    concurrency: z.number().int().positive().optional(),
    modelName: z.string().min(1).optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('node2vec'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    embeddingDimension: z.number().int().positive(),
    walkLength: z.number().int().positive().optional(),
    returnParam: z.number().positive().optional(),
    inOutParam: z.number().positive().optional(),
    walksPerNode: z.number().int().positive().optional(),
    randomSeed: z.number().int().nonnegative().optional(),
    concurrency: z.number().int().positive().optional(),
    modelName: z.string().min(1).optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('fastrp'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    nodeProperties: z.array(z.string().min(1)).optional(),
    embeddingDimension: z.number().int().positive(),
    iterationWeights: z.array(z.number()).optional(),
    normalizationStrength: z.number().positive().optional(),
    randomSeed: z.number().int().nonnegative().optional(),
    concurrency: z.number().int().positive().optional(),
    modelName: z.string().min(1).optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('hash_gnn'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    nodeProperties: z.array(z.string().min(1)).optional(),
    embeddingDimension: z.number().int().positive(),
    neighborInfluence: z.number().min(0).max(1).optional(),
    randomSeed: z.number().int().nonnegative().optional(),
    concurrency: z.number().int().positive().optional(),
    modelName: z.string().min(1).optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
  AlgorithmsBase.extend({
    op: z.literal('gat'),
    mode: AlgorithmModeSchema,
    graphName: GdsGraphNameSchema,
    nodeProperties: z.array(z.string().min(1)).optional(),
    embeddingDimension: z.number().int().positive(),
    heads: z.number().int().positive().optional(),
    attentionDropout: z.number().min(0).max(1).optional(),
    featureDropout: z.number().min(0).max(1).optional(),
    epochs: z.number().int().positive().optional(),
    learningRate: z.number().positive().optional(),
    batchSize: z.number().int().positive().optional(),
    randomSeed: z.number().int().nonnegative().optional(),
    concurrency: z.number().int().positive().optional(),
    modelName: z.string().min(1).optional(),
    estimateSubmode: EstimateSubmodeSchema,
  }),
]);
export type GdsAlgorithmsCall = z.infer<typeof GdsAlgorithmsCallSchema>;
