import React from 'react';
import type { DisplayDocument, DisplayElement, DisplayLayout, FormHandler } from './types';
import type { HydratorSnapshot } from '../data/semantic-hydrator';
import {
  RadixBadge,
  RadixButton,
  RadixCard,
  RadixCardContent,
  RadixCardFooter,
  RadixCardHeader,
  RadixCardSubtitle,
  RadixCardTitle,
  RadixMetricLabel,
  RadixMetricTrend,
  RadixMetricValue,
  RadixTable,
} from './radix-primitives';

// Import new schemas
import type {
  DialogShape,
  PopoverShape,
  DropdownMenuShape,
  TabsShape,
  AccordionShape,
  TooltipShape,
  ScrollAreaShape
} from '../schema/radix';

export interface RadixRenderContext {
  handler?: FormHandler;
  snapshot?: HydratorSnapshot;
  data?: Record<string, unknown>;
  mode?: 'view' | 'edit' | 'create';
}

type RadixRenderer = (element: DisplayElement, key: string | number, context: RadixRenderContext) => React.ReactNode;

const radixRegistry: Map<string, RadixRenderer> = new Map();

export function registerRadixComponent(type: string, renderer: RadixRenderer): void {
  radixRegistry.set(type, renderer);
}

export function getRadixComponent(type: string): RadixRenderer | undefined {
  return radixRegistry.get(type);
}

export function renderRadixElement(
  element: DisplayElement,
  key: string | number = 0,
  context: RadixRenderContext = {}
): React.ReactNode {
  const renderer = radixRegistry.get(element.type);
  if (renderer) {
    return renderer(element, key, context);
  }

  // Fallback for unknown elements
  const props = element.props ?? {};
  const description = props.description;
  const hasDescription = description !== undefined && description !== null && description !== '';
  return (
    <div key={key} className="rounded-xl border border-dashed border-slate-200 bg-slate-50 p-4 text-sm text-slate-600">
      <div className="font-semibold text-slate-900">{element.type}</div>
      {element.text && <p className="mt-1">{element.text}</p>}
      {hasDescription && <p className="mt-2 text-slate-500">{String(description)}</p>}
      {element.children?.map((child, idx) => renderRadixElement(child, `${key}:${idx}`, context))}
    </div>
  );
}

export function renderRadixLayout(layout: DisplayLayout, context: RadixRenderContext = {}): React.ReactNode {
  const layoutClasses = getRadixLayoutClasses(layout);
  return (
    <div className={layoutClasses}>
      {layout.children.map((child, index) => renderRadixElement(child, `layout:${index}`, context))}
    </div>
  );
}

export function renderRadixDocument(document: DisplayDocument, context: RadixRenderContext = {}): React.ReactNode {
  const metaDescription = document.meta?.description;
  const hasMetaDescription = metaDescription !== undefined && metaDescription !== null && metaDescription !== '';
  return (
    <div className="space-y-6">
      {document.title && (
        <div className="space-y-1">
          <h1 className="text-3xl font-semibold tracking-tight text-slate-900">{document.title}</h1>
          {hasMetaDescription && <p className="text-slate-500">{String(metaDescription)}</p>}
        </div>
      )}
      {renderRadixLayout(document.layout, context)}
    </div>
  );
}

function getRadixLayoutClasses(layout: DisplayLayout): string {
  const base: string[] = [];
  switch (layout.type) {
    case 'row':
      base.push('flex flex-col gap-4 md:flex-row');
      break;
    case 'grid':
      base.push('grid gap-4');
      base.push(layout.columns ? `md:grid-cols-${Math.min(layout.columns, 4)}` : 'md:grid-cols-2');
      break;
    case 'card':
      base.push('rounded-2xl border border-slate-200 bg-white p-6 shadow-sm');
      break;
    case 'page':
      base.push('rounded-3xl border border-slate-100 bg-white/60 p-8 shadow');
      break;
    default:
      base.push('flex flex-col gap-4');
  }

  if (layout.gap) {
    base.push(`gap-${Math.min(layout.gap, 8)}`);
  }
  if (layout.padding) {
    base.push(`p-${Math.min(layout.padding, 8)}`);
  }

  return base.join(' ');
}

export class RadixAdapter {
  private context: RadixRenderContext;

  constructor(context: RadixRenderContext = {}) {
    this.context = context;
  }

  register(type: string, renderer: RadixRenderer): void {
    registerRadixComponent(type, renderer);
  }

  render(document: DisplayDocument, overrides: RadixRenderContext = {}): React.ReactNode {
    return renderRadixDocument(document, { ...this.context, ...overrides });
  }
}

export const radixAdapter = new RadixAdapter();

// ---------------------------------------------------------------------------
// Default component registry
// ---------------------------------------------------------------------------

registerRadixComponent('text', (element, key) => (
  <span key={key} className="text-sm leading-6 text-slate-600">
    {element.text}
  </span>
));

