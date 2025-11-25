/**
 * Logical Operations: The Judgment of Reflection
 *
 * The judgment of reflection shows the movement of determination in the subject,
 * progressing from singular through particular to universal, revealing the genus
 * and transitioning to the judgment of necessity.
 *
 * Dialectical Movement:
 * - Singular Judgment: "this is an essential universal"
 * - Particular Judgment: "some singulars are universal"
 * - Universal Judgment: "allness" → objective universality → genus
 *
 * This judgment reveals how the subject reflects into itself, moving from
 * immediate singularity through particularity to true universality (genus).
 */

import type { LogicalOperation } from '../../../types';

// ============================================================================
// INTRODUCTION: Universal Collected into Unity
// ============================================================================

export const reflOp1IntroductionUnity: LogicalOperation = {
    id: 'refl-op-1-introduction-unity',
    chunkId: 'refl-1-introduction-unity',
    label: 'Introduction: Universal Collected into Unity; Determinate Content; Relational Determination',
    clauses: [
      'subject = singularAsSuch',
      'universal != abstractUniversality',
      'universal != singularProperty',
      'universal = collectedIntoUnity',
      'universal.collected = throughConnection(differentTerms)',
      'universal = coalescing(manifoldProperties, concreteExistences)',
      'judgmentOfReflection.firstHas = determinateContent',
      'content = formDetermination',
      'content.reflected = intoIdentity',
      'content.distinct = fromForm',
      'judgmentOfExistence.content = immediate',
      'judgmentOfExistence.content = abstract',
      'judgmentOfExistence.content = indeterminate',
      'examples = humanIsMortal',
      'examples = thingsArePerishable',
      'examples = thingIsUseful',
      'examples = thingIsHarmful',
      'predicates.express = essentiality',
      'essentiality = relationalDetermination',
      'essentiality = comprehensiveUniversality',
      'universality.stillDistinct = fromConceptUniversality',
      'universality != abstractUniversalityOfQualitativeJudgment',
      'universality.hasConnection = toImmediate',
      'universality.hasBasis = negativity',
      'concept.determines = immediateExistence',
      'concept.determines = relationalDeterminations',
      'relationalDeterminations.extend = across(diverseMultiplicity)',
      'trueUniversal = innerEssence(multiplicity)',
      'trueUniversal.in = sphereOfAppearance',
      'relativeNature != element(multiplicity)',
    ],
    predicates: [
      { name: 'collectedIntoUnity', args: ['universal'] },
      { name: 'coalescing', args: ['manifoldProperties', 'concreteExistences'] },
      { name: 'determinateContent', args: ['judgmentOfReflection'] },
      { name: 'relationalDetermination', args: ['essentiality'] },
      { name: 'comprehensiveUniversality', args: ['essentiality'] },
      { name: 'innerEssence', args: ['multiplicity'] },
    ],
    relations: [
      { predicate: 'collected', from: 'universal', to: 'throughConnection' },
      { predicate: 'reflected', from: 'content', to: 'intoIdentity' },
      { predicate: 'distinct', from: 'content', to: 'fromForm' },
      { predicate: 'extends', from: 'relationalDeterminations', to: 'across' },
      { predicate: 'is', from: 'trueUniversal', to: 'innerEssence' },
    ],
    candidateSummary: 'In the judgment of reflection, the subject is a singular as such, and the universal is no longer an abstract universality or a singular property, but is posited as a universal that has collected itself together into a unity through the connection of different terms, or as the coalescing of manifold properties and concrete existences. It is only in the judgment of reflection that we first have a determinate content strictly speaking, for the content is the form determination reflected into identity as distinct from the form. In the judgment of existence, the content is merely an immediate, abstract, indeterminate content. Examples of judgments of reflection express an essentiality which is however a relational determination or a comprehensive universality. This universality is still distinct from the universality of the concept as such; although it is no longer the abstract universality of the qualitative judgment, it still has a connection to the immediate from which it proceeds. The concept determines immediate existence to relational determinations that extend across the diverse multiplicity of concrete existence, so that the true universal is indeed the inner essence of that multiplicity, but is such in the sphere of appearance.',
    provenance: {
      sourceChunk: 'refl-1-introduction-unity',
      sourceOp: 'refl-op-1-introduction-unity',
    },
};

export const reflOp2NotMerelyQuantity: LogicalOperation = {
    id: 'refl-op-2-not-merely-quantity',
    chunkId: 'refl-2-not-merely-quantity',
    label: 'Not Merely Quantity; Most External Determination of Mediation',
    clauses: [
      'maySeemFitting = defineAs(judgmentOfReflection, judgmentOfQuantity)',
      'immediacy.sublated != justSublatedQuality',
      'immediacy.sublated != merelyQuantity',
      'quality = mostExternalImmediacy',
      'quantity = mostExternalDetermination',
      'quantity.belongs = toMediation',
    ],
    predicates: [
      { name: 'defineAs', args: ['judgmentOfReflection', 'judgmentOfQuantity'] },
      { name: 'mostExternalImmediacy', args: ['quality'] },
      { name: 'mostExternalDetermination', args: ['quantity'] },
    ],
    relations: [
      { predicate: 'belongs', from: 'quantity', to: 'toMediation' },
    ],
    candidateSummary: 'It may seem fitting to define the judgment of reflection as a judgment of quantity, just as the judgment of existence was defined also as qualitative judgment. But just as the immediacy in the latter was not just there, but was an immediacy which is also essentially mediated and abstract, so, here also, that same immediacy which is now sublated is not just sublated quality, and therefore not merely quantity; on the contrary, just as quality is the most external immediacy, so is quantity, in the same way, the most external determination belonging to mediation.',
    provenance: {
      sourceChunk: 'refl-2-not-merely-quantity',
      sourceOp: 'refl-op-2-not-merely-quantity',
    },
};

