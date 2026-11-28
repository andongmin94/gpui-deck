import { Link } from "fumapress/client";
import { BookOpen } from "lucide-react";
import { CatalogLayout } from "../catalog-layout";
import { itemTitle, registries } from "../directory/registries";
import { ChartsBrowser } from "./charts-browser";
import { chartCopy, chartTypes, type ChartExample } from "./charts";

export function ChartsGallery({
  lang,
  page,
}: {
  lang: string;
  page: { title: string; description?: string };
}) {
  const t = chartCopy(lang);
  const examples = registries
    .flatMap((registry) =>
      registry.items.flatMap((item): ChartExample[] => {
        if (!item.chartType && item.kind !== "chart") return [];
        const type = chartTypes.find((type) => type === item.chartType);
        if (!type) throw new Error(`Missing chart type for ${item.name}`);
        const title =
          item.slug === "chart"
            ? t.examples[registry.slug as keyof typeof t.examples]
            : itemTitle(item.slug, lang);
        return [
          {
            item,
            type,
            title,
            registry: registry.name,
            href: `/${lang}/directory/${registry.slug}#${item.slug}`,
          },
        ];
      }),
    )
    .sort((a, b) => a.item.slug.localeCompare(b.item.slug));

  return (
    <CatalogLayout lang={lang}>
      <header className="mx-auto flex max-w-3xl flex-col items-center py-8 text-center sm:py-16">
        <h1 className="text-4xl font-semibold tracking-[-0.045em] text-balance sm:text-5xl">
          {page.title}
        </h1>
        <p className="mt-4 max-w-xl text-base leading-relaxed text-balance text-fd-muted-foreground sm:text-lg">
          {page.description}
        </p>
        <Link
          href={`/${lang}/docs/components/chart`}
          className="mt-4 inline-flex min-h-9 items-center gap-2 rounded-lg bg-fd-secondary px-3.5 text-sm font-medium hover:bg-fd-accent"
        >
          <BookOpen aria-hidden className="size-4" />
          {t.docs}
        </Link>
      </header>
      <ChartsBrowser lang={lang} examples={examples} />
    </CatalogLayout>
  );
}
