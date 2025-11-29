import React from 'react';

function cx(...classes: Array<string | undefined | null | false>): string {
  return classes.filter(Boolean).join(' ');
}

function baseClasses(base: string, className?: string): string {
  return cx(base, className);
}

export const RadixCard = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={baseClasses('rounded-2xl border border-slate-200 bg-white shadow-sm', className)}
      {...props}
    />
  )
);
RadixCard.displayName = 'RadixCard';

export const RadixCardHeader = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={baseClasses('flex items-center justify-between gap-4 p-6', className)} {...props} />
  )
);
RadixCardHeader.displayName = 'RadixCardHeader';

export const RadixCardTitle = React.forwardRef<HTMLHeadingElement, React.HTMLAttributes<HTMLHeadingElement>>(
  ({ className, ...props }, ref) => (
    <h3 ref={ref} className={baseClasses('text-base font-medium text-slate-900', className)} {...props} />
  )
);
RadixCardTitle.displayName = 'RadixCardTitle';

export const RadixCardSubtitle = React.forwardRef<HTMLParagraphElement, React.HTMLAttributes<HTMLParagraphElement>>(
  ({ className, ...props }, ref) => (
    <p ref={ref} className={baseClasses('text-sm text-slate-500', className)} {...props} />
  )
);
RadixCardSubtitle.displayName = 'RadixCardSubtitle';

export const RadixCardContent = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={baseClasses('px-6 pb-6', className)} {...props} />
  )
);
RadixCardContent.displayName = 'RadixCardContent';

export const RadixCardFooter = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={baseClasses('flex flex-wrap items-center gap-3 px-6 pb-6', className)} {...props} />
  )
);
RadixCardFooter.displayName = 'RadixCardFooter';

export const RadixBadge = ({ color = 'slate', children }: { color?: 'slate' | 'blue' | 'green' | 'amber'; children: React.ReactNode }) => {
  const palette: Record<string, string> = {
    slate: 'bg-slate-100 text-slate-700',
    blue: 'bg-blue-100 text-blue-700',
    green: 'bg-emerald-100 text-emerald-700',
    amber: 'bg-amber-100 text-amber-700',
  };
  return <span className={baseClasses('inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium', palette[color])}>{children}</span>;
};

export const RadixButton = React.forwardRef<
  HTMLButtonElement,
  React.ButtonHTMLAttributes<HTMLButtonElement> & { variant?: 'primary' | 'secondary' | 'ghost' }
>(({ className, variant = 'primary', ...props }, ref) => {
  const variants: Record<string, string> = {
    primary: 'bg-slate-900 text-white hover:bg-slate-800',
    secondary: 'bg-white text-slate-900 border border-slate-200 hover:bg-slate-50',
    ghost: 'text-slate-600 hover:bg-slate-100',
  };
  return (
    <button
      ref={ref}
      className={baseClasses('rounded-lg px-4 py-2 text-sm font-medium transition', cx(variants[variant], className))}
      {...props}
    />
  );
});
RadixButton.displayName = 'RadixButton';

export function RadixTable({
  columns,
  rows,
  emptyLabel = 'No records',
}: {
  columns: Array<{ id: string; label: string }>;
  rows: Array<Record<string, unknown>>;
  emptyLabel?: string;
}) {
  if (!rows.length) {
    return <div className="rounded-2xl border border-dashed border-slate-200 bg-slate-50 p-6 text-sm text-slate-500">{emptyLabel}</div>;
  }

  return (
    <div className="overflow-hidden rounded-2xl border border-slate-200">
      <table className="min-w-full divide-y divide-slate-200">
        <thead className="bg-slate-50 text-left text-xs font-semibold uppercase tracking-wide text-slate-500">
          <tr>
            {columns.map(column => (
              <th key={column.id} scope="col" className="px-4 py-3">
                {column.label}
              </th>
            ))}
          </tr>
        </thead>
        <tbody className="divide-y divide-slate-100 bg-white text-sm text-slate-700">
          {rows.map((row, rowIndex) => (
            <tr key={rowIndex}>
              {columns.map(column => (
                <td key={column.id} className="px-4 py-3">
                  {formatCellValue(row[column.id])}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function formatCellValue(value: unknown): React.ReactNode {
  if (value == null) return '—';
  if (typeof value === 'number') return value.toLocaleString();
  if (value instanceof Date) return value.toLocaleDateString();
  if (Array.isArray(value)) return value.join(', ');
  if (typeof value === 'object') return JSON.stringify(value);
  return String(value);
}

export const RadixMetricValue = ({ value, unit }: { value: unknown; unit?: string }) => (
  <div className="text-3xl font-semibold text-slate-900">
    {formatMetricValue(value)}
    {unit && <span className="ml-1 text-sm font-medium text-slate-500">{unit}</span>}
  </div>
);

export const RadixMetricLabel = ({ children }: { children: React.ReactNode }) => (
  <div className="text-sm font-medium text-slate-500">{children}</div>
);

export const RadixMetricTrend = ({
  delta,
  trend,
}: {
  delta?: number;
  trend?: 'up' | 'down' | 'flat';
}) => {
  if (delta == null) return null;
  const palette = trend === 'down' ? 'text-rose-600' : trend === 'flat' ? 'text-slate-500' : 'text-emerald-600';
  const icon = trend === 'down' ? '↓' : trend === 'flat' ? '→' : '↑';
  return (
    <div className={cx('inline-flex items-center text-sm font-medium', palette)}>
      <span className="mr-1 text-base">{icon}</span>
      {delta}% vs prev
    </div>
  );
};

function formatMetricValue(value: unknown): React.ReactNode {
  if (value == null) return '—';
  if (typeof value === 'number') return value.toLocaleString();
  if (value instanceof Date) return value.toLocaleDateString();
  return String(value);
}
