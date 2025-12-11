//@graphics/list/render.tsx
import ListRenderer from './list';
import { ListShape } from '@graphics/schema/list';
import { ReactNode } from 'react';

// This is the render function that returns the component
export function renderList(list: ListShape): ReactNode {
  return <ListRenderer list={list} />;
}
