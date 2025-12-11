//@graphics/list/link.tsx
import { ReactNode } from 'react';
import NextLink from 'next/link';
import { Link } from '@graphics/schema/link';
import { PencilIcon, TrashIcon, PlusIcon } from '@heroicons/react/24/outline';

const ICON_MAP = {
  'pencil': <PencilIcon className="w-5 h-5" />,
  'trash': <TrashIcon className="w-5 h-5" />,
  'plus': <PlusIcon className="w-5 h-5" />,
};

export function renderLink(link: Link): ReactNode {
  // Map relation types to styles
  const getStylesByRelation = (relation?: string) => {
    switch(relation) {
      case 'action':
        return 'px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-500';
      case 'navigate':
        return 'text-blue-600 hover:underline';
      case 'reference':
        return 'text-gray-600 hover:text-gray-900';
      default:
        return 'text-blue-600 hover:underline';
    }
  };

  const icon = link.icon ? ICON_MAP[link.icon as keyof typeof ICON_MAP] : null;

  return (
    <NextLink
      href={link.href}
      className={`inline-flex items-center ${getStylesByRelation(link.relation)}`}
    >
      {icon && <span className="mr-2">{icon}</span>}
      {link.label}
    </NextLink>
  );
}
