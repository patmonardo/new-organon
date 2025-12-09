import React, { ElementType } from "react";
import { cn } from "../lib/utils";

/**
 * Text component props
 */
export interface TextProps extends React.HTMLAttributes<HTMLElement> {
  variant?: "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "body" | "small" | "caption";
  as?: ElementType;
  weight?: "normal" | "medium" | "semibold" | "bold";
}

/**
 * TextRenderer - Typography component with variants
 */
export function TextRenderer({
  variant = "body",
  as,
  weight,
  className,
  children,
  ...props
}: TextProps) {
  // Variant classes
  const variantClasses = {
    h1: "text-4xl md:text-5xl font-bold tracking-tight",
    h2: "text-3xl md:text-4xl font-bold tracking-tight",
    h3: "text-2xl md:text-3xl font-semibold",
    h4: "text-xl md:text-2xl font-semibold",
    h5: "text-lg md:text-xl font-medium",
    h6: "text-base md:text-lg font-medium",
    body: "text-base",
    small: "text-sm",
    caption: "text-xs text-gray-600",
  }[variant];

  // Weight classes
  const weightClasses = weight
    ? {
        normal: "font-normal",
        medium: "font-medium",
        semibold: "font-semibold",
        bold: "font-bold",
      }[weight]
    : "";

  // Determine element tag
  const Component: ElementType = as || (variant.startsWith("h") ? variant : "p");

  return (
    <Component
      className={cn(variantClasses, weightClasses, className)}
      {...props}
    >
      {children}
    </Component>
  );
}

/**
 * Heading shorthand components
 */
export const H1 = (props: Omit<TextProps, "variant">) => (
  <TextRenderer variant="h1" {...props} />
);
export const H2 = (props: Omit<TextProps, "variant">) => (
  <TextRenderer variant="h2" {...props} />
);
export const H3 = (props: Omit<TextProps, "variant">) => (
  <TextRenderer variant="h3" {...props} />
);
export const H4 = (props: Omit<TextProps, "variant">) => (
  <TextRenderer variant="h4" {...props} />
);
export const H5 = (props: Omit<TextProps, "variant">) => (
  <TextRenderer variant="h5" {...props} />
);
export const H6 = (props: Omit<TextProps, "variant">) => (
  <TextRenderer variant="h6" {...props} />
);

/**
 * Body text shorthand
 */
export const Body = (props: Omit<TextProps, "variant">) => (
  <TextRenderer variant="body" {...props} />
);

/**
 * Small text shorthand
 */
export const Small = (props: Omit<TextProps, "variant">) => (
  <TextRenderer variant="small" {...props} />
);

/**
 * Caption text shorthand
 */
export const Caption = (props: Omit<TextProps, "variant">) => (
  <TextRenderer variant="caption" {...props} />
);