export const reflOp3MovementSubject: LogicalOperation = {
    id: 'refl-op-3-movement-subject',
    chunkId: 'refl-3-movement-subject',
    label: 'Movement in Subject, Not Predicate; Reflected In-Itselfness',
    clauses: [
      'inJudgmentOfExistence.movement = inPredicate',
      'inJudgmentOfExistence.subject = underlyingBasis',
      'inJudgmentOfReflection.movement = inSubject',
      'judgmentOfReflection.has = reflectedInItselfness',
      'essential = universal',
      'essential = predicate',
      'predicate.constitutes = basis',
      'subject.measured = against(predicate)',
      'predicate.receives = furtherDetermination',
      'predicate.receives = indirectly',
      'subject.progression = directAdvance',
    ],
    predicates: [
      { name: 'reflectedInItselfness', args: ['judgmentOfReflection'] },
      { name: 'measured', args: ['subject', 'against'] },
      { name: 'directAdvance', args: ['subject.progression'] },
    ],
    relations: [
      { predicate: 'has', from: 'judgmentOfReflection', to: 'reflectedInItselfness' },
      { predicate: 'constitutes', from: 'predicate', to: 'basis' },
      { predicate: 'measured', from: 'subject', to: 'against' },
      { predicate: 'receives', from: 'predicate', to: 'furtherDetermination' },
    ],
    candidateSummary: 'Also to be noted concerning the determination as it appears in the movement of the judgment of reflection is that, in the judgment of immediate existence, the movement of the determination showed itself in the predicate, for this kind of judgment was in the determination of immediacy and its subject, therefore, appeared as the underlying basis. For a similar reason, in the judgment of reflection the onward movement of determination runs its course in the subject, for this judgment has the reflected in-itselfness for its determination. Hence the essential is here the universal or the predicate, and it is the latter, therefore, that constitutes the basis against which the subject is to be measured and determined accordingly. Yet the predicate also receives a further determination through the further development of the form of the subject, but it receives it indirectly, whereas the progression of the subject manifests itself, for the reason just given, as a direct advance in determination.',
    provenance: {
      sourceChunk: 'refl-3-movement-subject',
      sourceOp: 'refl-op-3-movement-subject',
    },
};

export const reflOp4Subsumption: LogicalOperation = {
    id: 'refl-op-4-subsumption',
    chunkId: 'refl-4-subsumption',
    label: 'Objective Signification; Judgments of Subsumption',
    clauses: [
      'singular.entersExistence = byVirtueOf(universality)',
      'singular.entersExistence = inEssentialDetermination',
      'essentialDetermination = relational',
      'essentiality.maintains = across(manifoldOfAppearance)',
      'subject = determinedInAndForItself',
      'subject.determinateness = inPredicate',
      'singular.reflected = into(predicate)',
      'predicate = universalEssence',
      'subject = concreteExistence',
      'subject = phenomenalSomething',
      'predicate.noLongerInheres = inSubject',
      'predicate = implicitBeing',
      'subject.subsumed = under(predicate)',
      'subject.subsumed = asAccidental',
      'judgmentsOfExistence = judgmentsOfInherence',
      'judgmentsOfReflection = judgmentsOfSubsumption',
    ],
    predicates: [
      { name: 'maintains', args: ['essentiality', 'across'] },
      { name: 'determinedInAndForItself', args: ['subject'] },
      { name: 'subsumed', args: ['subject', 'under'] },
      { name: 'judgmentsOfInherence', args: ['judgmentsOfExistence'] },
      { name: 'judgmentsOfSubsumption', args: ['judgmentsOfReflection'] },
    ],
    relations: [
      { predicate: 'entersExistence', from: 'singular', to: 'byVirtueOf' },
      { predicate: 'maintains', from: 'essentiality', to: 'across' },
      { predicate: 'reflected', from: 'singular', to: 'into' },
      { predicate: 'subsumed', from: 'subject', to: 'under' },
    ],
    candidateSummary: 'As regards the objective signification of the judgment, the singular enters into existence by virtue of its universality, but it does so in an essential determination which is relational, in an essentiality that maintains itself across the manifold of appearance; the subject is supposed to be that which is determined in and for itself; this is the determinateness which it has in its predicate. The singular, for its part, is reflected into this predicate which is its universal essence; to this extent, the subject is a concrete existence and a phenomenal something. In this judgment, the predicate no longer inheres in the subject, for it is rather the implicit being under which the singular subject is subsumed as an accidental. If the judgments of existence can also be defined as judgments of inherence, then the judgments of reflection are by contrast judgments of subsumption.',
    provenance: {
      sourceChunk: 'refl-4-subsumption',
      sourceOp: 'refl-op-4-subsumption',
    },
};

