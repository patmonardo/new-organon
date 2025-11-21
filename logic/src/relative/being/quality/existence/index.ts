// Existence module: re-export types and aggregate module data
import type { Chunk, LogicalOperation } from '../../../types'
import {
  CANONICAL_CHUNKS as ECH0,
  LOGICAL_OPERATIONS as EOP0,
} from './existence'
import {
  CANONICAL_CHUNKS as FCH0,
  LOGICAL_OPERATIONS as FOP0,
} from './finitude'
import {
  CANONICAL_CHUNKS as FCH1,
  LOGICAL_OPERATIONS as FOP1,
} from './finitude1'
import {
  CANONICAL_CHUNKS as FCH2,
  LOGICAL_OPERATIONS as FOP2,
} from './finitude2'
import {
  CANONICAL_CHUNKS as ICH0,
  LOGICAL_OPERATIONS as IOP0,
} from './infinity'
import {
  CANONICAL_CHUNKS as ICH1,
  LOGICAL_OPERATIONS as IOP1,
} from './infinity1'
import {
  CANONICAL_CHUNKS as ICH2,
  LOGICAL_OPERATIONS as IOP2,
} from './infinity2'

// Re-export types for convenience
export type { Chunk, LogicalOperation } from '../../../types'

// Existence registry
export const CANONICAL_CHUNKS: Chunk[] = [
  ...ECH0,
  ...FCH0,
  ...FCH1,
  ...FCH2,
  ...ICH0,
  ...ICH1,
  ...ICH2,
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...EOP0,
  ...FOP0,
  ...FOP1,
  ...FOP2,
  ...IOP0,
  ...IOP1,
  ...IOP2,
]

// Re-export accessors (if they exist in any of the modules)
// Note: Only export what actually exists in the modules
