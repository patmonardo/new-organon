import React from 'react';
import type { DisplayDocument, DisplayElement, DisplayLayout, FormHandler } from '../../../sdsl/types';
import type { HydratorSnapshot } from '../../../execution/semantic-hydrator';
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
} from '../../../schema/radix';

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

// --- New Radix Primitives Renderers ---

registerRadixComponent('dialog', (element, key, context) => {
  const props = element.props as unknown as DialogShape;
  const controller = (context as any).controller;

  // Determine open state:
  // 1. Controlled by parent (props.open)
  // 2. Controlled by controller state (using element ID or auto-generated ID)
  // 3. Default (false)
  const id = props.id || `dialog-${key}`;
  const isOpen = props.open ?? controller?.state?.openStates?.[id] ?? props.defaultOpen ?? false;

  const handleOpenChange = (open: boolean) => {
    controller?.setOpen?.(id, open);
  };

  return (
    <div key={key} className="p-4 border border-blue-200 rounded bg-blue-50 relative">
      <div className="font-bold mb-2">Dialog: {props.title}</div>

      {/* Trigger */}
      <div onClick={() => handleOpenChange(true)}>
        {renderRadixElement(props.trigger, `${key}:trigger`, context)}
      </div>

      {/* Content (Overlay) */}
      {isOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
          <div className="w-full max-w-lg rounded-xl bg-white p-6 shadow-2xl animate-in fade-in zoom-in-95 duration-200">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-semibold">{props.title}</h3>
              <button
                onClick={() => handleOpenChange(false)}
                className="rounded-full p-1 hover:bg-slate-100"
              >
                ✕
              </button>
            </div>
            {props.description && (
              <p className="mb-4 text-sm text-slate-500">{props.description}</p>
            )}
            {renderRadixElement(props.content, `${key}:content`, context)}
          </div>
        </div>
      )}
    </div>
  );
});

registerRadixComponent('popover', (element, key, context) => {
  const props = element.props as unknown as PopoverShape;
  const controller = (context as any).controller;
  const id = props.id || `popover-${key}`;
  const isOpen = props.open ?? controller?.state?.openStates?.[id] ?? props.defaultOpen ?? false;

  const handleToggle = () => {
    controller?.setOpen?.(id, !isOpen);
  };

  return (
    <div key={key} className="inline-block relative">
      <div onClick={handleToggle}>
        {renderRadixElement(props.trigger, `${key}:trigger`, context)}
      </div>
      {isOpen && (
        <div className="absolute z-10 mt-2 w-64 p-4 bg-white border border-slate-200 rounded-xl shadow-xl animate-in fade-in slide-in-from-top-2 duration-200">
          {renderRadixElement(props.content, `${key}:content`, context)}
        </div>
      )}
    </div>
  );
});

