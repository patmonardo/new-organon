import { describe, it, expect } from 'vitest';
import {
  assembleWorld,
  indexContent,
  DefaultWorldDriver,
} from '../../../src/absolute/essence/world';
import BaseDriver from '../../../src/absolute/core/driver';

describe('WorldDriver smoke tests', () => {
  it('DefaultWorldDriver extends BaseDriver', () => {
    expect(DefaultWorldDriver).toBeInstanceOf(BaseDriver);
  });

  it('assembleWorld returns world with things and relations arrays for empty input', () => {
    const input: any = {
      entities: [],
      relations: [],
      contexts: [],
      content: [],
    };
    const world = assembleWorld(input);
    expect(world).toBeTruthy();
    expect(world.shape).toBeTruthy();
    expect(Array.isArray(world.shape.things)).toBe(true);
    expect(Array.isArray(world.shape.relations)).toBe(true);
  });

  it('indexContent counts subtle and gross content', () => {
    const input: any = {
      content: [
        { shape: { kind: 'subtle' } },
        { shape: { kind: 'gross', of: { id: 'thing-1' } } },
      ],
    };
    const res = indexContent(input as any);
    expect(res.subtleWorldTotal).toBe(1);
    expect(res.grossByThing['thing-1']).toBe(1);
  });
});
