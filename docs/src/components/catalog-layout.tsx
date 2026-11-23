import type { ReactNode } from "react";
import { SiteFooter } from "./shell/footer";
import { SiteHeader } from "./shell/header";

export function CatalogLayout({ lang, children }: { lang: string; children: ReactNode }) {
  return (
    <div className="flex min-h-svh flex-col">
      <SiteHeader lang={lang} />
      <main
        id="main-content"
        className="mx-auto w-full min-w-0 max-w-[1320px] flex-1 px-4 pb-12 sm:px-6 lg:px-8 lg:pb-20 [html:lang(ko)_&]:break-keep"
      >
        {children}
      </main>
      <SiteFooter lang={lang} />
    </div>
  );
}
