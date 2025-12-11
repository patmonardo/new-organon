import { ButtonShape, defineButton } from '@graphics/schema/button';
import { Button } from './button';

/**
 * Type of actions available for invoices
 */
export type InvoiceAction = 'edit' | 'delete' | 'view' | 'create';

/**
 * Button specifically for customer actions
 */
export class InvoiceButton extends Button<ButtonShape> {
  constructor(
    private readonly invoiceId: string,
    private readonly action: InvoiceAction
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
          href: `/invoices/${this.invoiceId}/edit`
        });

      case 'delete':
        return defineButton({
          variant: 'danger',
          icon: 'trash',
          iconSource: 'heroicons',
          label: 'Delete',
          srOnly: true,
          customClass: 'rounded-md border p-2 hover:bg-gray-100',
          confirmMessage: 'Are you sure you want to delete this invoice?',
          refreshAfterAction: true,
          onClick: async () => {
            console.log(`Delete invoice ${this.invoiceId} - functionality coming soon`);
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
          href: `/invoices/${this.invoiceId}`
        });

      case 'create':
        return defineButton({
          variant: 'primary',
          icon: 'plus',
          iconSource: 'heroicons',
          label: 'Create Invoice',
          customClass: 'flex h-10 items-center rounded-lg bg-blue-600 px-4 text-sm font-medium text-white transition-colors hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600',
          href: '/invoices/create'
        });

      default:
        throw new Error(`Unknown customer action: ${this.action}`);
    }
  }
}

// Convenience components

export function InvoiceCreateButton() {
  const button = new InvoiceButton('new', 'create');
  return button.render();
}

export function InvoiceViewButton({ id }: { id: string }) {
  const button = new InvoiceButton(id, 'view');
  return button.render();
}

export function InvoiceEditButton({ id }: { id: string }) {
  const button = new InvoiceButton(id, 'edit');
  return button.render();
}

export function InvoiceDeleteButton({ id }: { id: string }) {
  const button = new InvoiceButton(id, 'delete');
  return button.render();
}
