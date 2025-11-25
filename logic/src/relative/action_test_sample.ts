import type { LogicalOperation } from './types';

// Sample Actionable Logical Operation
export const sampleActionOp: LogicalOperation = {
  id: 'sample-action-op',
  clauses: ['entity = created'],
  action: {
    type: 'morph.create',
    payload: {
      name: 'Test Entity',
      type: 'test.Entity',
    },
    conditions: ['user.isAdmin'],
  },
};

console.log('Sample Action Op:', sampleActionOp);