// ============================================================================
// A. THE SINGULAR JUDGMENT
// ============================================================================

export const reflOp5SingularJudgment: LogicalOperation = {
    id: 'refl-op-5-singular-judgment',
    chunkId: 'refl-5-singular-judgment',
    label: 'Singular Judgment: "This is Essential Universal"; Truth in Particular',
    clauses: [
      'immediateJudgmentOfReflection = "singular is universal"',
      'moreAccurately = "this is essentialUniversal"',
      '"this" != essentialUniversal',
      'positiveJudgment.mustBeTaken = negatively',
      'negation.doesNotAffect = predicate',
      'predicate = implicitBeing',
      'predicate != inheresInSubject',
      'subject = alterable',
      'subject.needs = determination',
      'negativeJudgment.says = "notAThis is universalOfReflection"',
      'inItself.has = moreUniversalConcreteExistence',
      'singularJudgment.hasTruth = inParticularJudgment',
    ],
    predicates: [
      { name: 'essentialUniversal', args: ['predicate'] },
      { name: 'alterable', args: ['subject'] },
      { name: 'needs', args: ['subject', 'determination'] },
      { name: 'universalOfReflection', args: ['predicate'] },
      { name: 'hasTruth', args: ['singularJudgment', 'inParticularJudgment'] },
    ],
    relations: [
      { predicate: 'mustBeTaken', from: 'positiveJudgment', to: 'negatively' },
      { predicate: 'doesNotAffect', from: 'negation', to: 'predicate' },
      { predicate: 'needs', from: 'subject', to: 'determination' },
      { predicate: 'hasTruth', from: 'singularJudgment', to: 'inParticularJudgment' },
    ],
    candidateSummary: 'Now the immediate judgment of reflection is again, "the singular is universal," but with the subject and predicate in the signification just explained. More accurately, therefore, it can also be expressed thus, "this is an essential universal." But a "this" is not an essential universal. That positive judgment (positive according to form) must as judgment be taken negatively. But inasmuch as the judgment of reflection is not merely something positive, the negation does not directly affect the predicate (a predicate which does not inhere in the subject but is rather its implicit being). On the contrary, it is the subject that is alterable and needs determination. The negative judgment is therefore to be understood as saying: "\'not a this\' is a universal of reflection"; such an in-itself has a more universal concrete existence than it would have in a "this." Accordingly, the singular judgment has its proximate truth in the particular judgment.',
    provenance: {
      sourceChunk: 'refl-5-singular-judgment',
      sourceOp: 'refl-op-5-singular-judgment',
    },
};

// ============================================================================
// B. THE PARTICULAR JUDGMENT
// ============================================================================

export const reflOp6ParticularExtension: LogicalOperation = {
    id: 'refl-op-6-particular-extension',
    chunkId: 'refl-6-particular-extension',
    label: 'Particular Judgment: Extension of Singular; Contains Both Positive/Negative',
    clauses: [
      'nonSingularity = particularity',
      'particularity = essentialSingularity',
      'particularity != abstract',
      'particularity = extension(singular)',
      'particularity.extension = inExternalReflection',
      'subject = "theseOnes"',
      'subject = "particularNumberOfSingulars"',
      'judgment = "someSingulars are universalOfReflection"',
      'judgment.appears = positive',
      'judgment.is = negative',
      '"some" = containsUniversality',
      '"some" = comprehensive',
      '"some" = particularity',
      '"some" = disproportionate(universality)',
      'negativeDetermination.obtained = throughTransition',
      'negativeDetermination = ofConnection',
      'negativeDetermination = ofCopula',
      'implicated = "someHumans are happy"',
      'immediateConsequence = "someHumans are not happy"',
      'positiveNegative.noLongerFall = outsideOneAnother',
      'particular.contains = both(positive, negative)',
      'particularJudgment = indeterminate',
    ],
    predicates: [
      { name: 'essentialSingularity', args: ['particularity'] },
      { name: 'extension', args: ['particularity', 'singular'] },
      { name: 'comprehensive', args: ['"some"'] },
      { name: 'disproportionate', args: ['"some"', 'universality'] },
      { name: 'indeterminate', args: ['particularJudgment'] },
    ],
    relations: [
      { predicate: 'is', from: 'nonSingularity', to: 'particularity' },
      { predicate: 'extension', from: 'particularity', to: 'inExternalReflection' },
      { predicate: 'contains', from: 'particular', to: 'both' },
      { predicate: 'noLongerFall', from: 'positiveNegative', to: 'outsideOneAnother' },
    ],
    candidateSummary: 'The non-singularity of the subject that must be posited in the first judgment of reflection instead of the subject\'s singularity is particularity. But particularity is determined in the judgment of reflection as essential singularity; particularity cannot be, therefore, a simple, abstract determination in which the singular would be sublated and the concrete existent dissolved, but is rather only an extension of this singular in external reflection. Thus the subject is: "these ones," or "a particular number of singulars." The judgment, "some singulars are a universal of reflection," appears at first to be a positive judgment, but it is just as well also negative; for "some" contains universality and may, accordingly, be regarded as comprehensive; but since it is particularity, it is equally disproportionate with respect to universality. The negative determination which the subject has obtained through the transition of the singular judgment also is, as we have shown above, the determination of the connection, the copula. Implicated in the judgment, "some humans are happy," is the immediate consequence: "some humans are not happy." When some things are useful, then, precisely for that reason, there also are some that are not useful. The positive and the negative judgment no longer fall outside one another, but the particular immediately contains both at the same time, precisely because it is a judgment of reflection. But the particular judgment is therefore indeterminate.',
    provenance: {
      sourceChunk: 'refl-6-particular-extension',
      sourceOp: 'refl-op-6-particular-extension',
    },
};

