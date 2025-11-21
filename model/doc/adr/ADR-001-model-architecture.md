# ADR-001: @model Package Architecture and Role

## Status
Accepted

## Context

The ORGANON platform implements a triadic dialectical architecture (BEC-MVC-TAW) for the Science of Action. The `@model` package is the MVC layer, mediating between pure logic (`@logic`/BEC) and practical synthesis (`@task`/TAW). It transforms universal, noumenal forms into experiential, empirical structures, serving as the Knowledge App Processor and the empirical foundation for applied ontology.

## Decision

- **@model** will embody the **MVC Layer (Active Logic of Experience)**, mapping and transforming the pure forms of `@logic` (Being, Essence, Concept) into actionable, experiential forms (Model, View, Controller).
- The package will serve as the **middleware and extension point** for Knowledge Apps, GUI adapters, and empirical modeling.
- **Schemas in `@model/src/schema/`** will be defined as dialectical extensions of their counterparts in `@logic`, maintaining isomorphic structure but as piecewise opposites.
- **Sublated Dyads** (resolved dualities) will be formalized to clarify the relationships between BEC and MVC, and to enable practical synthesis in `@task`.
- The architecture will support plugin systems and middleware, allowing for future extensibility and integration with GUI and workflow components.

## Consequences

- **@model** becomes the central hub for empirical modeling, GUI adaptation, and middleware, bridging pure logic and practical action.
- The dialectical mapping ensures that every schema in `@model` is both a transformation of a logical form and a foundation for practical synthesis.
- The package will facilitate the development of Knowledge Apps that operate on the principles of dialectical logic, empirical modeling, and action-oriented transformation.
- Future expansion will focus on plugin systems, middleware integration, and the formalization of sublated dyads and triads.

## Rationale

This decision aligns with the ORGANON platform’s core innovation: **MVC is BEC** — the same logical structure made experiential through action-driven transformation. By formalizing the role of `@model` as the mediator and transformer, we ensure systematic unity and extensibility across the platform.

## References

- [ORGANON README](../../README.md)
- [Hegelian Dialectics](https://en.wikipedia.org/wiki/Dialectic)
- [ADR Pattern](https://adr.github.io/)

---

*This ADR documents the foundational architecture and role of the @model package within the ORGANON Science of Action