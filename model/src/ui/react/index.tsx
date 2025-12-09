/**
 * React UI Components - Shape Renderers
 * Direct rendering of Shape objects to React elements
 */

// Utilities
export { cn } from './lib/utils';

// Button Components
export { ButtonRenderer } from './button/button';
export { LinkRenderer } from './button/link';

// List Components
export { ListRenderer } from './list/list';
export { BreadcrumbsRenderer } from './list/breadcrumbs';
export { PaginationRenderer } from './list/pagination';

// Card Components
export {
  CardRenderer,
  StatCardRenderer,
  ContainerCardRenderer,
} from './card/card';

// Card Primitives
export {
  CardPrimitive,
  CardHeaderPrimitive,
  CardTitlePrimitive,
  CardIconPrimitive,
  CardContentPrimitive,
  CardValuePrimitive,
  CardLabelPrimitive,
  CardDescriptionPrimitive,
  CardTrendPrimitive,
  CardProgressPrimitive,
  CardFooterPrimitive,
  CardButtonPrimitive,
  CardGridPrimitive,
} from './card/primitives';

// Search Components
export { SearchRenderer } from './search/search';

// Table Components
export { TableRenderer } from './table/table';

// Loading Skeletons
export {
  CardSkeleton,
  CardsSkeleton,
  TableRowSkeleton,
  TableSkeleton,
  DashboardSkeleton,
} from './skeleton/skeletons';

// Navigation Components
export {
  NavLinksRenderer,
  defaultNavItems,
  type NavItem,
  type NavLinksProps,
} from './list/navlinks';

export {
  SideNavRenderer,
  DefaultFooter,
  type SideNavProps,
} from './list/sidenav';

// Text Components
export {
  TextRenderer,
  H1,
  H2,
  H3,
  H4,
  H5,
  H6,
  Body,
  Small,
  Caption,
  type TextProps,
} from './text/text';

// Theme Tokens
export { theme, colors, spacing, borderRadius, shadows } from './theme';
export { fontFamily, fontSize, fontWeight, lineHeight, letterSpacing } from './theme/typography';