export const reflOp7UniversalNature: LogicalOperation = {
    id: 'refl-op-7-universal-nature',
    chunkId: 'refl-7-universal-nature',
    label: 'Subject Contains Universal Nature; Species Anticipated',
    clauses: [
      'subject.contains = particularForm("some")',
      'subject.contains = contentDetermination("humans")',
      'singularJudgment.subject = "thisHuman"',
      'singularJudgment.subject = "Gaius"',
      'particularJudgment.subject != "someGaiuses"',
      'Gaius = singularAsSingular',
      'toSome.added = universalContent',
      'universalContent = "humans"',
      'universalContent = "animals"',
      'universalContent != mereEmpiricalContent',
      'universalContent.determined = byFormOfJudgment',
      'universalContent = universal',
      '"some" = containsUniversality',
      'universality.mustBeSeparated = fromSingulars',
      'universality = universalNature',
      'universality = species("human")',
      'universality = species("animal")',
      'universality = result(judgmentOfReflection)',
      'universality = anticipated',
      'positiveJudgment.anticipates = determination',
      'positiveJudgment.anticipates = result(judgmentOfExistence)',
    ],
    predicates: [
      { name: 'universalNature', args: ['universality'] },
      { name: 'species', args: ['universality'] },
      { name: 'anticipated', args: ['universality'] },
      { name: 'determined', args: ['universalContent', 'byFormOfJudgment'] },
    ],
    relations: [
      { predicate: 'contains', from: 'subject', to: 'particularForm' },
      { predicate: 'contains', from: 'subject', to: 'contentDetermination' },
      { predicate: 'added', from: 'toSome', to: 'universalContent' },
      { predicate: 'mustBeSeparated', from: 'universality', to: 'fromSingulars' },
      { predicate: 'is', from: 'universality', to: 'result' },
    ],
    candidateSummary: 'If, in the example of such a judgment, we consider further the subject, "some humans," "some animals," etc., we find that it contains, besides the particular form determination of "some," also the content determination of "humans," etc. By the subject of the singular judgment one could mean, "this human," a singularity that properly pertains to external pointing; it would best be expressed, therefore, by something like "Gaius." But the subject of the particular judgment can no longer be "some Gaiuses," for Gaius is supposed to be a singular as singular. To the "some," therefore, there is added a more universal content, say "humans," "animals," etc. This is not a mere empirical content, but one which is determined by the form of the judgment; it is universal, that is, because "some" contains universality, and the latter must at the same time be separated from the singulars which the reflected singularity has as a basis. More precisely, this universality is also the universal nature or species "human," "animal" - the universality which is the result of the judgment of reflection, but anticipated; just as the positive judgment, since it has the singular for subject, also anticipates the determination which is the result of the judgment of existence.',
    provenance: {
      sourceChunk: 'refl-7-universal-nature',
      sourceOp: 'refl-op-7-universal-nature',
    },
};

export const reflOp8TotalityExtension: LogicalOperation = {
    id: 'refl-op-8-totality-extension',
    chunkId: 'refl-8-totality-extension',
    label: 'Subject as Totality; Extension to Universality (Allness)',
    clauses: [
      'subject.contains = singulars',
      'subject.contains = particularity',
      'subject.contains = universalNature',
      'subject = totality',
      'subject.consideration = external',
      'extension("this" to particularity) != commensurate',
      '"this" = perfectlyDeterminate',
      '"some" = indeterminate',
      'extension.ought = appropriateTo("this")',
      'extension.ought = completelyDetermined',
      'extension = totality',
      'extension = universality',
      'universality.hasBasis = "this"',
      'singular = reflectedIntoItself',
      'furtherDeterminations.run = outside(singular)',
      'particularity.determined = as("some")',
      'universality.attained = as("allness")',
      'particularJudgment.passedOver = intoUniversal',
    ],
    predicates: [
      { name: 'totality', args: ['subject'] },
      { name: 'perfectlyDeterminate', args: ['"this"'] },
      { name: 'indeterminate', args: ['"some"'] },
      { name: 'completelyDetermined', args: ['extension'] },
      { name: 'reflectedIntoItself', args: ['singular'] },
      { name: 'allness', args: ['universality'] },
    ],
    relations: [
      { predicate: 'contains', from: 'subject', to: 'singulars' },
      { predicate: 'contains', from: 'subject', to: 'particularity' },
      { predicate: 'contains', from: 'subject', to: 'universalNature' },
      { predicate: 'hasBasis', from: 'universality', to: '"this"' },
      { predicate: 'passedOver', from: 'particularJudgment', to: 'intoUniversal' },
    ],
    candidateSummary: 'Thus the subject that contains the singulars, their connection to particularity, and the universal nature, is already posited as the totality of the determinations of the concept. But, to be precise, this consideration is an external one. What is at first already posited in the subject by virtue of its form, in reciprocal connection, is the extension of the "this" to particularity; but this generalization is not commensurate to the "this"; the latter is perfectly determinate, but "some" is indeterminate. The extension ought to be appropriate to the "this" and therefore, in conformity with it, it ought to be completely determined; such an extension is totality, or, in the first instance, universality in general. This universality has the "this" for its basis, for the singular is here the singular reflected into itself; its further determinations run their course, therefore, outside it, and just as for this reason particularity determined itself as a "some," so the universality which the subject has attained is an "allness," and thus the particular judgment has passed over into the universal.',
    provenance: {
      sourceChunk: 'refl-8-totality-extension',
      sourceOp: 'refl-op-8-totality-extension',
    },
};

