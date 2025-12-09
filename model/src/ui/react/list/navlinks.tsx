"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn } from "../lib/utils";

/**
 * NavItem - A navigation menu item
 */
export interface NavItem {
  name: string;
  href: string;
  icon?: React.ComponentType<{ className?: string }>;
  disabled?: boolean;
}

/**
 * NavLinksProps - Configuration for navigation links
 */
export interface NavLinksProps {
  items: NavItem[];
  variant?: "vertical" | "horizontal";
  size?: "sm" | "md" | "lg";
  className?: string;
}

/**
 * NavLinksRenderer - Renders navigation links with active state
 */
export function NavLinksRenderer({
  items,
  variant = "vertical",
  size = "md",
  className,
}: NavLinksProps) {
  const pathname = usePathname();

  // Size classes
  const sizeClasses = {
    sm: "text-sm py-1 px-2",
    md: "text-base py-2 px-3",
    lg: "text-lg py-3 px-4",
  }[size];

  // Icon sizing
  const iconSizeClass = {
    sm: "w-4 h-4",
    md: "w-5 h-5",
    lg: "w-6 h-6",
  }[size];

  // Layout classes
  const layoutClasses = variant === "vertical" 
    ? "flex-col space-y-2" 
    : "flex-row space-x-4";

  return (
    <div className={cn("flex", layoutClasses, className)}>
      {items.map((item) => {
        const isActive = pathname === item.href;
        const LinkIcon = item.icon;

        return (
          <Link
            key={item.name}
            href={item.href}
            aria-disabled={item.disabled}
            className={cn(
              "flex items-center gap-2 rounded-md font-medium transition-colors",
              "hover:bg-gray-100 hover:text-gray-900",
              variant === "vertical" ? "w-full" : "",
              sizeClasses,
              isActive && "bg-blue-50 text-blue-600",
              item.disabled && "pointer-events-none opacity-60"
            )}
          >
            {LinkIcon && <LinkIcon className={iconSizeClass} />}
            <span>{item.name}</span>
          </Link>
        );
      })}
    </div>
  );
}

/**
 * Default navigation items for common dashboard
 */
export const defaultNavItems: NavItem[] = [
  {
    name: "Home",
    href: "/",
    icon: (props) => (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        {...props}
      >
        <path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
        <polyline points="9 22 9 12 15 12 15 22" />
      </svg>
    ),
  },
  {
    name: "Dashboard",
    href: "/dashboard",
    icon: (props) => (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        {...props}
      >
        <rect width="7" height="9" x="3" y="3" rx="1" />
        <rect width="7" height="5" x="14" y="3" rx="1" />
        <rect width="7" height="9" x="14" y="12" rx="1" />
        <rect width="7" height="5" x="3" y="16" rx="1" />
      </svg>
    ),
  },
  {
    name: "Invoices",
    href: "/invoices",
    icon: (props) => (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        {...props}
      >
        <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" />
        <polyline points="14 2 14 8 20 8" />
      </svg>
    ),
  },
  {
    name: "Customers",
    href: "/customers",
    icon: (props) => (
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        {...props}
      >
        <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
        <circle cx="9" cy="7" r="4" />
        <path d="M22 21v-2a4 4 0 0 0-3-3.87" />
        <path d="M16 3.13a4 4 0 0 1 0 7.75" />
      </svg>
    ),
  },
];

