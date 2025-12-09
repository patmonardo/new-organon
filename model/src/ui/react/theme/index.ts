/**
 * Design System Theme Tokens
 * Central export for all design tokens
 */

export * from "./colors";
export * from "./spacing";
export * from "./typography";

import { colors } from "./colors";
import { spacing, borderRadius, shadows } from "./spacing";
import { fontFamily, fontSize, fontWeight, lineHeight, letterSpacing } from "./typography";

/**
 * Complete theme object
 */
export const theme = {
  colors,
  spacing,
  borderRadius,
  shadows,
  fontFamily,
  fontSize,
  fontWeight,
  lineHeight,
  letterSpacing,
} as const;

export type Theme = typeof theme;