// ============================================================================
// C. THE UNIVERSAL JUDGMENT
// ============================================================================

export const reflOp9AllnessExternal: LogicalOperation = {
    id: 'refl-op-9-allness-external',
    chunkId: 'refl-9-allness-external',
    label: 'Universal Judgment: "Allness" as External Universality; Bad Infinity',
    clauses: [
      'universality = externalUniversalityOfReflection',
      'universality = "allness"',
      '"all" = allOfAllSingulars',
      'singular.remains = unchanged',
      'universality = commonality(selfSubsistingSingulars)',
      'universality = association(singulars)',
      'association = byWayOf(comparison)',
      'polynomial.universalValue = greaterThan(binomial)',
      'polynomial.displays = moreSingleTerms',
      'demand = resolveInFullUniversality',
      'demand.requires = pantonomial',
      'demand.requires = exhaustedInfinity',
      'binomial = pantonomial',
      'when(method = rule, binomial = pantonomial)',
      'method = trueUniversal',
      'rule = trueUniversal',
      'badInfinity.concept = achievedBeyond',
      'badInfinity = unattainableBeyond',
      'allness.exhausted = inSingulars',
      'allness = relapse(badInfinity)',
      'plurality = particularity',
      'plurality != allness',
      'obscureIntimation = universalityOfConcept',
      'universalityOfConcept = inAndForItself',
    ],
    predicates: [
      { name: 'externalUniversalityOfReflection', args: ['universality'] },
      { name: 'allness', args: ['universality'] },
      { name: 'commonality', args: ['selfSubsistingSingulars'] },
      { name: 'association', args: ['singulars'] },
      { name: 'trueUniversal', args: ['method'] },
      { name: 'trueUniversal', args: ['rule'] },
      { name: 'badInfinity', args: ['concept'] },
      { name: 'relapse', args: ['allness', 'badInfinity'] },
    ],
    relations: [
      { predicate: 'is', from: 'universality', to: 'externalUniversalityOfReflection' },
      { predicate: 'is', from: 'universality', to: '"allness"' },
      { predicate: 'is', from: '"all"', to: 'allOfAllSingulars' },
      { predicate: 'is', from: 'universality', to: 'commonality' },
      { predicate: 'is', from: 'universality', to: 'association' },
      { predicate: 'is', from: 'method', to: 'trueUniversal' },
      { predicate: 'is', from: 'rule', to: 'trueUniversal' },
      { predicate: 'relapse', from: 'allness', to: 'badInfinity' },
    ],
    candidateSummary: 'The universality of the subject of the universal judgment is the external universality of reflection, "allness"; the "all" is the all of all the singulars in which the singular remains unchanged. This universality is therefore only a commonality of self-subsisting singulars, an association of such singulars as comes about only by way of comparison. This is the association that first comes to mind at a subjective level of representation when there is talk of universality. The most obvious reason given for viewing a determination as universal is because it fits many. Also in analysis is this conception of universality the one most prevalent, as when, for instance, the development of a function in a polynomial is taken to have greater universal value than its development in a binomial, because the polynomial displays more single terms than the binomial. The demand that the function should be resolved in its full universality would require, strictly speaking, a pantonomial, the exhausted infinity. But here is where the limitation of that demand becomes apparent, and where the display of the infinite number of terms must rest satisfied with the ought it commands, and therefore also with a polynomial. But in fact the binomial is already the pantonomial in those cases where the method or the rule concerns only the dependence of one member on another, and the dependence of several terms on those that precede them does not particularize itself but remains one and the same underlying function. It is the method or the rule which is to be regarded as the true universal; in the progress of the development or in the development of a polynomial, the rule is only repeated, so that it gains nothing in universality through the increased number of terms. We have already spoken earlier of the bad infinity and its deception; the universality of the concept is the achieved beyond, whereas that bad infinity remains afflicted with a beyond which is unattainable but remains a mere progression to infinity. If it is allness that universality brings to mind, a universality that ought to be exhausted in singulars as singulars, then there has been a relapse into that bad infinity; or else it is mere plurality which is taken for allness. But plurality, however great it might be, remains inescapably only particularity: it is not allness. Yet there is in all this an obscure intimation of the universality of the concept as it exists in and for itself; it is the concept that violently strives to reach beyond the stubborn singularity to which pictorial representation clings and beyond the externality of its reflection, passing off allness as totality or rather as the category of the in-and-for-itself.',
    provenance: {
      sourceChunk: 'refl-9-allness-external',
      sourceOp: 'refl-op-9-allness-external',
    },
};

