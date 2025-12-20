/*
 * GENERATED FILE — DO NOT EDIT.
 *
 * Source: src/tools/codegen-dialectic-registry.ts
 *
 * This registry links Dialectic IR ("true IR") modules into the active form layer
 * by providing a stable lookup table consumable from relative/form.
 */

import type { DialecticIR } from '@schema/dialectic';

export type DialecticIRRegistryEntry = {
  key: string;
  id: string;
  title: string;
  section: string;
  module: string;
  export: string;
};

export const dialecticIRRegistryEntries: DialecticIRRegistryEntry[] = [
  { key: '@relative/being/quality/being-for-self/attraction-ir#attractionIR', id: 'attraction-ir', title: "Repulsion and Attraction IR: Exclusion, Attraction, Connection, Quantity", section: "I. BEING - A. QUALITY - C. Being-for-Self - C. Repulsion and Attraction", module: '@relative/being/quality/being-for-self/attraction-ir', export: 'attractionIR' },
  { key: '@relative/being/quality/being-for-self/being-for-self-ir#beingForSelfIR', id: 'being-for-self-ir', title: "Being-for-Self IR: Being-for-Itself, Being-for-One, The One", section: "I. BEING - A. QUALITY - C. Being-for-Self - A. Being-for-Itself as Such", module: '@relative/being/quality/being-for-self/being-for-self-ir', export: 'beingForSelfIR' },
  { key: '@relative/being/quality/being-for-self/one-many-ir#oneManyIR', id: 'one-many-ir', title: "One and Many IR: The One Within, The One and The Void, Many Ones", section: "I. BEING - A. QUALITY - C. Being-for-Self - B. The One and The Many", module: '@relative/being/quality/being-for-self/one-many-ir', export: 'oneManyIR' },
  { key: '@relative/being/quality/being/becoming-ir#becomingIR', id: 'becoming-ir', title: "Becoming IR: Unity of Being and Nothing", section: "C. BECOMING", module: '@relative/being/quality/being/becoming-ir', export: 'becomingIR' },
  { key: '@relative/being/quality/being/being-ir#beingIR', id: 'being-ir', title: "Being IR: Pure Being", section: "A. BEING", module: '@relative/being/quality/being/being-ir', export: 'beingIR' },
  { key: '@relative/being/quality/being/nothing-ir#nothingIR', id: 'nothing-ir', title: "Nothing IR: Pure Nothing", section: "B. NOTHING", module: '@relative/being/quality/being/nothing-ir', export: 'nothingIR' },
  { key: '@relative/being/quality/existence/affirmative-infinity-ir#affirmativeInfinityIR', id: 'affirmative-infinity-ir', title: "Affirmative Infinity IR: Unity, True Infinite as Becoming, True Infinite as Being", section: "I. BEING - A. QUALITY - C. Infinity - C. Affirmative Infinity", module: '@relative/being/quality/existence/affirmative-infinity-ir', export: 'affirmativeInfinityIR' },
  { key: '@relative/being/quality/existence/alternating-infinity-ir#alternatingInfinityIR', id: 'alternating-infinity-ir', title: "Alternating Infinity IR: Bad Infinite, Contradiction, Progress to Infinity", section: "I. BEING - A. QUALITY - C. Infinity - B. Alternating Determination", module: '@relative/being/quality/existence/alternating-infinity-ir', export: 'alternatingInfinityIR' },
  { key: '@relative/being/quality/existence/constitution-ir#constitutionIR', id: 'constitution-ir', title: "Constitution IR: Determination, Constitution, and Limit", section: "B. FINITUDE (b) Determination, constitution, and limit", module: '@relative/being/quality/existence/constitution-ir', export: 'constitutionIR' },
  { key: '@relative/being/quality/existence/existence-ir#existenceIR', id: 'existence-ir', title: "Existence IR: Determinateness of Being", section: "A. Existence as Such", module: '@relative/being/quality/existence/existence-ir', export: 'existenceIR' },
  { key: '@relative/being/quality/existence/finitude-ir#finitudeIR', id: 'finitude-ir', title: "Finitude IR: Finitude", section: "B. FINITUDE (c) Finitude", module: '@relative/being/quality/existence/finitude-ir', export: 'finitudeIR' },
  { key: '@relative/being/quality/existence/something-and-other-ir#somethingAndOtherIR', id: 'something-and-other-ir', title: "Something and Other IR: Something and Other", section: "B. FINITUDE (a) Something and other", module: '@relative/being/quality/existence/something-and-other-ir', export: 'somethingAndOtherIR' },
  { key: '@relative/being/quantity/quantity/limiting-quantity-ir#limitingQuantityIR', id: 'limiting-quantity-ir', title: "Limiting Quantity IR: Discrete Magnitude as Quantum", section: "C. THE LIMITING OF QUANTITY", module: '@relative/being/quantity/quantity/limiting-quantity-ir', export: 'limitingQuantityIR' },
  { key: '@relative/being/quantity/quantity/magnitude-ir#magnitudeIR', id: 'magnitude-ir', title: "Magnitude IR: Continuous and Discrete Magnitude", section: "B. CONTINUOUS AND DISCRETE MAGNITUDE", module: '@relative/being/quantity/quantity/magnitude-ir', export: 'magnitudeIR' },
  { key: '@relative/being/quantity/quantity/pure-quantity-ir#pureQuantityIR', id: 'pure-quantity-ir', title: "Pure Quantity IR: Sublated Being-for-Itself", section: "A. PURE QUANTITY", module: '@relative/being/quantity/quantity/pure-quantity-ir', export: 'pureQuantityIR' },
  { key: '@relative/being/quantity/quantum/infinity-ir#infinityIR', id: 'infinity-ir', title: "Infinity IR: Quantitative Infinity and Return to Quality", section: "D. QUANTITATIVE INFINITY", module: '@relative/being/quantity/quantum/infinity-ir', export: 'infinityIR' },
  { key: '@relative/being/quantity/quantum/number-ir#numberIR', id: 'number-ir', title: "Number IR: Complete Positedness, Amount and Unit", section: "A. NUMBER", module: '@relative/being/quantity/quantum/number-ir', export: 'numberIR' },
  { key: '@relative/being/quantity/quantum/quantum-ir#quantumIR', id: 'quantum-ir', title: "Quantum IR: Extensive and Intensive Quantum", section: "C. QUANTUM - Extensive and Intensive", module: '@relative/being/quantity/quantum/quantum-ir', export: 'quantumIR' },
  { key: '@relative/being/quantity/ratio/inverse-ir#inverseIR', id: 'inverse-ir', title: "Inverse IR: The Inverse Ratio", section: "B. THE INVERSE RATIO", module: '@relative/being/quantity/ratio/inverse-ir', export: 'inverseIR' },
  { key: '@relative/being/quantity/ratio/powers-ir#powersIR', id: 'powers-ir', title: "Powers IR: The Ratio of Powers and Transition to Measure", section: "C. THE RATIO OF POWERS", module: '@relative/being/quantity/ratio/powers-ir', export: 'powersIR' },
  { key: '@relative/being/quantity/ratio/ratio-ir#ratioIR', id: 'ratio-ir', title: "Ratio IR: Quantitative Relation", section: "D. THE QUANTITATIVE RATIO", module: '@relative/being/quantity/ratio/ratio-ir', export: 'ratioIR' },
  { key: '@relative/concept/object/chemism/chemism-ir#chemismIR', id: 'chemism-ir', title: "Chemism IR: Three Syllogisms, Concept Liberated, Purpose", section: "C. THE CONCEPT - II. OBJECTIVITY - B. Chemism - C. Transition of Chemism", module: '@relative/concept/object/chemism/chemism-ir', export: 'chemismIR' },
  { key: '@relative/concept/object/chemism/object-ir#chemicalObjectIR', id: 'chemical-object-ir', title: "Chemical Object IR: Non-Indifference, Contradiction, Striving", section: "C. THE CONCEPT - II. OBJECTIVITY - B. Chemism - A. The Chemical Object", module: '@relative/concept/object/chemism/object-ir', export: 'chemicalObjectIR' },
  { key: '@relative/concept/object/chemism/process-ir#chemicalProcessIR', id: 'chemical-process-ir', title: "Chemical Process IR: Affinity, Neutral Product, Disjunctive Syllogism", section: "C. THE CONCEPT - II. OBJECTIVITY - B. Chemism - B. The Process", module: '@relative/concept/object/chemism/process-ir', export: 'chemicalProcessIR' },
  { key: '@relative/concept/object/mechanism/mechanism-ir#mechanismIR', id: 'mechanism-ir', title: "Mechanism IR: Center, Syllogistic Structure, Law, Chemism", section: "C. THE CONCEPT - II. OBJECTIVITY - A. Mechanism - C. Absolute Mechanism", module: '@relative/concept/object/mechanism/mechanism-ir', export: 'mechanismIR' },
  { key: '@relative/concept/object/mechanism/object-ir#mechanicalObjectIR', id: 'mechanical-object-ir', title: "Mechanical Object IR: Syllogism, Plurality, Contradiction", section: "C. THE CONCEPT - II. OBJECTIVITY - A. Mechanism - A. The Mechanical Object", module: '@relative/concept/object/mechanism/object-ir', export: 'mechanicalObjectIR' },
  { key: '@relative/concept/object/mechanism/process-ir#mechanicalProcessIR', id: 'mechanical-process-ir', title: "Mechanical Process IR: Formal, Real, Product (Center/Law)", section: "C. THE CONCEPT - II. OBJECTIVITY - A. Mechanism - B. The Mechanical Process", module: '@relative/concept/object/mechanism/process-ir', export: 'mechanicalProcessIR' },
  { key: '@relative/concept/object/teleology/means-ir#meansIR', id: 'means-ir', title: "Means IR: Middle Term, Mechanical Object, Totality", section: "C. THE CONCEPT - II. OBJECTIVITY - C. Teleology - B. The Means", module: '@relative/concept/object/teleology/means-ir', export: 'meansIR' },
  { key: '@relative/concept/object/teleology/realized-ir#realizedPurposeIR', id: 'realized-purpose-ir', title: "Realized Purpose IR: Mechanism Under Dominance, External Purposiveness, Inner Connection", section: "C. THE CONCEPT - II. OBJECTIVITY - C. Teleology - C. The Realized Purpose", module: '@relative/concept/object/teleology/realized-ir', export: 'realizedPurposeIR' },
  { key: '@relative/concept/object/teleology/teleology-ir#teleologyIR', id: 'teleology-ir', title: "Teleology IR: Subjective Purpose, Realization, Idea", section: "C. THE CONCEPT - II. OBJECTIVITY - C. Teleology - A. Subjective Purpose", module: '@relative/concept/object/teleology/teleology-ir', export: 'teleologyIR' },
  { key: '@relative/concept/subject/concept/particular-ir#particularIR', id: 'particular-ir', title: "Particular IR: Immanent Moment, Totality, Understanding, Transition", section: "C. THE CONCEPT - I. SUBJECTIVITY - A. The Concept - 2. The Particular", module: '@relative/concept/subject/concept/particular-ir', export: 'particularIR' },
  { key: '@relative/concept/subject/concept/singular-ir#singularIR', id: 'singular-ir', title: "Singular IR: Self-Mediation, Indissolubility, Actuality, Judgment", section: "C. THE CONCEPT - I. SUBJECTIVITY - A. The Concept - 3. The Singular", module: '@relative/concept/subject/concept/singular-ir', export: 'singularIR' },
  { key: '@relative/concept/subject/concept/universal-ir#universalIR', id: 'universal-ir', title: "Universal IR: Pure Concept, Negation of Negation, Creative Principle", section: "C. THE CONCEPT - I. SUBJECTIVITY - A. The Concept - 1. The Universal", module: '@relative/concept/subject/concept/universal-ir', export: 'universalIR' },
  { key: '@relative/concept/subject/judgment/concept-ir#conceptJudgmentIR', id: 'concept-judgment-ir', title: "Concept Judgment IR: Assertoric, Problematic, Apodictic, Syllogism", section: "C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - D. Judgment of the Concept", module: '@relative/concept/subject/judgment/concept-ir', export: 'conceptJudgmentIR' },
  { key: '@relative/concept/subject/judgment/existence-ir#existenceIR', id: 'existence-ir', title: "Existence IR: Positive, Negative, Infinite Judgment", section: "C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - A. Judgment of Existence", module: '@relative/concept/subject/judgment/existence-ir', export: 'existenceIR' },
  { key: '@relative/concept/subject/judgment/necessity-ir#necessityIR', id: 'necessity-ir', title: "Necessity IR: Categorical, Hypothetical, Disjunctive, Concept as Posited", section: "C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - C. Judgment of Necessity", module: '@relative/concept/subject/judgment/necessity-ir', export: 'necessityIR' },
  { key: '@relative/concept/subject/judgment/reflection-ir#reflectionIR', id: 'reflection-ir', title: "Reflection IR: Singular, Particular, Universal Judgment, Genus", section: "C. THE CONCEPT - I. SUBJECTIVITY - B. The Judgment - B. Judgment of Reflection", module: '@relative/concept/subject/judgment/reflection-ir', export: 'reflectionIR' },
  { key: '@relative/concept/subject/syllogism/existence-ir#existenceSyllogismIR', id: 'existence-syllogism-ir', title: "Existence Syllogism IR: S-P-U, P-S-U, S-U-P, U-U-U", section: "C. THE CONCEPT - I. SUBJECTIVITY - C. The Syllogism - A. Syllogism of Existence", module: '@relative/concept/subject/syllogism/existence-ir', export: 'existenceSyllogismIR' },
  { key: '@relative/concept/subject/syllogism/necessity-ir#necessitySyllogismIR', id: 'necessity-syllogism-ir', title: "Necessity Syllogism IR: Categorical, Hypothetical, Disjunctive, Objectivity", section: "C. THE CONCEPT - I. SUBJECTIVITY - C. The Syllogism - C. Syllogism of Necessity", module: '@relative/concept/subject/syllogism/necessity-ir', export: 'necessitySyllogismIR' },
  { key: '@relative/concept/subject/syllogism/reflection-ir#reflectionSyllogismIR', id: 'reflection-syllogism-ir', title: "Reflection Syllogism IR: Allness, Induction, Analogy", section: "C. THE CONCEPT - I. SUBJECTIVITY - C. The Syllogism - B. Syllogism of Reflection", module: '@relative/concept/subject/syllogism/reflection-ir', export: 'reflectionSyllogismIR' },
  { key: '@relative/essence/appearance/relation/force-expression-ir#forceExpressionIR', id: 'force-expression-ir', title: "Force-Expression IR: Force, Solicitation, Infinity", section: "B. APPEARANCE - 3. Essential Relation - c. Force and Expression", module: '@relative/essence/appearance/relation/force-expression-ir', export: 'forceExpressionIR' },
  { key: '@relative/essence/appearance/relation/outer-inner-ir#outerInnerIR', id: 'outer-inner-ir', title: "Outer-Inner IR: One Identity, Immediate Conversion, Actuality", section: "B. APPEARANCE - 3. Essential Relation - b. Outer and Inner", module: '@relative/essence/appearance/relation/outer-inner-ir', export: 'outerInnerIR' },
  { key: '@relative/essence/appearance/relation/whole-parts-ir#wholePartsIR', id: 'whole-parts-ir', title: "Whole-Parts IR: Essential Relation, Reciprocal Conditioning, Tautology", section: "B. APPEARANCE - 3. Essential Relation - a. Whole and Parts", module: '@relative/essence/appearance/relation/whole-parts-ir', export: 'wholePartsIR' },
  { key: '@relative/essence/appearance/thing/dissolution-ir#dissolutionIR', id: 'dissolution-ir', title: "Dissolution IR: Absolute Porosity, Interpenetration, Appearance", section: "B. APPEARANCE - 1. The Thing - c. Dissolution", module: '@relative/essence/appearance/thing/dissolution-ir', export: 'dissolutionIR' },
  { key: '@relative/essence/appearance/thing/matter-ir#matterIR', id: 'matter-ir', title: "Matter IR: Property to Matter, Porous Matter", section: "B. APPEARANCE - 1. The Thing - b. Matter", module: '@relative/essence/appearance/thing/matter-ir', export: 'matterIR' },
  { key: '@relative/essence/appearance/thing/thing-ir#thingIR', id: 'thing-ir', title: "Thing IR: Concrete Existence, Thing-in-Itself, Property", section: "ESSENCE - B. APPEARANCE - A. THE THING", module: '@relative/essence/appearance/thing/thing-ir', export: 'thingIR' },
  { key: '@relative/essence/appearance/world/disappearance-ir#disappearanceIR', id: 'disappearance-ir', title: "Disappearance IR: Opposition, Law Realized, World Foundered", section: "B. APPEARANCE - 2. The World - c. Dissolution of Appearance", module: '@relative/essence/appearance/world/disappearance-ir', export: 'disappearanceIR' },
  { key: '@relative/essence/appearance/world/law-ir#lawIR', id: 'law-ir', title: "Law IR: Appearance, Law, Kingdom of Laws", section: "B. APPEARANCE - 2. The World - b. Law of Appearance", module: '@relative/essence/appearance/world/law-ir', export: 'lawIR' },
  { key: '@relative/essence/appearance/world/world-ir#worldIR', id: 'world-ir', title: "World IR: Kingdom of Laws, Suprasensible World, Opposition", section: "B. APPEARANCE - 2. The World - a. World-In-Itself", module: '@relative/essence/appearance/world/world-ir', export: 'worldIR' },
  { key: '@relative/essence/reflection/essence/essence-ir#essenceIR', id: 'essence-ir', title: "Essence IR: Truth of Being, Absolute Negativity, Reflection", section: "ESSENCE - A. ESSENCE AS REFLECTION WITHIN ITSELF", module: '@relative/essence/reflection/essence/essence-ir', export: 'essenceIR' },
  { key: '@relative/essence/reflection/essence/reflection-ir#reflectionIR', id: 'reflection-ir', title: "Reflection IR: Essence as Reflection, Positing, External, Determining", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. Reflection", module: '@relative/essence/reflection/essence/reflection-ir', export: 'reflectionIR' },
  { key: '@relative/essence/reflection/essence/shine-ir#shineIR', id: 'shine-ir', title: "Shine IR: Being as Shine and Infinite Determinateness", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 1. Shine", module: '@relative/essence/reflection/essence/shine-ir', export: 'shineIR' },
  { key: '@relative/essence/reflection/foundation/contradiction-ir#contradictionIR', id: 'contradiction-ir', title: "Contradiction IR: Opposition, Posited Contradiction, Ground", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - C. Contradiction", module: '@relative/essence/reflection/foundation/contradiction-ir', export: 'contradictionIR' },
  { key: '@relative/essence/reflection/foundation/difference-ir#differenceIR', id: 'difference-ir', title: "Difference IR: Absolute Difference, Diversity, Opposition", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - B. Difference", module: '@relative/essence/reflection/foundation/difference-ir', export: 'differenceIR' },
  { key: '@relative/essence/reflection/foundation/identity-ir#identityIR', id: 'identity-ir', title: "Identity IR: Essence as Simple Self-Identity", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - A. Identity", module: '@relative/essence/reflection/foundation/identity-ir', export: 'identityIR' },
  { key: '@relative/essence/reflection/ground/absolute-ir#absoluteIR', id: 'absolute-ir', title: "Absolute Ground IR: Form, Matter, Content", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - a. Absolute Ground", module: '@relative/essence/reflection/ground/absolute-ir', export: 'absoluteIR' },
  { key: '@relative/essence/reflection/ground/condition-ir#conditionIR', id: 'condition-ir', title: "Condition IR: Unconditioned, Procession, Concrete Existence", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - c. Condition", module: '@relative/essence/reflection/ground/condition-ir', export: 'conditionIR' },
  { key: '@relative/essence/reflection/ground/determinate-ir#determinateIR', id: 'determinate-ir', title: "Determinate Ground IR: Formal, Real, Complete, Conditioning", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - b. Determinate Ground", module: '@relative/essence/reflection/ground/determinate-ir', export: 'determinateIR' },
];

export const dialecticIRRegistry: Record<string, DialecticIRRegistryEntry> = Object.fromEntries(
  dialecticIRRegistryEntries.map(e => [e.key, e])
);

export const dialecticIRIdIndex: Record<string, string[]> = {
  "absolute-ir": ["@relative/essence/reflection/ground/absolute-ir#absoluteIR"],
  "affirmative-infinity-ir": ["@relative/being/quality/existence/affirmative-infinity-ir#affirmativeInfinityIR"],
  "alternating-infinity-ir": ["@relative/being/quality/existence/alternating-infinity-ir#alternatingInfinityIR"],
  "attraction-ir": ["@relative/being/quality/being-for-self/attraction-ir#attractionIR"],
  "becoming-ir": ["@relative/being/quality/being/becoming-ir#becomingIR"],
  "being-for-self-ir": ["@relative/being/quality/being-for-self/being-for-self-ir#beingForSelfIR"],
  "being-ir": ["@relative/being/quality/being/being-ir#beingIR"],
  "chemical-object-ir": ["@relative/concept/object/chemism/object-ir#chemicalObjectIR"],
  "chemical-process-ir": ["@relative/concept/object/chemism/process-ir#chemicalProcessIR"],
  "chemism-ir": ["@relative/concept/object/chemism/chemism-ir#chemismIR"],
  "concept-judgment-ir": ["@relative/concept/subject/judgment/concept-ir#conceptJudgmentIR"],
  "condition-ir": ["@relative/essence/reflection/ground/condition-ir#conditionIR"],
  "constitution-ir": ["@relative/being/quality/existence/constitution-ir#constitutionIR"],
  "contradiction-ir": ["@relative/essence/reflection/foundation/contradiction-ir#contradictionIR"],
  "determinate-ir": ["@relative/essence/reflection/ground/determinate-ir#determinateIR"],
  "difference-ir": ["@relative/essence/reflection/foundation/difference-ir#differenceIR"],
  "disappearance-ir": ["@relative/essence/appearance/world/disappearance-ir#disappearanceIR"],
  "dissolution-ir": ["@relative/essence/appearance/thing/dissolution-ir#dissolutionIR"],
  "essence-ir": ["@relative/essence/reflection/essence/essence-ir#essenceIR"],
  "existence-ir": ["@relative/being/quality/existence/existence-ir#existenceIR","@relative/concept/subject/judgment/existence-ir#existenceIR"],
  "existence-syllogism-ir": ["@relative/concept/subject/syllogism/existence-ir#existenceSyllogismIR"],
  "finitude-ir": ["@relative/being/quality/existence/finitude-ir#finitudeIR"],
  "force-expression-ir": ["@relative/essence/appearance/relation/force-expression-ir#forceExpressionIR"],
  "identity-ir": ["@relative/essence/reflection/foundation/identity-ir#identityIR"],
  "infinity-ir": ["@relative/being/quantity/quantum/infinity-ir#infinityIR"],
  "inverse-ir": ["@relative/being/quantity/ratio/inverse-ir#inverseIR"],
  "law-ir": ["@relative/essence/appearance/world/law-ir#lawIR"],
  "limiting-quantity-ir": ["@relative/being/quantity/quantity/limiting-quantity-ir#limitingQuantityIR"],
  "magnitude-ir": ["@relative/being/quantity/quantity/magnitude-ir#magnitudeIR"],
  "matter-ir": ["@relative/essence/appearance/thing/matter-ir#matterIR"],
  "means-ir": ["@relative/concept/object/teleology/means-ir#meansIR"],
  "mechanical-object-ir": ["@relative/concept/object/mechanism/object-ir#mechanicalObjectIR"],
  "mechanical-process-ir": ["@relative/concept/object/mechanism/process-ir#mechanicalProcessIR"],
  "mechanism-ir": ["@relative/concept/object/mechanism/mechanism-ir#mechanismIR"],
  "necessity-ir": ["@relative/concept/subject/judgment/necessity-ir#necessityIR"],
  "necessity-syllogism-ir": ["@relative/concept/subject/syllogism/necessity-ir#necessitySyllogismIR"],
  "nothing-ir": ["@relative/being/quality/being/nothing-ir#nothingIR"],
  "number-ir": ["@relative/being/quantity/quantum/number-ir#numberIR"],
  "one-many-ir": ["@relative/being/quality/being-for-self/one-many-ir#oneManyIR"],
  "outer-inner-ir": ["@relative/essence/appearance/relation/outer-inner-ir#outerInnerIR"],
  "particular-ir": ["@relative/concept/subject/concept/particular-ir#particularIR"],
  "powers-ir": ["@relative/being/quantity/ratio/powers-ir#powersIR"],
  "pure-quantity-ir": ["@relative/being/quantity/quantity/pure-quantity-ir#pureQuantityIR"],
  "quantum-ir": ["@relative/being/quantity/quantum/quantum-ir#quantumIR"],
  "ratio-ir": ["@relative/being/quantity/ratio/ratio-ir#ratioIR"],
  "realized-purpose-ir": ["@relative/concept/object/teleology/realized-ir#realizedPurposeIR"],
  "reflection-ir": ["@relative/concept/subject/judgment/reflection-ir#reflectionIR","@relative/essence/reflection/essence/reflection-ir#reflectionIR"],
  "reflection-syllogism-ir": ["@relative/concept/subject/syllogism/reflection-ir#reflectionSyllogismIR"],
  "shine-ir": ["@relative/essence/reflection/essence/shine-ir#shineIR"],
  "singular-ir": ["@relative/concept/subject/concept/singular-ir#singularIR"],
  "something-and-other-ir": ["@relative/being/quality/existence/something-and-other-ir#somethingAndOtherIR"],
  "teleology-ir": ["@relative/concept/object/teleology/teleology-ir#teleologyIR"],
  "thing-ir": ["@relative/essence/appearance/thing/thing-ir#thingIR"],
  "universal-ir": ["@relative/concept/subject/concept/universal-ir#universalIR"],
  "whole-parts-ir": ["@relative/essence/appearance/relation/whole-parts-ir#wholePartsIR"],
  "world-ir": ["@relative/essence/appearance/world/world-ir#worldIR"],
};

export function getDialecticIRMeta(key: string): DialecticIRRegistryEntry | undefined {
  return dialecticIRRegistry[key];
}

export function getDialecticIRKeysById(id: string): string[] {
  return dialecticIRIdIndex[id] ?? [];
}

/**
 * Load a DialecticIR by registry key.
 *
 * Note: This uses dynamic import of the source module specifier recorded in the registry.
 * In built JS output, resolution depends on your bundler/aliasing strategy.
 */
export async function loadDialecticIR(key: string): Promise<DialecticIR> {
  const meta = getDialecticIRMeta(key);
  if (!meta) throw new Error('Unknown DialecticIR key: ' + key);
  const mod = await import(meta.module);
  const ir = (mod as any)[meta.export] as unknown;
  return ir as DialecticIR;
}