registerRadixComponent('heading', (element, key) => {
  const level = (element.props?.level as number) || 2;
  const sizes = ['text-3xl', 'text-2xl', 'text-xl', 'text-lg', 'text-base', 'text-sm'];
  const Tag = level === 1 ? 'h1' : level === 2 ? 'h2' : level === 3 ? 'h3' : level === 4 ? 'h4' : level === 5 ? 'h5' : 'h6';
  return React.createElement(
    Tag,
    {
      key,
      className: `${sizes[level - 1] || 'text-lg'} font-semibold text-slate-900`,
    },
    element.text
  );
});

registerRadixComponent('paragraph', (element, key) => (
  <p key={key} className="text-base leading-7 text-slate-600">
    {element.text}
  </p>
));

registerRadixComponent('stack', (element, key, context) => (
  <div key={key} className="flex flex-col gap-4">
    {element.children?.map((child, idx) => renderRadixElement(child, `${key}:${idx}`, context))}
  </div>
));

registerRadixComponent('row', (element, key, context) => (
  <div key={key} className="flex flex-col gap-4 md:flex-row">
    {element.children?.map((child, idx) => renderRadixElement(child, `${key}:${idx}`, context))}
  </div>
));

registerRadixComponent('grid', (element, key, context) => {
  const cols = (element.props?.columns as number) || 2;
  return (
    <div key={key} className={`grid gap-4 md:grid-cols-${Math.min(cols, 4)}`}>
      {element.children?.map((child, idx) => renderRadixElement(child, `${key}:${idx}`, context))}
    </div>
  );
});

registerRadixComponent('card', (element, key, context) => {
  const props = element.props ?? {};
  const title = props.title;
  const subtitle = props.subtitle;
  const badge = props.badge;
  const hasHeader = Boolean(title ?? subtitle ?? badge);
  const badgeColor = props.badgeColor as string | undefined;

  return (
    <RadixCard key={key}>
      {hasHeader && (
        <RadixCardHeader>
          <div>
            {title !== undefined && title !== null && <RadixCardTitle>{String(title)}</RadixCardTitle>}
            {subtitle !== undefined && subtitle !== null && <RadixCardSubtitle>{String(subtitle)}</RadixCardSubtitle>}
          </div>
          {badge !== undefined && badge !== null && (
            <RadixBadge color={(badgeColor as any) ?? 'slate'}>{String(badge)}</RadixBadge>
          )}
        </RadixCardHeader>
      )}
      <RadixCardContent>
        {element.children?.map((child, idx) => renderRadixElement(child, `${key}:content:${idx}`, context))}
      </RadixCardContent>
      {renderActions(element, context)}
    </RadixCard>
  );
});

registerRadixComponent('metric', (element, key, context) => {
  const props = element.props ?? {};
  const metricKey = props.metric as string | undefined;
  const value = metricKey ? context.snapshot?.metrics?.[metricKey] : props.value;
  const label = props.label ?? metricKey;
  const hasDelta = typeof props.delta === 'number';
  const delta = hasDelta ? (props.delta as number) : undefined;
  const trend = (props.trend as 'up' | 'down' | 'flat' | undefined) ?? 'up';

  return (
    <RadixCard key={key} className="bg-slate-900 text-white">
      <RadixCardContent>
        {label !== undefined && label !== null && <RadixMetricLabel>{String(label)}</RadixMetricLabel>}
        <RadixMetricValue value={value} unit={props.unit as string | undefined} />
        {hasDelta && (
          <div className="mt-3">
            <RadixMetricTrend delta={delta} trend={trend} />
          </div>
        )}
      </RadixCardContent>
    </RadixCard>
  );
});

registerRadixComponent('table', (element, key, context) => {
  const props = element.props ?? {};
  const collectionKey = props.collection as string | undefined;
  const rows = collectionKey ? context.snapshot?.collections?.[collectionKey] : (props.rows as Record<string, unknown>[] | undefined);
  const data = Array.isArray(rows) ? rows : [];
  const columnsProp = props.columns as Array<{ id: string; label: string }> | undefined;
  const columns = columnsProp ?? inferColumns(data);
  return <RadixTable key={key} columns={columns} rows={data} emptyLabel={props.emptyLabel as string | undefined} />;
});

registerRadixComponent('list', (element, key) => {
  const items = (element.props?.items as Array<{ label: string; value?: string }>) ?? [];
  return (
    <ul key={key} className="divide-y divide-slate-200 rounded-2xl border border-slate-200 bg-white">
      {items.map((item, idx) => (
        <li key={idx} className="flex items-center justify-between px-4 py-3 text-sm text-slate-700">
          <span>{item.label}</span>
          {item.value && <span className="font-medium text-slate-900">{item.value}</span>}
        </li>
      ))}
    </ul>
  );
});

registerRadixComponent('actions', (element, key, context) => (
  <div key={key} className="flex flex-wrap gap-3">
    {element.children?.map((child, idx) => renderRadixElement(child, `${key}:action:${idx}`, context))}
  </div>
));

registerRadixComponent('button', (element, key, context) => {
  const props = element.props ?? {};
  const rawLabel = props.label ?? element.text ?? 'Action';
  const label = typeof rawLabel === 'string' ? rawLabel : String(rawLabel);
  const actionId = String(props.id || props.actionId || 'action');
  return (
    <RadixButton
      key={key}
      variant={(props.variant as any) ?? 'primary'}
      onClick={() => context.handler?.onAction?.(actionId, props)}
    >
      {label}
    </RadixButton>
  );
});