export const reflOp10EmpiricalAllness: LogicalOperation = {
    id: 'refl-op-10-empirical-allness',
    chunkId: 'refl-10-empirical-allness',
    label: 'Empirical Allness: Task and Ought; Subjective vs Objective',
    clauses: [
      'allness = empiricalUniversality',
      'singular = immediate',
      'singular = preGiven',
      'singular = externallyPicked',
      'reflection = external',
      'singular.indifferent = toReflection',
      'universality.cannotCombine = toUnity',
      'empiricalAllness = task',
      'empiricalAllness = ought',
      'empiricalAllness.cannotBeRepresented = inFormOfBeing',
      'empiricallyUniversalProposition = pluralityCountsForAllness',
      'if(noContrary, pluralityCountsForAllness)',
      'subjectiveAllness = objectiveAllness',
    ],
    predicates: [
      { name: 'empiricalUniversality', args: ['allness'] },
      { name: 'preGiven', args: ['singular'] },
      { name: 'externallyPicked', args: ['singular'] },
      { name: 'task', args: ['empiricalAllness'] },
      { name: 'ought', args: ['empiricalAllness'] },
      { name: 'pluralityCountsForAllness', args: ['empiricallyUniversalProposition'] },
    ],
    relations: [
      { predicate: 'is', from: 'allness', to: 'empiricalUniversality' },
      { predicate: 'indifferent', from: 'singular', to: 'toReflection' },
      { predicate: 'cannotCombine', from: 'universality', to: 'toUnity' },
      { predicate: 'is', from: 'empiricalAllness', to: 'task' },
      { predicate: 'is', from: 'empiricalAllness', to: 'ought' },
      { predicate: 'is', from: 'subjectiveAllness', to: 'objectiveAllness' },
    ],
    candidateSummary: 'This is apparent in other ways as well in the allness which is above all empirical universality. Inasmuch as the singular is presupposed as something immediate and is therefore pre-given and externally picked, the reflection which collects it into an allness is equally external to it. But because the singular, as a "this," is absolutely indifferent to such a reflection, the universality and the collected singularity cannot combine to form a unity. The empirical allness thus remains a task; it is an ought which, as such, cannot be represented in the form of being. Now an empirically universal proposition, for nevertheless such are advanced, rests on the tacit agreement that, if no instance of the contrary can be adduced, a plurality of cases ought to count for an allness; or that a subjective allness, namely the known cases, may be taken for an objective allness.',
    provenance: {
      sourceChunk: 'refl-10-empirical-allness',
      sourceOp: 'refl-op-10-empirical-allness',
    },
};

export const reflOp11AchievedUniversality: LogicalOperation = {
    id: 'refl-op-11-achieved-universality',
    chunkId: 'refl-11-achieved-universality',
    label: 'Subject Contains Achieved Universality; Posited Equal to Presupposed',
    clauses: [
      'subject.contains = achievedUniversality',
      'achievedUniversality = presupposed',
      'achievedUniversality = posited',
      '"allHumans".expresses1 = species("human")',
      '"allHumans".expresses2 = speciesInSingularization',
      'singulars.expanded = toUniversalityOfSpecies',
      'throughConjunction(universality, singularity)',
      'universality.perfectlyDetermined = asSingularity',
      'positedUniversality = equalTo(presupposed)',
    ],
    predicates: [
      { name: 'achievedUniversality', args: ['subject.contains'] },
      { name: 'presupposed', args: ['achievedUniversality'] },
      { name: 'posited', args: ['achievedUniversality'] },
      { name: 'species', args: ['"allHumans".expresses1'] },
      { name: 'speciesInSingularization', args: ['"allHumans".expresses2'] },
      { name: 'perfectlyDetermined', args: ['universality', 'asSingularity'] },
      { name: 'equalTo', args: ['positedUniversality', 'presupposed'] },
    ],
    relations: [
      { predicate: 'contains', from: 'subject', to: 'achievedUniversality' },
      { predicate: 'expanded', from: 'singulars', to: 'toUniversalityOfSpecies' },
      { predicate: 'is', from: 'positedUniversality', to: 'equalTo' },
    ],
    candidateSummary: 'Now a closer examination of the universal judgment before us shows that the subject, as we have just noted, contains the achieved universality as presupposed; it even contains it as posited in it. "All humans" expresses, first, the species "human"; second, this species in its singularization, but in such a way that the singulars are at the same time expanded to the universality of the species; conversely, through this conjunction with singularity, the universality is just as perfectly determined as singularity, and the posited universality has thereby become equal to what was presupposed.',
    provenance: {
      sourceChunk: 'refl-11-achieved-universality',
      sourceOp: 'refl-op-11-achieved-universality',
    },
};

