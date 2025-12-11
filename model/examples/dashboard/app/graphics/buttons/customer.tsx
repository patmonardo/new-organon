import { ButtonShape, defineButton } from '@graphics/schema/button';
import { Button } from './button';

/**
 * Type of actions available for customers
 */
export type CustomerAction = 'edit' | 'delete' | 'view' | 'create';

/**
 * Button specifically for customer actions
 */
export class CustomerButton extends Button<ButtonShape> {
  constructor(
    private readonly customerId: string,
    private readonly action: CustomerAction
  ) {
    super();
  }

  protected getButtonShape(): ButtonShape {
    switch (this.action) {
      case 'edit':
        return defineButton({
          variant: 'secondary',
          icon: 'pencil',
          iconSource: 'heroicons',
          label: 'Edit',
          srOnly: true,
          customClass: 'rounded-md border p-2 hover:bg-gray-100',
          href: `/customers/${this.customerId}/edit`
        });

      case 'delete':
        return defineButton({
          variant: 'danger',
          icon: 'trash',
          iconSource: 'heroicons',
          label: 'Delete',
          srOnly: true,
          customClass: 'rounded-md border p-2 hover:bg-gray-100',
          confirmMessage: 'Are you sure you want to delete this customer?',
          refreshAfterAction: true,
          onClick: async () => {
            console.log(`Delete customer ${this.customerId} - functionality coming soon`);
            // Will be implemented when connected to data layer
            return Promise.resolve(); // Makes TypeScript happy
          }
        });

      case 'view':
        return defineButton({
          variant: 'ghost',
          icon: 'eye',
          iconSource: 'heroicons',
          label: 'View Details',
          href: `/customers/${this.customerId}`
        });

      case 'create':
        return defineButton({
          variant: 'primary',
          icon: 'plus',
          iconSource: 'heroicons',
          label: 'Create Customer',
          customClass: 'flex h-10 items-center rounded-lg bg-blue-600 px-4 text-sm font-medium text-white transition-colors hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600',
          href: '/customers/create'
        });

      default:
        throw new Error(`Unknown customer action: ${this.action}`);
    }
  }
}

// Convenience components
export function CustomerEditButton({ id }: { id: string }) {
  const button = new CustomerButton(id, 'edit');
  return button.render();
}

export function CustomerDeleteButton({ id }: { id: string }) {
  const button = new CustomerButton(id, 'delete');
  return button.render();
}

export function CustomerViewButton({ id }: { id: string }) {
  const button = new CustomerButton(id, 'view');
  return button.render();
}

export function CustomerCreateButton() {
  const button = new CustomerButton('new', 'create');
  return button.render();
}
