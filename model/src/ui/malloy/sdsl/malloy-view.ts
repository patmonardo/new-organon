/**
 * MalloyView - Display Translator
 *
 * EXCLUDED FOR NOW - Focusing on transactional CRUD first.
 * Analytics/Malloy can be added back later.
 *
 * This is a stub to prevent import errors.
 */

import type React from 'react';

export interface MalloyViewOptions {
  viewSpec: any;
  result: any;
  display?: 'table' | 'chart' | 'card';
}

/**
 * MalloyView - Stub implementation
 *
 * Excluded while focusing on transactional CRUD.
 * Will be re-implemented when analytics layer is needed.
 */
export class MalloyView {
  async display(
    mode: string,
    format: string,
    options: MalloyViewOptions
  ): Promise<React.ReactNode> {
    // Stub - returns null (excluded for now)
    return null;
  }
}

