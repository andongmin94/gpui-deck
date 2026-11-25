import { Link } from "fumapress/client";
import { ArrowLeft, ArrowRight, ArrowUpRight, BookOpen } from "lucide-react";

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
  const t = blockCopy(lang);
  return (
    <CatalogLayout lang={lang}>
      <header className="mx-auto flex max-w-3xl flex-col items-center py-8 text-center sm:py-16">
        <h1 className="text-4xl font-semibold tracking-[-0.045em] text-balance sm:text-5xl">
          {t.galleryTitle}
        </h1>
        <p className="mt-4 max-w-xl text-base leading-relaxed text-balance text-fd-muted-foreground sm:text-lg">
          {page.description}
        </p>
      </header>
      <div className="mb-7 flex flex-wrap items-center justify-between gap-x-4 gap-y-2 border-b pb-3">
        <nav aria-label={t.all} className="flex flex-wrap items-center sm:gap-1">
          {entries.map(({ block, title }) => (
            <a
              key={block.name}
              href={`#${block.name}`}
              aria-label={t.jumpTo(title)}
              className="inline-flex min-h-9 items-center rounded-md px-2 text-sm font-medium text-fd-muted-foreground hover:bg-fd-secondary hover:text-fd-foreground"
            >
              {title}
            </a>
          ))}
        </nav>
        <Link
          href={`/${lang}/docs/guides/blocks`}
          title={t.setup}
          className="inline-flex min-h-9 min-w-8 items-center justify-center gap-1.5 rounded-md text-sm font-medium hover:bg-fd-secondary sm:px-2"
        >
          <span className="sr-only sm:not-sr-only">{t.setup}</span>
          <BookOpen aria-hidden className="size-4" />
        </Link>
      </div>
      <div className="space-y-12 sm:space-y-16">
        {entries.map(({ block, title, description }) => (
          <article
            key={block.name}
            id={block.name}
            aria-labelledby={`${block.name}-title`}
            className="min-w-0 scroll-mt-20"
          >
            <h2
              id={`${block.name}-title`}
              className="text-xl font-semibold tracking-tight sm:text-2xl"
            >
              <Link
                href={`/${lang}/blocks/${block.name}`}
                className="inline-flex items-center gap-1 rounded hover:underline"
              >
                {title}
                <ArrowUpRight aria-hidden className="size-4 text-fd-muted-foreground" />
              </Link>
            </h2>
            <p className="mt-1 mb-4 text-sm leading-relaxed text-fd-muted-foreground">
              {description}
            </p>
            <BlockViewer lang={lang} title={title} block={block} command={block.command} />
            <div className="mt-2 flex flex-wrap items-center gap-x-4">
              {block.components.map(({ label, path }) => (
                <Link
                  key={path}
                  href={`/${lang}/docs/${path}`}
                  className="inline-flex min-h-8 items-center gap-0.5 rounded text-xs font-medium text-fd-muted-foreground hover:text-fd-foreground hover:underline"
                >
                  {label}
                  <ArrowUpRight aria-hidden className="size-3" />
                </Link>
              ))}
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
