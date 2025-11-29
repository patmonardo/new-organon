import React from 'react';
import { ControllerRegistry } from './registry';

interface PageProps {
  params: { slug: string[] };
  searchParams?: Record<string, string>;
}

export default async function UniversalPage({ params, searchParams }: PageProps) {
  const path = '/' + params.slug.join('/');
  const ControllerClass = ControllerRegistry.get(path);

  if (!ControllerClass) {
    return <div>404 - Not Found: {path}</div>;
  }

  // Instantiate controller (default to 'view' mode for page load)
  // In a real app, we might check searchParams for mode or action
  const controller = new ControllerClass('view');

  // Render the view
  // Note: renderReact returns { element, document }
  const { element } = controller.reactView.renderReact();

  return element;
}
