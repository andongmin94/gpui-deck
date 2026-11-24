import { Link } from "fumapress/client";
import { ArrowLeft, ArrowRight, ArrowUpRight } from "lucide-react";
import { RegistryBrowser } from "./registry-browser";
import { CatalogLayout } from "../catalog-layout";
import { itemTitle, type Registry, registryCopy } from "./registries";

type PageInfo = { title: string; description?: string };

const heading = "text-4xl font-semibold tracking-[-0.05em] sm:text-5xl";
const description =
  "mt-3 max-w-2xl text-base leading-relaxed text-pretty text-fd-muted-foreground sm:text-lg";
const quietLink =
  "inline-flex min-h-8 items-center gap-1.5 rounded text-sm text-fd-muted-foreground hover:text-fd-foreground hover:underline underline-offset-4";

export function RegistryDirectory({
  lang,
  page,
  entries,
}: {
  lang: string;
  page: PageInfo;
  entries: { registry: Registry; page: PageInfo }[];
}) {
  const t = registryCopy(lang);
  return (
    <CatalogLayout lang={lang}>
      <header className="py-10 sm:py-16">
        <h1 className={heading}>{page.title}</h1>
        <p className={description}>{page.description}</p>
      </header>
      <div className="grid gap-5 md:grid-cols-2">
        {entries.map(({ registry, page: entry }) => {
          const href = `/${lang}/directory/${registry.slug}`;
          const cover = registry.items.find((item) => item.slug === registry.featured[0])!;
          return (
            <article
              key={registry.slug}
              className="flex min-w-0 flex-col overflow-hidden rounded-2xl border"
            >
              <Link
                href={href}
                aria-label={registry.name}
                className="block overflow-hidden border-b bg-fd-secondary focus-visible:outline-offset-[-4px]"
              >
                <picture>
                  <source
                    media="(max-width: 640px)"
                    srcSet={`${cover.poster}-mobile.webp`}
                    width={390}
                    height={cover.mobileHeight}
                  />
                  <img
                    src={`${cover.poster}.webp`}
                    alt=""
                    width={cover.posterWidth}
                    height={cover.posterHeight}
                    className="block h-64 w-full object-contain"
                  />
                </picture>
              </Link>
              <div className="flex flex-1 flex-col items-start gap-4 p-6 sm:p-8">
                <span className="text-sm text-fd-muted-foreground">
                  @{registry.author} · {registry.items.length} {t.count}
                </span>
                <h2 className="text-3xl font-semibold tracking-tight">{registry.name}</h2>
                <p className="max-w-md leading-relaxed text-fd-muted-foreground">
                  {entry.description}
                </p>
                <ul className="flex flex-wrap gap-1.5">
                  {registry.featured.map((slug) => (
                    <li key={slug}>
                      <Link
                        href={`${href}#${slug}`}
                        className="inline-flex rounded-full border px-3 py-1 text-xs font-medium hover:bg-fd-secondary"
                      >
                        {itemTitle(slug, lang)}
                      </Link>
                    </li>
                  ))}
                </ul>
                <Link
                  href={href}
                  className="mt-2 inline-flex min-h-10 items-center gap-2 rounded-lg bg-fd-primary px-4 text-sm font-medium text-fd-primary-foreground hover:opacity-85"
                >
                  {t.open}
                  <ArrowRight aria-hidden className="size-4" />
                </Link>
              </div>
            </article>
          );
        })}
      </div>
    </CatalogLayout>
  );
}

export function RegistryDetail({
  lang,
  page,
  registry,
}: {
  lang: string;
  page: PageInfo;
  registry: Registry;
}) {
  const t = registryCopy(lang);
  return (
    <CatalogLayout lang={lang}>
      <header className="border-b py-7 sm:py-11">
        <Link href={`/${lang}/directory`} className={quietLink}>
          <ArrowLeft aria-hidden className="size-4" />
          {t.directory}
        </Link>
        <h1 className={`mt-4 ${heading}`}>{page.title}</h1>
        <p className={description}>{page.description}</p>
        <div className="mt-3 flex flex-wrap items-center gap-x-5">
          <a className={quietLink} href={`https://github.com/${registry.author}`}>
            @{registry.author}
          </a>
          <a className={quietLink} href={registry.homepage}>
            {t.original}
            <ArrowUpRight aria-hidden className="size-3.5" />
          </a>
          <Link className={quietLink} href={`/${lang}/docs/guides/blocks#registry`}>
            {t.guide}
            <ArrowRight aria-hidden className="size-3.5" />
          </Link>
        </div>
      </header>
      <RegistryBrowser key={registry.slug} lang={lang} collection={registry.slug} />
    </CatalogLayout>
  );
}
