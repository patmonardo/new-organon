# FormDB, HyperStore, and “Sensation → Intuition”

This note pins down the client/server vocabulary used around the GDS kernel boundary.

## FormDB

**FormDB** is a specific kind of **KnowledgeGraph** in this system.

Practically:

- It is a **PropertyGraph** (nodes/relationships + properties).
- It is the *client-facing* “Form” store: what the client reads/writes/queries as “knowledge”.

## HyperStore

**HyperStore** is a special **GraphStore** conceptually:

- It embodies the FormDB KnowledgeStore **and** the FactStores.
- It is the unified storage substrate the kernel can persist into and stream from.

At the protocol level today, HyperStore is represented as **GraphStore addressability**:

- a graph is named (`graphName`) and placed in a catalog/store
- operations return references/handles instead of large payloads

## Sensation → Intuition (kernel perspective)

In your terms:

- **Manifolds of sensation**: algorithm streams, graph weights, ML pipelines, embeddings, GraphSAGE/GNN training signals.
- The kernel’s role is to *persist* these into a PropertyGraph as structured state:
  **a mapping from sensation → intuition**.

So what “flows out of the kernel” (over GDS-L) is typically not raw sensation, but:

- a **GraphStore handle/reference** (e.g. `graphName`)
- plus proof/witness/trace metadata describing how that state was produced

See also:

- `gdsl/doc/GDS-LINK.md` (GDS-L vs G-DSL split)
- `gdsl/src/schema/handles.ts` (explicit graph handle schema)


