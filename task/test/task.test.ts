import { describe, it, expect } from 'vitest';
import { task } from '../src/index.js';

describe('@organon/task', () => {
  it('exports task', () => {
    expect(task).toBe('task');
  });
});
