import React from 'react';
import {
  BanknotesIcon,
  ClockIcon,
  UserGroupIcon,
  DocumentTextIcon,
  ArrowUpIcon,
  ArrowDownIcon,
} from '@heroicons/react/24/outline';
import { CardShape, StatCardShape, ContainerCardShape } from '@/model/schema/card';
import {
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
  CardGridPrimitive,
} from './primitives';

// Map card types to icons
const iconMap: Record<string, React.ReactElement | null> = {
  collected: <BanknotesIcon className="h-5 w-5" />,
  pending: <ClockIcon className="h-5 w-5" />,
  invoices: <DocumentTextIcon className="h-5 w-5" />,
  customers: <UserGroupIcon className="h-5 w-5" />,
  default: null
};

// Map card types to colors
const colorMap: Record<string, string> = {
  collected: 'text-green-600',
  pending: 'text-yellow-600',
  invoices: 'text-blue-600',
  customers: 'text-purple-600',
  default: 'text-gray-600'
};

/**
 * CardRenderer - renders a basic card Shape using primitives
 */
export function CardRenderer({ shape }: { shape: CardShape }) {
  const { layout } = shape;
  const title = layout?.title;
  const subtitle = layout?.description;
  const content = layout?.sections;
  const containerClass = layout?.className || "shadow-sm";

  return (
    <CardPrimitive className={containerClass}>
      {(title || subtitle) && (
        <CardHeaderPrimitive>
          <div>
            {title && <CardTitlePrimitive>{title}</CardTitlePrimitive>}
            {subtitle && <CardDescriptionPrimitive>{subtitle}</CardDescriptionPrimitive>}
          </div>
        </CardHeaderPrimitive>
      )}
    </CardPrimitive>
  );
}

/**
 * StatCardRenderer - renders a statistics card Shape with trends and progress
 */
export function StatCardRenderer({ shape }: { shape: StatCardShape }) {
  const { layout, stats } = shape;
  const title = layout?.title;
  const variant = layout?.type || 'default';
  const icon = layout?.icon || variant;
  const value = stats?.value;
  const trend = stats?.trend === 'up' ? 'up' : stats?.trend === 'down' ? 'down' : undefined;
  const trendValue = stats?.change?.toString() || stats?.comparison;
  const progress = stats?.goalProgress;
  const label = stats?.timeframe;
  const containerClass = layout?.className || "shadow-sm";

  const cardIcon = iconMap[icon || 'default'] || null;
  const iconColor = colorMap[variant] || colorMap.default;

  return (
    <CardPrimitive className={containerClass}>
      <CardHeaderPrimitive>
        <CardTitlePrimitive>{title}</CardTitlePrimitive>
        <CardIconPrimitive className={iconColor}>
          {cardIcon}
        </CardIconPrimitive>
      </CardHeaderPrimitive>

      <CardContentPrimitive>
        <div className="flex items-baseline">
          <CardValuePrimitive>{value}</CardValuePrimitive>
          {label && <CardLabelPrimitive>{label}</CardLabelPrimitive>}
        </div>

        {trend && trendValue && (
          <CardTrendPrimitive trend={trend}>
            {trend === 'up' && <ArrowUpIcon className="w-4 h-4 mr-1" />}
            {trend === 'down' && <ArrowDownIcon className="w-4 h-4 mr-1" />}
            <span>{trendValue}</span>
          </CardTrendPrimitive>
        )}

        {progress !== undefined && (
          <CardProgressPrimitive progress={progress}>
            Progress
          </CardProgressPrimitive>
        )}
      </CardContentPrimitive>
    </CardPrimitive>
  );
}

/**
 * ContainerCardRenderer - renders a container card Shape with grid layout support
 */
export function ContainerCardRenderer({
  shape,
  children
}: {
  shape: ContainerCardShape;
  children?: React.ReactNode;
}) {
  const { layout } = shape;
  const title = layout?.title;
  const subtitle = layout?.description;
  const columns = layout?.columns;
  const containerClass = layout?.className || "shadow-sm";

  return (
    <CardPrimitive className={containerClass}>
      {(title || subtitle) && (
        <CardHeaderPrimitive>
          <div>
            {title && <CardTitlePrimitive>{title}</CardTitlePrimitive>}
            {subtitle && <CardDescriptionPrimitive>{subtitle}</CardDescriptionPrimitive>}
          </div>
        </CardHeaderPrimitive>
      )}

      <CardContentPrimitive>
        {columns ? (
          <CardGridPrimitive columns={columns === 'double' ? 2 : 1}>
            {children}
          </CardGridPrimitive>
        ) : (
          children
        )}
      </CardContentPrimitive>
    </CardPrimitive>
  );
}

