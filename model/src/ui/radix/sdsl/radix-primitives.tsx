import React from 'react';

// =============================================================================
// UTILITIES
// =============================================================================

function cx(...classes: Array<string | undefined | null | false>): string {
  return classes.filter(Boolean).join(' ');
}

function baseClasses(base: string, className?: string): string {
  return cx(base, className);
}

// =============================================================================
// LAYOUT COMPONENTS
// =============================================================================

export const RadixPage = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={baseClasses('min-h-screen bg-gradient-to-br from-slate-50 via-white to-slate-100 p-6 md:p-8', className)}
      {...props}
    />
  )
);
RadixPage.displayName = 'RadixPage';

export const RadixPageHeader = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement> & { title?: string; subtitle?: string }>(
  ({ className, title, subtitle, children, ...props }, ref) => (
    <div ref={ref} className={baseClasses('mb-8', className)} {...props}>
      {title && <h1 className="text-3xl font-bold tracking-tight text-slate-900">{title}</h1>}
      {subtitle && <p className="mt-2 text-base text-slate-600">{subtitle}</p>}
      {children}
    </div>
  )
);
RadixPageHeader.displayName = 'RadixPageHeader';

export const RadixGrid = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement> & { columns?: number }>(
  ({ className, columns = 3, ...props }, ref) => {
    const colClass = columns === 2 ? 'md:grid-cols-2' : columns === 4 ? 'md:grid-cols-4' : 'md:grid-cols-3';
    return (
      <div
        ref={ref}
        className={baseClasses(`grid gap-6 ${colClass}`, className)}
        {...props}
      />
    );
  }
);
RadixGrid.displayName = 'RadixGrid';

export const RadixStack = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement> & { gap?: number }>(
  ({ className, gap = 6, ...props }, ref) => (
    <div
      ref={ref}
      className={baseClasses(`flex flex-col gap-${gap}`, className)}
      {...props}
    />
  )
);
RadixStack.displayName = 'RadixStack';

// =============================================================================
// CARD COMPONENTS
// =============================================================================

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

// =============================================================================
// METRIC CARD (Standalone Dashboard Metric)
// =============================================================================

export interface RadixMetricCardProps {
  label: string;
  value: unknown;
  unit?: string;
  delta?: number;
  trend?: 'up' | 'down' | 'flat';
  icon?: React.ReactNode;
  className?: string;
}

export function RadixMetricCard({ label, value, unit, delta, trend, icon, className }: RadixMetricCardProps) {
  return (
    <RadixCard className={cx('overflow-hidden', className)}>
      <RadixCardContent className="pt-6">
        <div className="flex items-start justify-between">
          <div>
            <RadixMetricLabel>{label}</RadixMetricLabel>
            <RadixMetricValue value={value} unit={unit} />
            {delta !== undefined && (
              <div className="mt-2">
                <RadixMetricTrend delta={delta} trend={trend} />
              </div>
            )}
          </div>
          {icon && (
            <div className="rounded-xl bg-slate-100 p-3 text-slate-600">
              {icon}
            </div>
          )}
        </div>
      </RadixCardContent>
    </RadixCard>
  );
}

// =============================================================================
// AVATAR
// =============================================================================

export interface RadixAvatarProps {
  src?: string;
  alt?: string;
  fallback?: string;
  size?: 'sm' | 'md' | 'lg' | 'xl';
  className?: string;
}

export function RadixAvatar({ src, alt, fallback, size = 'md', className }: RadixAvatarProps) {
  const sizeClasses = {
    sm: 'h-8 w-8 text-xs',
    md: 'h-10 w-10 text-sm',
    lg: 'h-12 w-12 text-base',
    xl: 'h-16 w-16 text-lg',
  };

  const [error, setError] = React.useState(false);

  if (src && !error) {
    return (
      <img
        src={src}
        alt={alt || 'Avatar'}
        onError={() => setError(true)}
        className={cx('rounded-full object-cover', sizeClasses[size], className)}
      />
    );
  }

  // Fallback
  const initials = fallback || (alt ? alt.split(' ').map(n => n[0]).join('').slice(0, 2) : '??');
  return (
    <div className={cx(
      'flex items-center justify-center rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 font-semibold text-white',
      sizeClasses[size],
      className
    )}>
      {initials.toUpperCase()}
    </div>
  );
}

// =============================================================================
// STATUS BADGE
// =============================================================================

export interface RadixStatusBadgeProps {
  status: string;
  className?: string;
}

export function RadixStatusBadge({ status, className }: RadixStatusBadgeProps) {
  const statusColors: Record<string, string> = {
    PAID: 'bg-emerald-100 text-emerald-700',
    PENDING: 'bg-amber-100 text-amber-700',
    OVERDUE: 'bg-rose-100 text-rose-700',
    DRAFT: 'bg-slate-100 text-slate-600',
    ACTIVE: 'bg-emerald-100 text-emerald-700',
    INACTIVE: 'bg-slate-100 text-slate-600',
  };

  const color = statusColors[status.toUpperCase()] || 'bg-slate-100 text-slate-700';

  return (
    <span className={cx(
      'inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium',
      color,
      className
    )}>
      {status}
    </span>
  );
}

// =============================================================================
// FORM FIELD PRIMITIVES
// =============================================================================

export interface RadixInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  hint?: string;
}

