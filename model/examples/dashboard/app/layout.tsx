import type { ReactNode } from "react";
import "./globals.css";

export const metadata = {
  title: "Dashboard V4 Demo",
  description: "Example data modeling dashboard",
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body className="min-h-screen bg-gray-50 text-slate-900">{children}</body>
    </html>
  );
}

