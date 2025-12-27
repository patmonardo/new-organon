/**
 * ML Algorithm Dispatchers
 *
 * High-level dispatchers for ML algorithms that orchestrate calls to GDS
 * through the GDSL protocol. These provide a clean TypeScript API for
 * similarity and embeddings algorithms.
 */

import { invokeGdsApplicationCall, type KernelPort, type GdsAlgorithmsCall } from '@organon/gdsl';

// ============================================================================
// Similarity Algorithms
// ============================================================================

export interface SimilarityConfig {
	graphName: string;
	nodeProperties?: string[];
	topK?: number;
	sampleRate?: number;
	perturbationRate?: number;
	maxIterations?: number;
	similarityCutoff?: number;
	degreeCutoff?: number;
	randomSeed?: number;
	concurrency?: number;
	sourceNodeLabel?: string;
	targetNodeLabel?: string;
}

export interface NodeSimilarityConfig {
	graphName: string;
	degreeCutoff?: number;
	similarityCutoff?: number;
	upperDegreeCutoff?: number;
	lowerDegreeCutoff?: number;
	topK?: number;
	bottomK?: number;
	topN?: number;
	bottomN?: number;
	concurrency?: number;
	sourceNodeLabel?: string;
	targetNodeLabel?: string;
}

export class SimilarityDispatcher {
	constructor(private port: KernelPort) {}

	async runKNN(config: SimilarityConfig, mode: 'stream' | 'stats' | 'mutate' | 'write' = 'stream') {
		const call: GdsAlgorithmsCall = {
			facade: 'algorithms',
			op: 'knn',
			mode,
			user: { username: 'default', isAdmin: false },
			databaseId: 'default',
			graphName: config.graphName,
			nodeProperties: config.nodeProperties || [],
			topK: config.topK || 10,
			sampleRate: config.sampleRate,
			perturbationRate: config.perturbationRate,
			maxIterations: config.maxIterations,
			similarityCutoff: config.similarityCutoff,
			degreeCutoff: config.degreeCutoff,
			randomSeed: config.randomSeed,
			concurrency: config.concurrency,
		};

		return await invokeGdsApplicationCall(this.port, call);
	}

	async runNodeSimilarity(config: NodeSimilarityConfig, mode: 'stream' | 'stats' | 'mutate' | 'write' = 'stream') {
		const call: GdsAlgorithmsCall = {
			facade: 'algorithms',
			op: 'node_similarity',
			mode,
			user: { username: 'default', isAdmin: false },
			databaseId: 'default',
			graphName: config.graphName,
			degreeCutoff: config.degreeCutoff,
			similarityCutoff: config.similarityCutoff,
			upperDegreeCutoff: config.upperDegreeCutoff,
			lowerDegreeCutoff: config.lowerDegreeCutoff,
			topK: config.topK,
			bottomK: config.bottomK,
			topN: config.topN,
			bottomN: config.bottomN,
			concurrency: config.concurrency,
		};

		return await invokeGdsApplicationCall(this.port, call);
	}

	async runFilteredKNN(config: SimilarityConfig, mode: 'stream' | 'stats' | 'mutate' | 'write' = 'stream') {
		const call: GdsAlgorithmsCall = {
			facade: 'algorithms',
			op: 'filtered_knn',
			mode,
			user: { username: 'default', isAdmin: false },
			databaseId: 'default',
			graphName: config.graphName,
			nodeProperties: config.nodeProperties || [],
			topK: config.topK || 10,
			sampleRate: config.sampleRate,
			perturbationRate: config.perturbationRate,
			maxIterations: config.maxIterations,
			similarityCutoff: config.similarityCutoff,
			degreeCutoff: config.degreeCutoff,
			randomSeed: config.randomSeed,
			concurrency: config.concurrency,
			sourceNodeLabel: config.sourceNodeLabel,
			targetNodeLabel: config.targetNodeLabel,
		};

		return await invokeGdsApplicationCall(this.port, call);
	}

	async runFilteredNodeSimilarity(config: NodeSimilarityConfig, mode: 'stream' | 'stats' | 'mutate' | 'write' = 'stream') {
		const call: GdsAlgorithmsCall = {
			facade: 'algorithms',
			op: 'filtered_node_similarity',
			mode,
			user: { username: 'default', isAdmin: false },
			databaseId: 'default',
			graphName: config.graphName,
			degreeCutoff: config.degreeCutoff,
			similarityCutoff: config.similarityCutoff,
			upperDegreeCutoff: config.upperDegreeCutoff,
			lowerDegreeCutoff: config.lowerDegreeCutoff,
			topK: config.topK,
			bottomK: config.bottomK,
			topN: config.topN,
			bottomN: config.bottomN,
			concurrency: config.concurrency,
			sourceNodeLabel: config.sourceNodeLabel,
			targetNodeLabel: config.targetNodeLabel,
		};

		return await invokeGdsApplicationCall(this.port, call);
	}
}

// ============================================================================
// Node Embeddings Algorithms
// ============================================================================

export interface EmbeddingConfig {
	graphName: string;
	nodeProperties?: string[];
	embeddingDimension: number;
	randomSeed?: number;
	concurrency?: number;
	modelName?: string;
}

