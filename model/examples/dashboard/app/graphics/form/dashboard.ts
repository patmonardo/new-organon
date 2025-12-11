//@graphics/form/dashboard.tsx
import { DashboardFormShape } from '@graphics/schema/dashboard'
import type { Dashboard } from '@/data/schema/dashboard'

export class DashboardForm {
  constructor(private readonly data?: Dashboard) { }

  create(): DashboardFormShape {
    return DashboardFormShape.parse({
      layout: {
        title: 'New Dashboard',
        columns: 'single',
        sections: [
          {
            title: 'Dashboard Configuration',
            fields: ['title', 'layout', 'widgets']
          }
        ]
      },
      fields: {
        elements: [
          {
            id: 'title',
            type: 'text',
            label: 'Dashboard Title',
            required: true
          },
          {
            id: 'layout',
            type: 'select',
            label: 'Layout Type',
            required: true
          },
          {
            id: 'widgets',
            type: 'widget-list',
            label: 'Widgets',
            required: true
          }
        ]
      },
      actions: [
        { type: 'cancel', label: 'Cancel', variant: 'secondary' },
        { type: 'submit', label: 'Create Dashboard', variant: 'primary' }
      ]
    })
  }

  edit(): DashboardFormShape {
    return DashboardFormShape.parse({
      layout: {
        title: 'Edit Dashboard',
        columns: 'single',
        sections: [
          {
            title: 'Dashboard Configuration',
            fields: ['title', 'layout', 'widgets']
          }
        ]
      },
      fields: {
        elements: [
          {
            id: 'title',
            type: 'text',
            label: 'Dashboard Title',
            required: true,
            defaultValue: this.data?.title
          },
          {
            id: 'layout',
            type: 'select',
            label: 'Layout Type',
            required: true,
            defaultValue: this.data?.layout.type
          },
          {
            id: 'widgets',
            type: 'widget-list',
            label: 'Widgets',
            required: true,
            defaultValue: JSON.stringify(this.data?.widgets)
          }
        ]
      },
      actions: [
        { type: 'cancel', label: 'Cancel', variant: 'secondary' },
        { type: 'submit', label: 'Save Changes', variant: 'primary' }
      ]
    })
  }
}