export const reflOp12ObjectiveUniversality: LogicalOperation = {
    id: 'refl-op-12-objective-universality',
    chunkId: 'refl-12-objective-universality',
    label: 'Singularity Expanded to Allness; Objective Universality; "The Human Being"',
    clauses: [
      'singularity.expanded = toAllness',
      'singularity.posited = asNegativity',
      'negativity = identicalSelfReference',
      'singularity.notRemained = firstSingularity',
      'singularity = determination(identicalWithUniversality)',
      'singularity = absoluteDeterminateness(universal)',
      'firstSingularity = notImmediateSingularity',
      'firstSingularity = throughDialecticalMovement',
      'firstSingularity = negativeIdentity',
      'truePresupposition = inItself',
      'reflection.notExternal = toSingularity',
      'reflection.makesExplicit = whatWasImplicit',
      'result = objectiveUniversality',
      'subject.shed = formDetermination',
      '"allHumans" → "theHumanBeing"',
    ],
    predicates: [
      { name: 'expanded', args: ['singularity', 'toAllness'] },
      { name: 'identicalSelfReference', args: ['negativity'] },
      { name: 'absoluteDeterminateness', args: ['universal'] },
      { name: 'negativeIdentity', args: ['firstSingularity'] },
      { name: 'objectiveUniversality', args: ['result'] },
      { name: 'makesExplicit', args: ['reflection', 'whatWasImplicit'] },
    ],
    relations: [
      { predicate: 'expanded', from: 'singularity', to: 'toAllness' },
      { predicate: 'posited', from: 'singularity', to: 'asNegativity' },
      { predicate: 'is', from: 'negativity', to: 'identicalSelfReference' },
      { predicate: 'notRemained', from: 'singularity', to: 'firstSingularity' },
      { predicate: 'is', from: 'singularity', to: 'determination' },
      { predicate: 'is', from: 'result', to: 'objectiveUniversality' },
    ],
    candidateSummary: 'But, strictly speaking, we should not anticipate the presupposed but should rather consider the result for itself in the form determination. The singularity, inasmuch as it is expanded to allness, is posited as negativity, and this is identical self-reference. It has not remained, therefore, that first singularity (of Gaius, for instance) but is a determination identical with universality, or the absolute determinateness of the universal. That first singularity of the singular judgment was not the immediate singularity of the positive judgment, but came about through the dialectical movement of the judgment of existence in general; it was already determined to be the negative identity of the determinations of that judgment. This is the true presupposition in the judgment of reflection; as contrasted to the positing that runs its course in that judgment, that first determinateness of singularity was the latter\'s in-itself; consequently, what singularity is in itself, through the movement of the judgment of reflection is now posited, posited, that is, as the identical self-reference of the determinate. Therefore the reflection that expanded the singularity to allness is not external to it; on the contrary, it only makes explicit what was before implicit. Hence the result is in truth the objective universality. The subject has thus shed the form determination of the judgment of reflection that made its way from the "this" to the "allness" through the "some." Instead of "all humans," we now have to say "the human being."',
    provenance: {
      sourceChunk: 'refl-12-objective-universality',
      sourceOp: 'refl-op-12-objective-universality',
    },
};

export const reflOp13Genus: LogicalOperation = {
    id: 'refl-op-13-genus',
    chunkId: 'refl-13-genus',
    label: 'Genus: Concrete Universality; Relation Reversed; Judgment Sublated',
    clauses: [
      'universality = genus',
      'genus = concreteInUniversality',
      'genus.doesNotInhere = inSubject',
      'genus != property',
      'genus.contains = allSingularDeterminacies',
      'genus.dissolved = intoSubstantialPurity',
      'genus.posited = asNegativeSelfIdentity',
      'genus = essentiallySubject',
      'genus.noLongerSubsumed = underPredicate',
      'judgmentOfReflection = judgmentOfSubsumption',
      'predicate = implicitUniversal',
      'predicate = relationalDetermination',
      'predicate = mark',
      'predicate.makesSubject = appearance',
      'when(subject = objectiveUniversality)',
      'subject.ceases = subsumed',
      'predicate = particular',
      'relation.reversed = itself',
      'judgment.sublated = itself',
    ],
    predicates: [
      { name: 'genus', args: ['universality'] },
      { name: 'concreteInUniversality', args: ['genus'] },
      { name: 'negativeSelfIdentity', args: ['genus'] },
      { name: 'essentiallySubject', args: ['genus'] },
      { name: 'judgmentOfSubsumption', args: ['judgmentOfReflection'] },
      { name: 'reversed', args: ['relation', 'itself'] },
      { name: 'sublated', args: ['judgment', 'itself'] },
    ],
    relations: [
      { predicate: 'is', from: 'universality', to: 'genus' },
      { predicate: 'doesNotInhere', from: 'genus', to: 'inSubject' },
      { predicate: 'contains', from: 'genus', to: 'allSingularDeterminacies' },
      { predicate: 'is', from: 'genus', to: 'essentiallySubject' },
      { predicate: 'is', from: 'judgmentOfReflection', to: 'judgmentOfSubsumption' },
      { predicate: 'reversed', from: 'relation', to: 'itself' },
      { predicate: 'sublated', from: 'judgment', to: 'itself' },
    ],
    candidateSummary: 'The universality that has thereby arisen is the genus, or the universality which is concrete in its universality. The genus does not inhere in the subject; it is not one property of it or a property at all; it contains all singular determinacies dissolved into its substantial purity. Because it is thus posited as this negative self-identity, it is for that reason essentially subject, but one that is no longer subsumed under its predicate. Consequently the nature of the judgment of reflection is now altogether altered. This judgment was essentially a judgment of subsumption. The predicate was determined, in contrast to its subject, as the implicit universal; according to its content, it could be taken as an essentially relational determination or also as a mark, a determination which makes the subject essentially only an appearance. But when determined to objective universality, the subject ceases to be subsumed under such a relational determination or the collecting grasp of reflection; with respect to this objective universality, a predicate of this sort is rather a particular. The relation of subject and predicate has thus reversed itself, and to this extent the judgment has at this point sublated itself.',
    provenance: {
      sourceChunk: 'refl-13-genus',
      sourceOp: 'refl-op-13-genus',
    },
};

