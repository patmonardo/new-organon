import Link from "next/link";
import { cn } from "../lib/utils";
import { NavLinksRenderer, NavItem, defaultNavItems } from "./navlinks";

/**
 * SideNavProps - Configuration for sidebar navigation
 */
export interface SideNavProps {
  logo?: React.ReactNode;
  logoHref?: string;
  navItems?: NavItem[];
  footer?: React.ReactNode;
  className?: string;
}

/**
 * SideNavRenderer - Renders a sidebar navigation with logo, links, and footer
 */
export function SideNavRenderer({
  logo,
  logoHref = "/",
  navItems = defaultNavItems,
  footer,
  className,
}: SideNavProps) {
  return (
    <div
      className={cn(
        "flex h-full flex-col p-4 border-r bg-white",
        className
      )}
    >
      {/* Logo/Header */}
      <div className="mb-6">
        <Link
          href={logoHref}
          className="flex items-center rounded-md p-2 h-14 transition-colors hover:bg-gray-100"
        >
          {logo || <DefaultLogo />}
        </Link>
      </div>

      {/* Navigation Links */}
      <div className="flex flex-col grow">
        <NavLinksRenderer items={navItems} variant="vertical" />
        <div className="flex-1" />
      </div>

      {/* Footer */}
      {footer && (
        <div className="mt-auto">
          {footer}
        </div>
      )}
    </div>
  );
}

/**
 * Default logo component
 */
function DefaultLogo() {
  return (
    <div className="flex items-center gap-2">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        className="h-8 w-8 text-blue-600"
      >
        <circle cx="12" cy="12" r="10" />
        <polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" />
      </svg>
      <span className="text-xl font-bold text-gray-900">Organon</span>
    </div>
  );
}

/**
 * Default footer with sign out button
 */
export function DefaultFooter({ onSignOut }: { onSignOut?: () => void }) {
  return (
    <button
      onClick={onSignOut}
      className="flex w-full items-center gap-2 rounded-md p-2 text-sm font-medium transition-colors hover:bg-gray-100"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        className="h-5 w-5"
      >
        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
        <polyline points="16 17 21 12 16 7" />
        <line x1="21" y1="12" x2="9" y2="12" />
      </svg>
      <span>Sign Out</span>
    </button>
  );
}

