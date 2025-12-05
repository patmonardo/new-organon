import {
  DialogShape,
  TabsShape,
} from '../src/schema';
import type { DisplayDocument } from '../src/sdsl/types';

/**
 * Radix Demo: A Complex "Human in the Loop" UI
 * --------------------------------------------
 * This example demonstrates how to construct a complex UI using the
 * Radix-UI TS-JSON Language.
 */

export const settingsDialog: DialogShape = {
  type: 'dialog',
  title: 'Settings',
  description: 'Manage your account settings and preferences.',
  modal: true,
  trigger: {
    type: 'button',
    props: { label: 'Open Settings', variant: 'secondary' }
  },
  content: {
    type: 'tabs',
    defaultValue: 'account',
    triggers: [
      { value: 'account', label: 'Account' },
      { value: 'password', label: 'Password' },
      { value: 'notifications', label: 'Notifications' }
    ],
    contents: [
      {
        value: 'account',
        content: {
          type: 'stack',
          children: [
            {
              type: 'card',
              props: {
                title: 'Profile',
                subtitle: 'Update your personal information'
              },
              children: [
                { type: 'text', text: 'Name: John Doe' },
                { type: 'text', text: 'Email: john@example.com' }
              ]
            }
          ]
        }
      },
      {
        value: 'password',
        content: {
          type: 'card',
          props: { title: 'Change Password' },
          children: [
            { type: 'text', text: 'Password change form goes here...' }
          ]
        }
      },
      {
        value: 'notifications',
        content: {
          type: 'stack',
          children: [
            { type: 'text', text: 'Configure your notification preferences.' },
            {
              type: 'list',
              props: {
                items: [
                  { label: 'Email Notifications', value: 'On' },
                  { label: 'Push Notifications', value: 'Off' }
                ]
              }
            }
          ]
        }
      }
    ]
  } as TabsShape
};

// A full document example
export const dashboardPage: DisplayDocument = {
  title: 'Dashboard',
  layout: {
    type: 'page',
    children: [
      {
        type: 'row',
        children: [
          {
            type: 'metric',
            props: { label: 'Total Revenue', value: '$45,231.89', delta: 20.1, trend: 'up' }
          },
          {
            type: 'metric',
            props: { label: 'Subscriptions', value: '+2350', delta: 180.1, trend: 'up' }
          },
          {
            type: 'metric',
            props: { label: 'Active Now', value: '+573', delta: 201, trend: 'up' }
          }
        ]
      },
      {
        type: 'grid',
        props: { columns: 2 },
        children: [
          {
            type: 'card',
            props: { title: 'Recent Sales', subtitle: 'You made 265 sales this month.' },
            children: [
              { type: 'text', text: 'Sales list goes here...' }
            ]
          },
          {
            type: 'card',
            props: { title: 'Overview' },
            children: [
              // Nesting the Settings Dialog here for demo purposes
              settingsDialog
            ]
          }
        ]
      }
    ]
  }
};