registerRadixComponent('json', (element, key) => (
  <pre key={key} className="rounded-2xl bg-slate-950 p-4 text-xs text-slate-100">
    {JSON.stringify(element.props?.value ?? element.props, null, 2)}
  </pre>
));

function renderActions(element: DisplayElement, context: RadixRenderContext): React.ReactNode {
  const actions = element.props?.actions as Array<{
    id: string;
    label: string;
    variant?: 'primary' | 'secondary' | 'ghost';
  }> | undefined;

  if (!actions?.length) {
    return null;
  }

  return (
    <RadixCardFooter>
      {actions.map(action => (
        <RadixButton
          key={action.id}
          variant={action.variant ?? 'secondary'}
          onClick={() => context.handler?.onAction?.(action.id, element.props)}
        >
          {action.label}
        </RadixButton>
      ))}
    </RadixCardFooter>
  );
}




// --- New Radix Primitives Renderers ---

registerRadixComponent('dialog', (element, key, context) => {
  const props = element.props as unknown as DialogShape;
  // Placeholder for actual Dialog implementation
  // In a real app, this would use Radix UI Dialog primitives
  return (
    <div key={key} className="p-4 border border-blue-200 rounded bg-blue-50">
      <div className="font-bold">Dialog: {props.title}</div>
      <div>{renderRadixElement(props.trigger, `${key}:trigger`, context)}</div>
      {props.open && (
        <div className="mt-2 p-2 bg-white border border-gray-200 rounded">
          {renderRadixElement(props.content, `${key}:content`, context)}
        </div>
      )}
    </div>
  );
});

registerRadixComponent('popover', (element, key, context) => {
  const props = element.props as unknown as PopoverShape;
  return (
    <div key={key} className="inline-block relative">
      {renderRadixElement(props.trigger, `${key}:trigger`, context)}
      {props.open && (
        <div className="absolute z-10 p-2 bg-white border border-gray-200 rounded shadow-lg">
          {renderRadixElement(props.content, `${key}:content`, context)}
        </div>
      )}
    </div>
  );
});

registerRadixComponent('dropdown-menu', (element, key, context) => {
  const props = element.props as unknown as DropdownMenuShape;
  return (
    <div key={key} className="inline-block relative">
      {renderRadixElement(props.trigger, `${key}:trigger`, context)}
      {props.open && (
        <ul className="absolute z-10 bg-white border border-gray-200 rounded shadow-lg min-w-[150px]">
          {props.items.map((item, idx) => (
            <li key={idx} className="px-4 py-2 hover:bg-gray-100 cursor-pointer">
              {item.label}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
});

registerRadixComponent('tabs', (element, key, context) => {
  const props = element.props as unknown as TabsShape;
  return (
    <div key={key} className="flex flex-col">
      <div className="flex border-b border-gray-200">
        {props.triggers.map((trigger, idx) => (
          <button key={idx} className="px-4 py-2 hover:bg-gray-50">
            {trigger.label}
          </button>
        ))}
      </div>
      <div className="p-4">
        {props.contents.map((content, idx) => (
          <div key={idx} className={content.value === props.defaultValue ? 'block' : 'hidden'}>
            {renderRadixElement(content.content, `${key}:content:${idx}`, context)}
          </div>
        ))}
      </div>
    </div>
  );
});

registerRadixComponent('accordion', (element, key, context) => {
  const props = element.props as unknown as AccordionShape;
  return (
    <div key={key} className="border border-gray-200 rounded">
      {props.items.map((item, idx) => (
        <div key={idx} className="border-b border-gray-200 last:border-0">
          <button className="w-full px-4 py-2 text-left hover:bg-gray-50 font-medium">
            {item.trigger}
          </button>
          <div className="p-4">
            {renderRadixElement(item.content, `${key}:content:${idx}`, context)}
          </div>
        </div>
      ))}
    </div>
  );
});

registerRadixComponent('tooltip', (element, key, context) => {
  const props = element.props as unknown as TooltipShape;
  return (
    <div key={key} className="inline-block relative group">
      {renderRadixElement(props.trigger, `${key}:trigger`, context)}
      <div className="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-2 py-1 bg-black text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap">
        {props.content}
      </div>
    </div>
  );
});

registerRadixComponent('scroll-area', (element, key, context) => {
  const props = element.props as unknown as ScrollAreaShape;
  return (
    <div key={key} className="overflow-auto max-h-[300px] border border-gray-200 rounded">
      {renderRadixElement(props.content, `${key}:content`, context)}
    </div>
  );
});

function inferColumns(rows: Array<Record<string, unknown>>): Array<{ id: string; label: string }> {
  if (!rows.length) {
    return [{ id: 'value', label: 'Value' }];
  }
  const sample = rows[0];
  return Object.keys(sample).map(key => ({ id: key, label: key.replace(/([A-Z])/g, ' $1').replace(/^./, c => c.toUpperCase()) }));
}
