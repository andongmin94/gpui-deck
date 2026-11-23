import { Link } from "fumapress/client";
import { ArrowLeft, ArrowRight, ArrowUpRight } from "lucide-react";
import { RegistryBrowser } from "./registry-browser";
import { CatalogLayout } from "../catalog-layout";
import { itemTitle, neobrutal, registryCopy } from "./neobrutal";

type PageInfo = { title: string; description?: string };

const heading = "text-4xl font-semibold tracking-[-0.05em] sm:text-5xl";
const description =
  "mt-3 max-w-2xl text-base leading-relaxed text-pretty text-fd-muted-foreground sm:text-lg";
const quietLink =
  "inline-flex min-h-8 items-center gap-1.5 rounded text-sm text-fd-muted-foreground hover:text-fd-foreground hover:underline underline-offset-4";

export function RegistryDirectory({
  lang,
  page,
  registry,
}: {
  lang: string;
  page: PageInfo;
  registry: PageInfo;
}) {
  const t = registryCopy(lang);
  const href = `/${lang}/directory/neobrutal`;
  return (
    <CatalogLayout lang={lang}>
      <header className="py-10 sm:py-16">
        <h1 className={heading}>{page.title}</h1>
        <p className={description}>{page.description}</p>
      </header>
      <article className="grid overflow-hidden rounded-2xl border md:grid-cols-2">
        <Link
          href={href}
          aria-label={neobrutal.name}
          className="block overflow-hidden bg-[#fdf7c4] focus-visible:outline-offset-[-4px]"
        >
          <picture>
            <source
              media="(max-width: 640px)"
              srcSet="/images/registries/neobrutal/login-mobile.webp"
              width={390}
              height={520}
            />
            <img
              src="/images/registries/neobrutal/login.webp"
              alt=""
              width={760}
              height={520}
              className="block max-h-72 w-full object-contain md:h-full md:max-h-none"
            />
          </picture>
        </Link>
        <div className="flex flex-col items-start justify-center gap-4 p-6 sm:p-10">
          <span className="text-sm text-fd-muted-foreground">
            @{neobrutal.author} · {neobrutal.items.length} {t.count}
          </span>
          <h2 className="text-3xl font-semibold tracking-tight">{neobrutal.name}</h2>
          <p className="max-w-md leading-relaxed text-fd-muted-foreground">
            {registry.description}
          </p>
          <ul className="flex flex-wrap gap-1.5">
            {["login", "button", "input", "dashboard-template"].map((slug) => (
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
    </CatalogLayout>
  );
}

export function RegistryDetail({ lang, page }: { lang: string; page: PageInfo }) {
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
          <a className={quietLink} href={`https://github.com/${neobrutal.author}`}>
            @{neobrutal.author}
          </a>
          <a className={quietLink} href={neobrutal.homepage}>
            {t.original}
            <ArrowUpRight aria-hidden className="size-3.5" />
          </a>
          <Link className={quietLink} href={`/${lang}/docs/guides/blocks#registry`}>
            {t.guide}
            <ArrowRight aria-hidden className="size-3.5" />
          </Link>
        </div>
      </header>
      <RegistryBrowser lang={lang} />
    </CatalogLayout>
  );
}
