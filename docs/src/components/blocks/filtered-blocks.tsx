"use client";

import { Link } from "fumapress/client";
import { ArrowUpRight, BookOpen } from "lucide-react";
import { useEffect, useState } from "react";
import { optionLabel } from "../create/copy";
import { blockCopy, type DocLink, type PreviewItem, type Purpose, purposes } from "./block-copy";
import { BlockViewer } from "./block-viewer";

export type Screen = {
  block: PreviewItem & { command: string };
  title: string;
  description?: string;
  purpose: Purpose;
  style: string;
  href: string;
  components?: DocLink[];
};

type Filters = { purpose: string; style: string };
const all: Filters = { purpose: "", style: "" };
const selectClass =
  "h-10 w-full min-w-0 rounded-lg border bg-fd-background px-2 text-sm font-medium sm:min-w-32";

export function FilteredBlocks({
  lang,
  screens,
  styles,
}: {
  lang: string;
  screens: Screen[];
  styles: Record<string, string>;
}) {
  const t = blockCopy(lang);
  const [filters, setFilters] = useState<Filters | null>(null);
  useEffect(() => {
    function sync() {
      const params = new URLSearchParams(window.location.search);
      const purpose = purposes.find((id) => id === params.get("purpose")) ?? "";
      const style = params.get("style") ?? "";
      setFilters({ purpose, style: Object.hasOwn(styles, style) ? style : "" });
    }
    sync();
    window.addEventListener("popstate", sync);
    return () => window.removeEventListener("popstate", sync);
  }, [styles]);
  const selected = filters ?? all;
  const visible = screens.filter(
    (screen) =>
      (!selected.style || screen.style === selected.style) &&
      (!selected.purpose || screen.purpose === selected.purpose),
  );

  function apply(next: Filters) {
    const url = new URL(window.location.href);
    for (const [key, value] of Object.entries(next)) {
      if (value) url.searchParams.set(key, value);
      else url.searchParams.delete(key);
    }
    url.hash = "";
    if (url.href !== window.location.href) window.history.pushState(window.history.state, "", url);
    setFilters(next);
  }

  return (
    <>
      <div className="mb-7 flex flex-wrap items-end justify-between gap-x-4 gap-y-2 border-b pb-4">
        <fieldset
          disabled={!filters}
          className="grid min-w-[min(100%,15rem)] flex-1 grid-cols-2 gap-2 sm:flex sm:min-w-0 sm:flex-initial sm:gap-3"
        >
          <legend className="sr-only">{t.all}</legend>
          <label className="grid min-w-0 gap-1 text-xs text-fd-muted-foreground">
            {t.purpose}
            <select
              className={selectClass}
              value={selected.purpose}
              onChange={(e) => apply({ ...selected, purpose: e.target.value })}
            >
              <option value="">{t.everyPurpose}</option>
              {purposes.map((id) => (
                <option key={id} value={id}>
                  {t.purposes[id]}
                </option>
              ))}
            </select>
          </label>
          <label className="grid min-w-0 gap-1 text-xs text-fd-muted-foreground">
            {t.style}
            <select
              className={selectClass}
              value={selected.style}
              onChange={(e) => apply({ ...selected, style: e.target.value })}
            >
              <option value="">{t.everyStyle}</option>
              {Object.entries(styles).map(([id, label]) => (
                <option key={id} value={id}>
                  {optionLabel({ value: id, label: label.replace(/ UI$/, "") }, lang)}
                </option>
              ))}
            </select>
          </label>
        </fieldset>
        <div className="flex items-center gap-2.5">
          <output className="text-xs tabular-nums text-fd-muted-foreground">
            {visible.length} / {screens.length}
          </output>
          <Link
            href={`/${lang}/docs/guides/blocks`}
            title={t.setup}
            className="inline-flex min-h-10 min-w-8 items-center justify-center gap-1.5 rounded-md text-sm font-medium hover:bg-fd-secondary sm:px-2"
          >
            <span className="sr-only sm:not-sr-only">{t.setup}</span>
            <BookOpen aria-hidden className="size-4" />
          </Link>
        </div>
      </div>
      {visible.length === 0 && (
        <div className="grid justify-items-start gap-3 rounded-xl border p-6">
          <p>{t.empty}</p>
          <button
            type="button"
            onClick={() => apply(all)}
            className="min-h-9 rounded-md bg-fd-secondary px-3 text-sm font-medium hover:bg-fd-accent"
          >
            {t.clear}
          </button>
        </div>
      )}
      {purposes.map((purpose) => {
        const group = visible.filter((entry) => entry.purpose === purpose);
        if (!group.length) return null;
        return (
          <section key={purpose} aria-labelledby={`blocks-${purpose}`} className="mb-12 last:mb-0">
            <h2
              id={`blocks-${purpose}`}
              className="mb-4 text-xl font-semibold tracking-tight sm:text-2xl"
            >
              {t.purposes[purpose]}
            </h2>
            <div
              className={`grid items-start gap-x-6 gap-y-8 ${group.length > 1 ? "xl:grid-cols-2" : ""}`}
            >
              {group.map(({ block, title, description, href, style, components }) => (
                <article
                  key={block.name}
                  id={block.name}
                  aria-labelledby={`${block.name}-title`}
                  className="min-w-0 scroll-mt-20 has-[iframe]:col-span-full"
                >
                  <div className="mb-2 flex items-center justify-between gap-3">
                    <h3 id={`${block.name}-title`} className="font-medium">
                      <Link
                        href={href}
                        aria-label={style === "base" ? undefined : `${title} · ${styles[style]}`}
                        className="inline-flex items-center gap-1 rounded hover:underline"
                      >
                        {title}
                        <ArrowUpRight aria-hidden className="size-3.5 text-fd-muted-foreground" />
                      </Link>
                    </h3>
                    <div className="flex shrink-0 items-center gap-2">
                      {block.name === "billing-dashboard" && (
                        <Link
                          href={`${href}#new-app`}
                          className="inline-flex min-h-8 items-center text-xs underline underline-offset-4"
                        >
                          {t.newApp}
                        </Link>
                      )}
                      <span className="rounded-md bg-fd-secondary px-2 py-0.5 text-xs font-medium">
                        {styles[style]}
                      </span>
                    </div>
                  </div>
                  <BlockViewer lang={lang} title={title} block={block} command={block.command} />
                  {description && (
                    <p className="mt-2 text-sm text-fd-muted-foreground">{description}</p>
                  )}
                  {components && (
                    <div className="mt-1 flex flex-wrap items-center gap-x-4">
                      {components.map(({ label, path }) => (
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
                  )}
                </article>
              ))}
            </div>
          </section>
        );
      })}
    </>
  );
}
