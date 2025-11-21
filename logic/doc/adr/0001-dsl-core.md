# ADR 0001 — Organon Logic DSL (HLO Core)

Status
- Proposed (v0). Targeting Quality → Quantity → Measure workstream.

Context
- We extracted High-Level Operations (HLOs) from Hegel’s Logic (Essence + Concept).
- We need a persistent, executable representation: declarative, composable, provenance-rich.
- Execution model is a Dialectical Concept Processor Unit (D-CPU): one logical operation per cycle. No ALU; quantitative math delegated to a later math coprocessor (GPU-like).

Decision
- Adopt a minimal, typed, rule-based DSL with:
  - Core relational kernel: facts and rules over typed terms (Var | Sym | Str | Num).
  - Safe monotone base with stratified negation; built-in guards (eq/lt/…); aggregates (count/sum/min/max/avg).
  - Meta envelope per fact/rule for tags, annotations, and provenance.
  - Mapping from current clause DSL:
    - tag(X,"v") → fact tag(X,"v") or rule head tag/2.
    - assert(p(a,b)) → atom p(a,b) in rule body or fact.
    - annotate(target,{…}) → Meta.annotate on fact/rule.
    - relations { predicate, from, to } → binary fact predicate(from,to).

Why
- Keeps logic core small and analyzable; interoperates with Datalog/ASP/SMT backends.
- Matches Hegelian HLOs: first-class tagging, relations, and “posited” conditions as rules.
- Provenance and annotations remain orthogonal (no leakage into logical consequences).

Scope and Semantics
- Closed-world/stratified core for evaluation; explicit bridges to open-world graphs (RDF/SPARQL) as import/export (not during fixpoint).
- Aggregates are group-by over base relations; arithmetic via Builtin guards; bulk numeric ops can be offloaded later.
- Namespacing for domains (con:, judg:, qual:, quan:) via symbolic terms.

Consequences
- Pros: Simple, explainable, auditable; modular backends; direct mapping from HLOs.
- Cons: Non-monotonic/default reasoning needs ASP backend; higher-order encodings require defunctionalization or SMT.

Alternatives Considered
- Prolog (SLD): Expressive, but top‑down search and cut/operator overloading complicate determinism and audits.
- Pure ASP (clingo): Great for defaults/choice, but overkill for the monotone core; keep as optional backend.
- RDF/SPARQL-only: OWA and no recursion/aggregates (portable) but poor fit for fixpoint reasoning; use for exchange.
- KIF/Common Logic as primary: Rich, but heavyweight for daily execution; keep as an interchange/upper ontology.

Influences and References
- Datalog
  - Soufflé Datalog: https://souffle-lang.github.io/
  - Flix (Datalog + types/effects): https://flix.dev/
  - Differential Datalog (DDlog): https://github.com/vmware/differential-datalog
- ASP
  - clingo (Potassco): https://potassco.org/clingo/
- Prolog family
  - SWI‑Prolog: https://www.swi-prolog.org/
  - Mercury: https://mercurylang.org/
  - Logtalk: https://logtalk.org/
- CLP/SMT/ATP (logic engines, not CAS)
  - Z3: https://github.com/Z3Prover/z3
  - cvc5: https://cvc5.github.io/
  - E Prover: https://wwwlehre.dhbw-stuttgart.de/~sschulz/E/E.html
  - Isabelle/HOL: https://isabelle.in.tum.de/
  - Lean: https://leanprover.github.io/
- Rule engines / Equational reasoning
  - Drools (Rete): https://www.drools.org/
  - egg / egglog (e-graphs): https://egraphs-good.github.io/; https://github.com/egraphs-good/egglog
- Knowledge interchange
  - KIF: http://www-ksl.stanford.edu/knowledge-sharing/kif/
  - Common Logic (ISO/IEC 24707): https://common-logic.github.io/
  - PowerLoom: https://www.isi.edu/isd/LOOM/PowerLoom/
- RDF/SPARQL (exchange)
  - RDF 1.1 Semantics: https://www.w3.org/TR/rdf11-mt/
  - SPARQL 1.1 Query: https://www.w3.org/TR/sparql11-query/

Implementation Notes
- Core AST (implemented): logic/dsl/core.ts
  - Term, Atom, Fact, Rule, Program, Builtin guards, Aggregate literals, Meta.
- Domain helpers (planned/initial):
  - Quality/Quantity constructors in logic/dsl/qual_quan.ts.
- Engines:
  - v0: in‑memory semi‑naive Datalog evaluator (with stratified negation + aggregates).
  - v1: emit Soufflé/Clingo/Z3 from Program; choose backend per query.
- Planner
  - Route Builtin arithmetic and Aggregates to a math coprocessor (later).
  - Maintain provenance on derived facts (materialized view with meta).

Migration from clause DSL
- Parse existing HLO clauses:
  - tag(...) → fact tag/2 with Meta.provenance = { sourceChunk, sourceOp }.
  - assert(foo(...)) → fact foo/... or put as body literal in a new summarizing rule.
  - annotate(T,{…}) → attach to the corresponding fact/rule’s Meta.annotate.
  - relations → facts rel/2.
- Group per chunk/op into Programs; emit both Program JSON and backend code.

Open Questions
- Names and sorts: add type annotations to terms? (Recommended for Quantity/Measure.)
- Defaults/exceptions (e.g., “normally …”): incorporate ASP selectively.
- OWA integration: import RDF graphs into base facts; export derived tag/rel facts back.

Acceptance Criteria
- Program.validate() lists predicates and counts; QA ensures no unsafe constructs (e.g., recursion through negation).
- Round‑trip: HLO → Program → backend code → results → tags/relations persisted with provenance.
- Deterministic fixpoint on core fragments; reproducible traces via D-CPU runtime.

Appendix: File Map
- logic/dsl/core.ts — Core AST + constructors + validator.
- logic/dsl/qual_quan.ts — Domain constructors (Quality/Quantity).
- logic/dsl/examples — Mapping samples and quality/quantity examples.
- logic/doc/logic/concept/runtime.ts — D-CPU executor (HLO cycle machine).
