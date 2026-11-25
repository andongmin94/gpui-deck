"use client";

import type * as PageTree from "fumadocs-core/page-tree";
import type { TOCItemType } from "fumadocs-core/toc";
import { Link } from "fumapress/client";
import { ArrowLeft, ArrowRight } from "lucide-react";
import type { ReactNode } from "react";

import { copyFor } from "./copy";
import { SiteFooter } from "./footer";
import { SiteHeader } from "./header";
import { SidebarNav } from "./sidebar";
import { TableOfContents } from "./toc";

export interface PageLink {
  name: ReactNode;
  url: string;
}

const pagerButton =
  "inline-flex size-8 items-center justify-center rounded-md bg-fd-secondary text-fd-secondary-foreground transition-colors hover:bg-fd-accent";

/** A documentation page in shadcn/ui's frame: sidebar, article and table of contents. */
export function DocsShell({
  lang,
  tree,
  title,
  description,
  toc,
  previous,
  next,
  children,
}: {
  lang: string;
  tree: PageTree.Root;
  title: ReactNode;
  description?: ReactNode;
  toc: TOCItemType[];
  previous?: PageLink;
  next?: PageLink;
  children: ReactNode;
}) {
  const t = copyFor(lang);
  return (
    <div className="flex min-h-svh flex-col">
      <SiteHeader lang={lang} tree={tree} />
      <div className="mx-auto flex w-full max-w-[1400px] flex-1 px-4 md:px-6">
        <aside className="sticky top-14 hidden h-[calc(100svh-3.5rem)] w-60 shrink-0 no-scrollbar overflow-y-auto overscroll-contain py-8 pr-4 lg:block">
          <SidebarNav tree={tree} label={t.docs} />
        </aside>
        <main id="main-content" className="flex min-w-0 flex-1 justify-center py-8 lg:px-8">
          <div className="w-full max-w-[44rem]">
            <div className="flex items-start justify-between gap-4">
              <h1 className="scroll-m-20 text-3xl font-semibold tracking-tight">{title}</h1>
              <div className="flex shrink-0 gap-2 pt-1">
                {previous && (
                  <Link
                    href={previous.url}
                    aria-label={`${t.previous}: ${String(previous.name)}`}
                    className={pagerButton}
                  >
                    <ArrowLeft className="size-4" />
                  </Link>
                )}
                {next && (
                  <Link
                    href={next.url}
                    aria-label={`${t.next}: ${String(next.name)}`}
                    className={pagerButton}
                  >
                    <ArrowRight className="size-4" />
                  </Link>
                )}
              </div>
            </div>
            {description && (
              <p className="mt-2 text-base text-balance text-fd-muted-foreground">{description}</p>
            )}
            <article className="prose mt-8 max-w-none">{children}</article>
            <div className="mt-12 flex items-center justify-between gap-4 border-t pt-6 text-sm">
              {previous ? (
                <Link
                  href={previous.url}
                  className="inline-flex items-center gap-2 rounded-md bg-fd-secondary px-3 py-2 font-medium transition-colors hover:bg-fd-accent"
                >
                  <ArrowLeft className="size-4" />
                  {previous.name}
                </Link>
              ) : (
                <span />
              )}
              {next && (
                <Link
                  href={next.url}
                  className="inline-flex items-center gap-2 rounded-md bg-fd-secondary px-3 py-2 font-medium transition-colors hover:bg-fd-accent"
                >
                  {next.name}
                  <ArrowRight className="size-4" />
                </Link>
              )}
            </div>
          </div>
        </main>
        <aside className="sticky top-14 hidden h-[calc(100svh-3.5rem)] no-scrollbar w-56 shrink-0 overflow-y-auto py-8 xl:block">
          <TableOfContents toc={toc} title={t.onThisPage} />
        </aside>
      </div>
      <SiteFooter lang={lang} />
    </div>
  );
}
