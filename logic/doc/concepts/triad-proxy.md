# Triad Proxy — Small UserLand API over a Vast Core

Intent
- Expose a usable interface for the Relational Form Processor (Ring 1) while Ring 0 remains a scalable Graph/Data/Compute fabric.

Contract (UserLand)
- Repository<T>: get(id):T|null; create(doc):T; update(id, mutateFn, { expectedRevision? }):T; delete(id):boolean.
- EventBus: publish/subscribe for processor events.

Reality (Core)
- Ring 0 hosts ML/NLP pipelines, KV/feature stores, graph engines, schedulers/queues, vectors/ANN — a vast kernel.
- The Triad hides this behind small ports so processors remain simple, composable, and testable.

Principle
- Small API, vast core: processors stay focused on E/P/R; Core scales and evolves underneath without changing the UserLand contract.
```// filepath: /home/pat/VSCode/organon/logic/docs/concepts/triad-proxy.md
# Triad Proxy — Small UserLand API over a Vast Core

Intent
- Expose a usable interface for the Relational Form Processor (Ring 1) while Ring 0 remains a scalable Graph/Data/Compute fabric.

Contract (UserLand)
- Repository<T>: get(id):T|null; create(doc):T; update(id, mutateFn, { expectedRevision? }):T; delete(id):boolean.
- EventBus: publish/subscribe for processor events.

Reality (Core)
- Ring 0 hosts ML/NLP pipelines, KV/feature stores, graph engines, schedulers/queues, vectors/ANN — a vast kernel.
- The Triad hides this behind small ports so processors remain simple, composable, and testable.

Principle
- Small API, vast core: processors stay focused on E/P/R; Core scales and evolves underneath without