registerRadixComponent('dropdown-menu', (element, key, context) => {
  const props = element.props as unknown as DropdownMenuShape;
  const controller = (context as any).controller;
  const id = props.id || `dropdown-${key}`;
  const isOpen = props.open ?? controller?.state?.openStates?.[id] ?? props.defaultOpen ?? false;

  const handleToggle = () => {
    controller?.setOpen?.(id, !isOpen);
  };

  return (
    <div key={key} className="inline-block relative">
      <div onClick={handleToggle}>
        {renderRadixElement(props.trigger, `${key}:trigger`, context)}
      </div>
      {isOpen && (
        <ul className="absolute right-0 z-10 mt-2 min-w-[180px] overflow-hidden rounded-lg border border-slate-200 bg-white p-1 shadow-lg animate-in fade-in zoom-in-95 duration-100">
          {props.items.map((item, idx) => (
            <li
              key={idx}
              className={`
                flex items-center justify-between px-2 py-1.5 text-sm rounded-md cursor-pointer
                ${item.disabled ? 'opacity-50 cursor-not-allowed' : 'hover:bg-slate-100'}
              `}
              onClick={() => {
                if (item.disabled) return;
                if (item.onSelect) context.handler?.onAction?.(item.onSelect);
                handleToggle(); // Close on select
              }}
            >
              <span className="flex items-center gap-2">
                {item.icon && <span>{item.icon}</span>}
                {item.label}
              </span>
              {item.shortcut && <span className="text-xs text-slate-400">{item.shortcut}</span>}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
});

registerRadixComponent('tabs', (element, key, context) => {
  const props = element.props as unknown as TabsShape;
  const controller = (context as any).controller;
  const id = props.id || `tabs-${key}`;
  const value = props.value ?? controller?.state?.valueStates?.[id] ?? props.defaultValue;

  const handleValueChange = (val: string) => {
    controller?.setValue?.(id, val);
  };

  return (
    <div key={key} className="flex flex-col gap-4">
      <div className="flex border-b border-slate-200">
        {props.triggers.map((trigger, idx) => {
          const isActive = trigger.value === value;
          return (
            <button
              key={idx}
              onClick={() => handleValueChange(trigger.value)}
              className={`
                px-4 py-2 text-sm font-medium border-b-2 transition-colors
                ${isActive ? 'border-slate-900 text-slate-900' : 'border-transparent text-slate-500 hover:text-slate-700 hover:border-slate-300'}
              `}
            >
              {trigger.icon && <span className="mr-2">{trigger.icon}</span>}
              {trigger.label}
            </button>
          );
        })}
      </div>
      <div className="mt-2">
        {props.contents.map((content, idx) => {
          if (content.value !== value) return null;
          return (
            <div key={idx} className="animate-in fade-in duration-200">
              {renderRadixElement(content.content, `${key}:content:${idx}`, context)}
            </div>
          );
        })}
      </div>
    </div>
  );
});

registerRadixComponent('accordion', (element, key, context) => {
  const props = element.props as unknown as AccordionShape;
  const controller = (context as any).controller;
  const id = props.id || `accordion-${key}`;

  // Handle single vs multiple
  const rawValue = props.value ?? controller?.state?.valueStates?.[id] ?? props.defaultValue;
  const value = Array.isArray(rawValue) ? rawValue : [rawValue].filter(Boolean) as string[];

  const handleToggle = (itemValue: string) => {
    let newValue: string | string[];
    if (props.typeProp === 'multiple') {
      newValue = value.includes(itemValue)
        ? value.filter(v => v !== itemValue)
        : [...value, itemValue];
    } else {
      // Single mode
      // If collapsible and already open, close it. Otherwise open it.
      newValue = (props.collapsible && value.includes(itemValue)) ? [] : [itemValue];
    }
    controller?.setValue?.(id, newValue);
  };

  return (
    <div key={key} className="divide-y divide-slate-200 rounded-lg border border-slate-200 bg-white">
      {props.items.map((item, idx) => {
        const isOpen = value.includes(item.value);
        return (
          <div key={idx}>
            <button
              onClick={() => handleToggle(item.value)}
              className="flex w-full items-center justify-between px-4 py-3 text-left text-sm font-medium hover:bg-slate-50 transition-colors"
            >
              {item.trigger}
              <span className={`transform transition-transform duration-200 ${isOpen ? 'rotate-180' : ''}`}>
                ↓
              </span>
            </button>
            {isOpen && (
              <div className="px-4 pb-4 pt-0 text-sm text-slate-600 animate-in slide-in-from-top-1 duration-200">
                {renderRadixElement(item.content, `${key}:content:${idx}`, context)}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
});

registerRadixComponent('tooltip', (element, key, context) => {
  const props = element.props as unknown as TooltipShape;
  return (
    <div key={key} className="inline-block relative group">
      {renderRadixElement(props.trigger, `${key}:trigger`, context)}
      <div className="
        absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-2 py-1
        bg-slate-900 text-white text-xs rounded shadow-lg
        opacity-0 group-hover:opacity-100 transition-opacity duration-200
        pointer-events-none whitespace-nowrap z-50
      ">
        {props.content}
        {/* Arrow */}
        <div className="absolute top-full left-1/2 -translate-x-1/2 -mt-1 border-4 border-transparent border-t-slate-900"></div>
      </div>
    </div>
  );
});

registerRadixComponent('scroll-area', (element, key, context) => {
  const props = element.props as unknown as ScrollAreaShape;
  return (
    <div key={key} className="overflow-auto max-h-[300px] rounded-lg border border-slate-200 pr-2">
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
