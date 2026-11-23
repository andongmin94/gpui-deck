import { Link } from "fumapress/client";
import { ArrowLeft, ArrowRight } from "lucide-react";

import { CatalogLayout } from "../catalog-layout";
import { type Block, blockCopy } from "./block-copy";
import { BlockViewer } from "./block-viewer";
import { InstallCommand } from "./install-command";

type PageInfo = { title: string; description?: string };
type GalleryEntry = PageInfo & { block: Block };

export function BlocksGallery({
  lang,
  page,
  entries,
}: {
  lang: string;
  page: PageInfo;
  entries: GalleryEntry[];
}) {
  return (
    <CatalogLayout lang={lang}>
      <div className="mx-auto flex max-w-2xl flex-col items-center py-10 text-center sm:py-16">
        <h1 className="text-4xl font-semibold tracking-[-0.045em] sm:text-5xl">{page.title}</h1>
        <p className="mt-4 max-w-xl text-base leading-relaxed text-balance text-fd-muted-foreground sm:text-lg">
          {page.description}
        </p>
      </div>
      <div className="grid gap-5 sm:grid-cols-2 lg:grid-cols-3">
        {entries.map(({ block, title, description }) => (
          <article
            key={block.name}
            className="flex min-w-0 flex-col overflow-hidden rounded-2xl border bg-fd-background shadow-xs"
          >
            <Link
              href={`/${lang}/blocks/${block.name}`}
              className="group block focus-visible:outline-offset-[-3px]"
            >
              <div className="border-b bg-fd-secondary/50 p-3 sm:p-4">
                <div className="overflow-hidden rounded-lg border bg-white shadow-sm group-hover:shadow-md">
                  <img
                    src={`${block.poster}.webp`}
                    width={1440}
                    height={820}
                    alt=""
                    data-block={block.name}
                    className="block h-auto w-full data-[block=login]:scale-[1.65] data-[block=sidebar-settings]:origin-top data-[block=sidebar-settings]:scale-[1.16]"
                  />
                </div>
              </div>
              <h2 className="flex items-center justify-between gap-2 px-5 pt-5 text-xl font-semibold tracking-tight group-hover:underline">
                {title}
                <ArrowRight aria-hidden className="size-4 shrink-0" />
              </h2>
            </Link>
            <p className="mt-2 flex-1 px-5 text-sm leading-relaxed text-fd-muted-foreground">
              {description}
            </p>
            <div className="mt-4 flex flex-wrap items-center gap-1.5 px-5">
              {block.components.map(({ label, path }) => (
                <Link
                  key={path}
                  href={`/${lang}/docs/${path}`}
                  className="rounded-full border px-2.5 py-1 text-xs font-medium hover:bg-fd-secondary"
                >
                  {label}
                </Link>
              ))}
            </div>
            <div className="mt-4 border-t px-3 py-1.5">
              <InstallCommand lang={lang} command={block.command} compact />
            </div>
          </article>
        ))}
      </div>
    </CatalogLayout>
  );
}

export function BlockDetail({ lang, page, block }: { lang: string; page: PageInfo; block: Block }) {
  const t = blockCopy(lang);
  return (
    <CatalogLayout lang={lang}>
      <div className="py-7 sm:pt-11 sm:pb-7">
        <Link
          href={`/${lang}/blocks`}
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
      <div className="mb-5 flex flex-col gap-2">
        <div className="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
          <div className="w-full min-w-0 max-w-[53rem]">
            <InstallCommand key={block.name} lang={lang} command={block.command} />
          </div>
          <Link
            href={`/${lang}/docs/guides/blocks`}
            className="inline-flex min-h-8 shrink-0 items-center gap-1.5 self-start rounded px-1 text-sm font-medium text-fd-muted-foreground hover:text-fd-foreground sm:self-center"
          >
            {t.setup}
            <ArrowRight aria-hidden className="size-3.5" />
          </Link>
        </div>
        <p className="flex flex-wrap items-baseline gap-2 px-1 text-xs text-fd-muted-foreground">
          {t.view}
          <code className="font-mono text-fd-foreground">{block.view}</code>
        </p>
      </div>
      <BlockViewer key={block.name} lang={lang} title={page.title} block={block} />
    </CatalogLayout>
  );
}