export interface GraphSageConfig extends EmbeddingConfig {
	aggregator?: 'mean' | 'pool' | 'lstm';
	activationFunction?: 'relu' | 'sigmoid' | 'tanh';
	batchSize?: number;
	epochs?: number;
	learningRate?: number;
	sampleSizes?: number[];
	negativeSampleWeight?: number;
}

export interface Node2VecConfig extends EmbeddingConfig {
	walkLength?: number;
	returnParam?: number;
	inOutParam?: number;
	walksPerNode?: number;
}

export interface FastRPConfig extends EmbeddingConfig {
	iterationWeights?: number[];
	normalizationStrength?: number;
}

export interface HashGNNConfig extends EmbeddingConfig {
	neighborInfluence?: number;
}

export interface GATConfig extends EmbeddingConfig {
	heads?: number;
	attentionDropout?: number;
	featureDropout?: number;
	epochs?: number;
	learningRate?: number;
	batchSize?: number;
}

export class NodeEmbeddingsDispatcher {
	constructor(private port: KernelPort) {}

	async runGraphSage(config: GraphSageConfig, mode: 'stream' | 'stats' | 'mutate' | 'write' | 'train' = 'train') {
		const call: GdsAlgorithmsCall = {
			facade: 'algorithms',
			op: 'graph_sage',
			mode,
			user: { username: 'default', isAdmin: false },
			databaseId: 'default',
			graphName: config.graphName,
			nodeProperties: config.nodeProperties,
			embeddingDimension: config.embeddingDimension,
			aggregator: config.aggregator,
			activationFunction: config.activationFunction,
			batchSize: config.batchSize,
			epochs: config.epochs,
			learningRate: config.learningRate,
			sampleSizes: config.sampleSizes,
			negativeSampleWeight: config.negativeSampleWeight,
			randomSeed: config.randomSeed,
			concurrency: config.concurrency,
			modelName: config.modelName,
		};

		return await invokeGdsApplicationCall(this.port, call);
	}

	async runNode2Vec(config: Node2VecConfig, mode: 'stream' | 'stats' | 'mutate' | 'write' | 'train' = 'train') {
		const call: GdsAlgorithmsCall = {
			facade: 'algorithms',
			op: 'node2vec',
			mode,
			user: { username: 'default', isAdmin: false },
			databaseId: 'default',
			graphName: config.graphName,
			embeddingDimension: config.embeddingDimension,
			walkLength: config.walkLength,
			returnParam: config.returnParam,
			inOutParam: config.inOutParam,
			walksPerNode: config.walksPerNode,
			randomSeed: config.randomSeed,
			concurrency: config.concurrency,
			modelName: config.modelName,
		};

		return await invokeGdsApplicationCall(this.port, call);
	}

	async runFastRP(config: FastRPConfig, mode: 'stream' | 'stats' | 'mutate' | 'write' | 'train' = 'train') {
		const call: GdsAlgorithmsCall = {
			facade: 'algorithms',
			op: 'fastrp',
			mode,
			user: { username: 'default', isAdmin: false },
			databaseId: 'default',
			graphName: config.graphName,
			nodeProperties: config.nodeProperties,
			embeddingDimension: config.embeddingDimension,
			iterationWeights: config.iterationWeights,
			normalizationStrength: config.normalizationStrength,
			randomSeed: config.randomSeed,
			concurrency: config.concurrency,
			modelName: config.modelName,
		};

		return await invokeGdsApplicationCall(this.port, call);
	}

	async runHashGNN(config: HashGNNConfig, mode: 'stream' | 'stats' | 'mutate' | 'write' | 'train' = 'train') {
		const call: GdsAlgorithmsCall = {
			facade: 'algorithms',
			op: 'hash_gnn',
			mode,
			user: { username: 'default', isAdmin: false },
			databaseId: 'default',
			graphName: config.graphName,
			nodeProperties: config.nodeProperties,
			embeddingDimension: config.embeddingDimension,
			neighborInfluence: config.neighborInfluence,
			randomSeed: config.randomSeed,
			concurrency: config.concurrency,
			modelName: config.modelName,
		};

		return await invokeGdsApplicationCall(this.port, call);
	}

	async runGAT(config: GATConfig, mode: 'stream' | 'stats' | 'mutate' | 'write' | 'train' = 'train') {
		const call: GdsAlgorithmsCall = {
			facade: 'algorithms',
			op: 'gat',
			mode,
			user: { username: 'default', isAdmin: false },
			databaseId: 'default',
			graphName: config.graphName,
			nodeProperties: config.nodeProperties,
			embeddingDimension: config.embeddingDimension,
			heads: config.heads,
			attentionDropout: config.attentionDropout,
			featureDropout: config.featureDropout,
			epochs: config.epochs,
			learningRate: config.learningRate,
			batchSize: config.batchSize,
			randomSeed: config.randomSeed,
			concurrency: config.concurrency,
			modelName: config.modelName,
		};

		return await invokeGdsApplicationCall(this.port, call);
	}
}

// ============================================================================
// Unified ML Dispatcher
// ============================================================================

export class MLDispatcher {
	similarity: SimilarityDispatcher;
	embeddings: NodeEmbeddingsDispatcher;

	constructor(port: KernelPort) {
		this.similarity = new SimilarityDispatcher(port);
		this.embeddings = new NodeEmbeddingsDispatcher(port);
	}
}
