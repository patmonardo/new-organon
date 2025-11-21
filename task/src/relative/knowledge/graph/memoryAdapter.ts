// In-memory GraphPort implementation. Suitable for tests and local runs.
// Replace with an @organon/gds adapter when ready.

import type { GraphPort, GraphQuery, NodeId, EdgeId } from './port';
import type {
  KnowledgeNode,
  KnowledgeEdge,
  KnowledgeGraph,
  KnowledgeUnit,
  TopicModel,
} from '../../schema';

// Type guard to refine unknown to string[] safely
const isStringArray = (v: unknown): v is string[] =>
  Array.isArray(v) && v.every((t) => typeof t === 'string');

export class InMemoryGraphAdapter implements GraphPort {
  private nodes = new Map<NodeId, KnowledgeNode>();
  private edges = new Map<EdgeId, KnowledgeEdge>();
  private graphs = new Map<string, KnowledgeGraph>();
  private kuByNode = new Map<NodeId, KnowledgeUnit[]>();
  private topicByGraph = new Map<string, TopicModel[]>();

  upsertNode(node: KnowledgeNode): Promise<KnowledgeNode> {
    this.nodes.set(node.id, node);
    return Promise.resolve(node);
  }

  getNode(id: NodeId): Promise<KnowledgeNode | undefined> {
    return Promise.resolve(this.nodes.get(id));
  }

  deleteNode(id: NodeId): Promise<boolean> {
    return Promise.resolve(this.nodes.delete(id));
  }

  upsertEdge(edge: KnowledgeEdge): Promise<KnowledgeEdge> {
    this.edges.set(edge.id, edge);
    return Promise.resolve(edge);
  }

  getEdge(id: EdgeId): Promise<KnowledgeEdge | undefined> {
    return Promise.resolve(this.edges.get(id));
  }

  deleteEdge(id: EdgeId): Promise<boolean> {
    return Promise.resolve(this.edges.delete(id));
  }

  neighbors(id: NodeId, relation?: string): Promise<KnowledgeNode[]> {
    const out = Array.from(this.edges.values()).filter(
      (e) => e.source === id && (!relation || e.relation === relation),
    );
    const nodes = out
      .map((e) => this.nodes.get(e.target))
      .filter((n): n is KnowledgeNode => Boolean(n));
    return Promise.resolve(nodes);
  }

  query(q: GraphQuery): Promise<KnowledgeNode[]> {
    const all = Array.from(this.nodes.values());
    const filtered = all.filter((n) => {
      if (q.nodeType && n.type !== q.nodeType) return false;
      if (
        q.labelContains &&
        !n.label.toLowerCase().includes(q.labelContains.toLowerCase())
      )
        return false;
      if (q.withTag) {
        const md: unknown = (n as { metadata?: unknown }).metadata;
        const tagsUnknown: unknown = (md as Record<string, unknown> | undefined)
          ?.tags;
        if (!isStringArray(tagsUnknown)) {
          return false;
        }
        if (!tagsUnknown.includes(q.withTag)) return false;
      }
      return true;
    });
    return Promise.resolve(filtered);
  }

  attachKnowledgeUnit(nodeId: NodeId, ku: KnowledgeUnit): Promise<void> {
    const arr = this.kuByNode.get(nodeId) ?? [];
    arr.push(ku);
    this.kuByNode.set(nodeId, arr);
    return Promise.resolve();
  }

  attachTopicModel(graphId: string, topic: TopicModel): Promise<void> {
    const arr = this.topicByGraph.get(graphId) ?? [];
    arr.push(topic);
    this.topicByGraph.set(graphId, arr);
    return Promise.resolve();
  }

  getGraph(graphId: string): Promise<KnowledgeGraph | undefined> {
    return Promise.resolve(this.graphs.get(graphId));
  }

  setGraph(graph: KnowledgeGraph): Promise<void> {
    this.graphs.set(graph.id, graph);
    // Also hydrate nodes/edges maps for convenience
    graph.nodes.forEach((n) => this.nodes.set(n.id, n));
    graph.edges.forEach((e) => this.edges.set(e.id, e));
    return Promise.resolve();
  }
}

export default InMemoryGraphAdapter;
