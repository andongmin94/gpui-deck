"use client";

import { Link, useRouter } from "fumapress/client";
import { ArrowUpRight } from "lucide-react";
import { BlockViewer } from "../blocks/block-viewer";
import { chartCopy, chartTypes, type ChartExample } from "./charts";

export function ChartsBrowser({ lang, examples }: { lang: string; examples: ChartExample[] }) {
  const { query } = useRouter();
  const requested = new URLSearchParams(query).get("type");
  const selected = chartTypes.find((type) => type === requested) ?? "all";
  const visible = examples.filter((entry) => selected === "all" || entry.type === selected);
  const t = chartCopy(lang);
  return (
    <>
      <nav aria-label={t.filter} className="mb-6 flex flex-wrap gap-1 border-b pb-3">
        {(["all", ...chartTypes] as const).map((type) => (
          <Link
            key={type}
            href={`/${lang}/charts${type === "all" ? "" : `?type=${type}`}`}
            aria-current={selected === type ? "page" : undefined}
            className="inline-flex min-h-9 items-center rounded-lg px-3 text-sm font-medium text-fd-muted-foreground hover:bg-fd-secondary aria-[current]:bg-fd-secondary aria-[current]:text-fd-foreground"
          >
            {t.types[type]}
          </Link>
        ))}
      </nav>
      <div className="grid items-start gap-x-6 gap-y-8 xl:grid-cols-2">
        {visible.map(({ item, type, title, registry, href }) => (
          <article
            key={item.name}
            id={item.name}
            aria-labelledby={`${item.name}-title`}
            className="min-w-0 scroll-mt-20"
          >
            <header className="mb-3">
              <p className="mb-1 text-sm text-fd-muted-foreground">
                {registry} · {t.types[type]}
              </p>
              <h2 id={`${item.name}-title`} className="text-xl font-semibold tracking-tight">
                <Link
                  href={href}
                  className="inline-flex items-center gap-1 rounded hover:underline"
                >
                  {title}
                  <ArrowUpRight aria-hidden className="size-4 text-fd-muted-foreground" />
                </Link>
              </h2>
            </header>
            <BlockViewer lang={lang} title={title} block={item} command={item.command} />
          </article>
        ))}
      </div>
    </>
  );
}