export const reflOp14TransitionNecessity: LogicalOperation = {
    id: 'refl-op-14-transition-necessity',
    chunkId: 'refl-14-transition-necessity',
    label: 'Transition to Judgment of Necessity; Identity in Copula',
    clauses: [
      'sublationOfJudgment = determinationOfCopula',
      'sublationOfJudgment = oneAndSame',
      'subject.raised = toUniversality',
      'subject.equal = toPredicate',
      'subject.predicate = identical',
      'subject.predicate.coincide = inCopula',
      'identity = genus',
      'identity = natureInAndForItself',
      'identity.divides = innerNature',
      'innerNature.connects = subjectAndPredicate',
      'connection = ofNecessity',
      'twoTerms = unessentialDistinctions',
      'whatBelongsToAllSingulars = belongsToGenus',
      'whatBelongsToAllSingulars = byNature',
      'immediateConsequence = subjectShedsFormDetermination',
      'basis = newJudgment',
      'newJudgment = judgmentOfNecessity',
    ],
    predicates: [
      { name: 'oneAndSame', args: ['sublationOfJudgment', 'determinationOfCopula'] },
      { name: 'coincide', args: ['subject.predicate', 'inCopula'] },
      { name: 'genus', args: ['identity'] },
      { name: 'natureInAndForItself', args: ['identity'] },
      { name: 'innerNature', args: ['identity.divides'] },
      { name: 'ofNecessity', args: ['connection'] },
      { name: 'judgmentOfNecessity', args: ['newJudgment'] },
    ],
    relations: [
      { predicate: 'is', from: 'sublationOfJudgment', to: 'determinationOfCopula' },
      { predicate: 'raised', from: 'subject', to: 'toUniversality' },
      { predicate: 'equal', from: 'subject', to: 'toPredicate' },
      { predicate: 'identical', from: 'subject', to: 'predicate' },
      { predicate: 'coincide', from: 'subject.predicate', to: 'inCopula' },
      { predicate: 'is', from: 'identity', to: 'genus' },
      { predicate: 'connects', from: 'innerNature', to: 'subjectAndPredicate' },
      { predicate: 'is', from: 'newJudgment', to: 'judgmentOfNecessity' },
    ],
    candidateSummary: 'This sublation of the judgment coincides with what the determination of the copula becomes, as we still have to consider; the sublation of the determinations of judgment and their transition into the copula are one and same. For inasmuch as the subject has raised itself to universality, it has become in this determination equal to the predicate which, as the reflected universality, also contains particularity within itself; subject and predicate are therefore identical, that is, in the copula they have come to coincide. This identity is the genus or the nature of a thing in and for itself. Inasmuch as this identity, therefore, again divides, it is the inner nature by virtue of which a subject and predicate are connected to each other. This is a connection of necessity wherein the two terms of the judgment are only unessential distinctions. That what belongs to all the singulars of a genus belongs to the genus by nature, is an immediate consequence. It expresses what we have just seen; that the subject, e.g. "all humans," sheds its form determination and "the human being" is what it should say instead. This combination, implicit and explicit, constitutes the basis of a new judgment, the judgment of necessity.',
    provenance: {
      sourceChunk: 'refl-14-transition-necessity',
      sourceOp: 'refl-op-14-transition-necessity',
    },
};

export const judgmentOfReflectionOperations: LogicalOperation[] = [
  reflOp1IntroductionUnity,
  reflOp2NotMerelyQuantity,
  reflOp3MovementSubject,
  reflOp4Subsumption,
  reflOp5SingularJudgment,
  reflOp6ParticularExtension,
  reflOp7UniversalNature,
  reflOp8TotalityExtension,
  reflOp9AllnessExternal,
  reflOp10EmpiricalAllness,
  reflOp11AchievedUniversality,
  reflOp12ObjectiveUniversality,
  reflOp13Genus,
  reflOp14TransitionNecessity,
];

