import { Link } from "fumapress/client";
import { ArrowLeft, ArrowRight, ChartColumn, FileStack, Table2 } from "lucide-react";

import { SiteFooter } from "../shell/footer";
import { SiteHeader } from "../shell/header";
import { billing, blockCopy } from "./block-copy";
import { BlockPoster } from "./block-poster";
import { BlockViewer } from "./block-viewer";
import { InstallCommand } from "./install-command";

type PageInfo = { title: string; description?: string };

export function BlocksPage({
  lang,
  page,
  block,
}: {
  lang: string;
  page: PageInfo;
  block?: PageInfo;
}) {
  const t = blockCopy(lang);
  const gallery = `/${lang}/blocks`;
  const detail = `${gallery}/${billing.name}`;
  const guide = `/${lang}/docs/guides/blocks`;
  return (
    <div className="flex min-h-svh flex-col">
      <SiteHeader lang={lang} />
      <main
        id="main-content"
        className="mx-auto w-full min-w-0 max-w-[1320px] flex-1 px-4 pb-12 sm:px-6 lg:px-8 lg:pb-20 [html:lang(ko)_&]:break-keep"
      >
        {block ? (
          <>
            <div className="mx-auto flex max-w-2xl flex-col items-center py-10 text-center sm:py-16">
              <h1 className="text-4xl font-semibold tracking-[-0.045em] sm:text-5xl">
                {page.title}
              </h1>
              <p className="mt-4 max-w-xl text-base leading-relaxed text-balance text-fd-muted-foreground sm:text-lg">
                {page.description}
              </p>
              <Link
                href={guide}
                className="mt-5 inline-flex min-h-9 items-center gap-2 rounded-full bg-fd-secondary px-4 text-sm font-medium hover:bg-fd-accent"
              >
                <FileStack aria-hidden className="size-4" />
                {t.share}
              </Link>
            </div>
            <article className="flex min-w-0 flex-col-reverse overflow-hidden rounded-3xl border bg-fd-background shadow-xs md:grid md:grid-cols-[minmax(0,1.7fr)_minmax(16rem,1fr)]">
              <Link
                href={detail}
                aria-label={t.open}
                className="group flex min-w-0 items-center justify-center border-t bg-fd-secondary/45 p-3 sm:p-5 md:border-t-0 md:border-e lg:p-8"
              >
                <div className="w-full overflow-hidden rounded-xl border bg-white shadow-sm transition-shadow group-hover:shadow-md">
                  <BlockPoster alt={t.image} />
                </div>
              </Link>
              <div className="flex min-w-0 flex-col items-start justify-center p-5 sm:p-8 lg:p-10">
                <h2 className="text-2xl font-semibold tracking-tight sm:text-3xl">
                  <Link href={detail} className="hover:underline">
                    {block.title}
                  </Link>
                </h2>
                <p className="mt-3 max-w-prose text-sm leading-relaxed text-fd-muted-foreground sm:text-base">
                  {block.description}
                </p>
                <div className="mt-4 flex flex-wrap gap-2">
                  <Link
                    href={`/${lang}/docs/components/chart`}
                    className="inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs font-medium hover:bg-fd-secondary"
                  >
                    <ChartColumn aria-hidden className="size-3.5" />
                    Chart
                  </Link>
                  <Link
                    href={`/${lang}/docs/recipes/data-table`}
                    className="inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs font-medium hover:bg-fd-secondary"
                  >
                    <Table2 aria-hidden className="size-3.5" />
                    DataTable
                  </Link>
                </div>
                <div className="mt-6 flex max-w-full flex-wrap items-center gap-2">
                  <Link
                    href={detail}
                    className="inline-flex min-h-9 items-center gap-2 rounded-full bg-fd-primary px-4 text-sm font-medium text-fd-primary-foreground hover:opacity-85"
                  >
                    {t.open}
                    <ArrowRight aria-hidden className="size-4" />
                  </Link>
                  <InstallCommand lang={lang} compact />
                </div>
              </div>
            </article>
          </>
        ) : (
          <>
            <div className="py-7 sm:pt-11 sm:pb-7">
              <Link
                href={gallery}
                className="inline-flex items-center gap-1.5 rounded text-sm font-medium text-fd-muted-foreground hover:text-fd-foreground"
              >
                <ArrowLeft aria-hidden className="size-4" />
                {t.all}
              </Link>
              <h1 className="mt-5 text-3xl font-semibold tracking-[-0.045em] sm:text-5xl">
                {page.title}
              </h1>
              <p className="mt-3 max-w-2xl text-base leading-relaxed text-fd-muted-foreground sm:text-lg">
                {page.description}
              </p>
            </div>
            <div className="mb-5 flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
              <div className="w-full min-w-0 max-w-[53rem]">
                <InstallCommand lang={lang} />
              </div>
              <Link
                href={guide}
                className="inline-flex min-h-8 shrink-0 items-center gap-1.5 self-start rounded px-1 text-sm font-medium text-fd-muted-foreground hover:text-fd-foreground sm:self-center"
              >
                {t.setup}
                <ArrowRight aria-hidden className="size-3.5" />
              </Link>
            </div>
            <BlockViewer lang={lang} title={page.title} />
          </>
        )}
      </main>
      <SiteFooter lang={lang} />
    </div>
  );
}
