//@graphics/cards/card.tsx
import React from 'react';
import {
  BanknotesIcon,
  ClockIcon,
  UserGroupIcon,
  DocumentTextIcon
} from '@heroicons/react/24/outline';

// Define available card types
export type CardType = 'collected' | 'pending' | 'invoices' | 'customers' | 'default';

// Define props for the Card component
export interface CardProps {
  title: string;
  value: string;
  type?: string | CardType;
  icon?: React.ReactNode;
  className?: string;
}

// Map card types to icons
const iconMap: Record<CardType, React.ReactElement | null> = {
  collected: <BanknotesIcon className="h-5 w-5" />,
  pending: <ClockIcon className="h-5 w-5" />,
  invoices: <DocumentTextIcon className="h-5 w-5" />,
  customers: <UserGroupIcon className="h-5 w-5" />,
  default: null
};

// Map card types to colors
const colorMap: Record<CardType, string> = {
  collected: 'text-green-600',
  pending: 'text-yellow-600',
  invoices: 'text-blue-600',
  customers: 'text-purple-600',
  default: 'text-gray-600'
};

export function Card({ title, value, type = 'default', icon, className = '' }: CardProps) {
  // Use type assertion to help TypeScript understand the indexing
  const cardIcon = icon || iconMap[type as CardType];
  const iconColor = colorMap[type as CardType];

  return (
    <div className={`rounded-xl bg-white p-6 shadow-sm ${className}`}>
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium text-gray-500">{title}</h3>
        <span className={iconColor}>
          {cardIcon}
        </span>
      </div>
      <p className="mt-2 text-2xl font-semibold text-gray-900">{value}</p>
    </div>
  );
}

// Define a generic ContainerCard component for flexible content
export interface ContainerCardProps {
  children: React.ReactNode;
  className?: string;
}

export function ContainerCard({ children, className = '' }: ContainerCardProps) {
  return (
    <div className={`rounded-xl bg-white p-6 shadow-sm ${className}`}>
      {children}
    </div>
  );
}

// Export a specialized StatCard for statistics that takes the same props
export function StatCard(props: CardProps) {
  return <Card {...props} />;
}
