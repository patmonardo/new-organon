// Graph Port scoped under <knowledge> namespace
// Keep tiny and swappable: @organon/gds will provide production adapters.

import type {
  KnowledgeNode,
  KnowledgeEdge,
  KnowledgeGraph,
  KnowledgeUnit,
  TopicModel,
} from '../../schema';

export type NodeId = string;
export type EdgeId = string;

export interface GraphQuery {
  nodeType?: KnowledgeNode['type'];
  labelContains?: string;
  withTag?: string;
}

export interface GraphPort {
  upsertNode(node: KnowledgeNode): Promise<KnowledgeNode>;
  getNode(id: NodeId): Promise<KnowledgeNode | undefined>;
  deleteNode(id: NodeId): Promise<boolean>;

  upsertEdge(edge: KnowledgeEdge): Promise<KnowledgeEdge>;
  getEdge(id: EdgeId): Promise<KnowledgeEdge | undefined>;
  deleteEdge(id: EdgeId): Promise<boolean>;

  neighbors(id: NodeId, relation?: string): Promise<KnowledgeNode[]>;
  query(q: GraphQuery): Promise<KnowledgeNode[]>;

  attachKnowledgeUnit(nodeId: NodeId, ku: KnowledgeUnit): Promise<void>;
  attachTopicModel(graphId: string, topic: TopicModel): Promise<void>;

  getGraph(graphId: string): Promise<KnowledgeGraph | undefined>;
  setGraph(graph: KnowledgeGraph): Promise<void>;
}

export default GraphPort;
