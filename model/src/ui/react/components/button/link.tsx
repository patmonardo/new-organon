import Link from 'next/link';
import { LinkShape } from '@/model/schema/link';
import {
  PencilIcon,
  TrashIcon,
  EyeIcon,
  PlusIcon,
} from '@heroicons/react/24/outline';

/**
 * LinkRenderer - renders a LinkShape to React elements
 */
export function LinkRenderer({ link }: { link: LinkShape }) {
  const layout = link.layout;
  const href = layout?.href ?? '#';
  const label = layout?.label;
  const relation = layout?.relation;
  const icon = layout?.icon;
  const customClass = layout?.className;

  // Map relation types to default icons
  const getIcon = () => {
    if (icon === 'pencil') return <PencilIcon className="w-5" />;
    if (icon === 'trash') return <TrashIcon className="w-5" />;
    if (icon === 'eye') return <EyeIcon className="w-5" />;
    if (icon === 'plus') return <PlusIcon className="w-5" />;

    // Default icons by relation type
    if (relation === 'edit') return <PencilIcon className="w-5" />;
    if (relation === 'delete') return <TrashIcon className="w-5" />;
    if (relation === 'view') return <EyeIcon className="w-5" />;
    if (relation === 'create') return <PlusIcon className="w-5" />;

    return null;
  };

  // Map relation types to styles
  const getClasses = () => {
    if (customClass) return customClass;

    const baseClasses = 'rounded-md border p-2 hover:bg-gray-100';

    if (relation === 'delete') {
      return `${baseClasses} text-red-600 hover:bg-red-50`;
    }
    if (relation === 'edit' || relation === 'update') {
      return `${baseClasses} text-blue-600`;
    }
    if (relation === 'create') {
      return `${baseClasses} bg-blue-600 text-white hover:bg-blue-500`;
    }

    return baseClasses;
  };

  return (
    <Link href={href} className={getClasses()}>
      <span className="sr-only">{label || relation || 'Link'}</span>
      {getIcon()}
    </Link>
  );
}

