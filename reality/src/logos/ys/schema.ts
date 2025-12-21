export type SanskritScript = 'devanagari' | 'iast' | 'plain';

export type SutraRef = {
  book: 'I' | 'II' | 'III' | 'IV';
  sutra: number;
  label: string; // e.g. "III.55"
};

export type SutraText = {
  ref: SutraRef;
  devanagari: string;
  iast: string;
};

export type Syllable = {
  text: string;
  script: SanskritScript;
};

export type PadacchedaToken = {
  surface: string;
  normalized?: string;
  gloss?: string;
};

export type Morphology = {
  pos:
    | 'noun'
    | 'verb'
    | 'adjective'
    | 'indeclinable'
    | 'compound'
    | 'participle'
    | 'unknown';
  case?: 'nom' | 'acc' | 'gen' | 'loc' | 'instr' | 'dat' | 'abl' | 'voc';
  number?: 'sg' | 'du' | 'pl';
  gender?: 'm' | 'f' | 'n';
  derivation?: string;
  notes?: string;
};

export type TokenAnalysis = {
  token: PadacchedaToken;
  morphology?: Morphology;
  semantics?: string;
};

export type OperatorKind =
  | 'source'
  | 'predicate'
  | 'quantifier-domain'
  | 'quantifier-manner'
  | 'temporality'
  | 'closure';

export type Operator = {
  kind: OperatorKind;
  token: string;
  claim: string;
};

export type SourceRef = {
  id: string;
  title: string;
  locator?: string; // page, lecture, sutra, etc.
  url?: string;
  note?: string;
};

export type SutraAnalysis = {
  text: SutraText;
  syllables?: Syllable[];
  padaccheda?: PadacchedaToken[];
  tokens?: TokenAnalysis[];
  operators?: Operator[];
  sources?: SourceRef[];
  thesis?: string;
};

export function makeSutraRef(book: SutraRef['book'], sutra: number): SutraRef {
  return { book, sutra, label: `${book}.${sutra}` };
}