export const RadixInput = React.forwardRef<HTMLInputElement, RadixInputProps>(
  ({ label, error, hint, className, id, ...props }, ref) => {
    const inputId = id || `input-${Math.random().toString(36).slice(2)}`;
    return (
      <div className="space-y-1.5">
        {label && (
          <label htmlFor={inputId} className="block text-sm font-medium text-slate-700">
            {label}
            {props.required && <span className="ml-1 text-rose-500">*</span>}
          </label>
        )}
        <input
          ref={ref}
          id={inputId}
          className={cx(
            'block w-full rounded-lg border px-3 py-2 text-sm transition-colors',
            'focus:outline-none focus:ring-2 focus:ring-offset-0',
            error
              ? 'border-rose-300 focus:border-rose-500 focus:ring-rose-500/20'
              : 'border-slate-300 focus:border-slate-900 focus:ring-slate-900/20',
            props.disabled && 'bg-slate-50 text-slate-500',
            className
          )}
          {...props}
        />
        {error && <p className="text-xs text-rose-600">{error}</p>}
        {hint && !error && <p className="text-xs text-slate-500">{hint}</p>}
      </div>
    );
  }
);
RadixInput.displayName = 'RadixInput';

export interface RadixSelectProps extends React.SelectHTMLAttributes<HTMLSelectElement> {
  label?: string;
  error?: string;
  options: Array<{ value: string; label: string }>;
}

export const RadixSelect = React.forwardRef<HTMLSelectElement, RadixSelectProps>(
  ({ label, error, options, className, id, ...props }, ref) => {
    const selectId = id || `select-${Math.random().toString(36).slice(2)}`;
    return (
      <div className="space-y-1.5">
        {label && (
          <label htmlFor={selectId} className="block text-sm font-medium text-slate-700">
            {label}
            {props.required && <span className="ml-1 text-rose-500">*</span>}
          </label>
        )}
        <select
          ref={ref}
          id={selectId}
          className={cx(
            'block w-full rounded-lg border px-3 py-2 text-sm transition-colors',
            'focus:outline-none focus:ring-2 focus:ring-offset-0',
            error
              ? 'border-rose-300 focus:border-rose-500 focus:ring-rose-500/20'
              : 'border-slate-300 focus:border-slate-900 focus:ring-slate-900/20',
            props.disabled && 'bg-slate-50 text-slate-500',
            className
          )}
          {...props}
        >
          {options.map(opt => (
            <option key={opt.value} value={opt.value}>{opt.label}</option>
          ))}
        </select>
        {error && <p className="text-xs text-rose-600">{error}</p>}
      </div>
    );
  }
);
RadixSelect.displayName = 'RadixSelect';

// =============================================================================
// PROFILE HEADER
// =============================================================================

export interface RadixProfileHeaderProps {
  name: string;
  email?: string;
  imageUrl?: string;
  badge?: string;
  badgeColor?: 'slate' | 'blue' | 'green' | 'amber';
  children?: React.ReactNode;
  className?: string;
}

export function RadixProfileHeader({ name, email, imageUrl, badge, badgeColor, children, className }: RadixProfileHeaderProps) {
  return (
    <RadixCard className={className}>
      <RadixCardContent className="pt-6">
        <div className="flex items-start gap-4">
          <RadixAvatar src={imageUrl} alt={name} size="xl" />
          <div className="flex-1">
            <div className="flex items-center gap-3">
              <h2 className="text-xl font-semibold text-slate-900">{name}</h2>
              {badge && <RadixBadge color={badgeColor}>{badge}</RadixBadge>}
            </div>
            {email && <p className="mt-1 text-sm text-slate-600">{email}</p>}
            {children}
          </div>
        </div>
      </RadixCardContent>
    </RadixCard>
  );
}

// =============================================================================
// DATA LIST (Key-Value pairs)
// =============================================================================

export interface RadixDataListProps {
  items: Array<{ label: string; value: React.ReactNode }>;
  columns?: 1 | 2 | 3;
  className?: string;
}

export function RadixDataList({ items, columns = 2, className }: RadixDataListProps) {
  const colClass = columns === 1 ? 'grid-cols-1' : columns === 3 ? 'sm:grid-cols-3' : 'sm:grid-cols-2';
  return (
    <dl className={cx(`grid gap-4 ${colClass}`, className)}>
      {items.map((item, i) => (
        <div key={i} className="space-y-1">
          <dt className="text-sm font-medium text-slate-500">{item.label}</dt>
          <dd className="text-sm text-slate-900">{item.value ?? '—'}</dd>
        </div>
      ))}
    </dl>
  );
}

// =============================================================================
// EMPTY STATE
// =============================================================================

export interface RadixEmptyStateProps {
  title?: string;
  description?: string;
  icon?: React.ReactNode;
  action?: React.ReactNode;
  className?: string;
}

export function RadixEmptyState({ title = 'No data', description, icon, action, className }: RadixEmptyStateProps) {
  return (
    <div className={cx('flex flex-col items-center justify-center rounded-2xl border border-dashed border-slate-200 bg-slate-50 p-8 text-center', className)}>
      {icon && <div className="mb-4 text-slate-400">{icon}</div>}
      <h3 className="text-sm font-medium text-slate-900">{title}</h3>
      {description && <p className="mt-1 text-sm text-slate-500">{description}</p>}
      {action && <div className="mt-4">{action}</div>}
    </div>
  );
}
