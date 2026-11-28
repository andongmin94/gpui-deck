"use client";

import { Link } from "fumapress/client";
import { ArrowUpRight, BookOpen, ChevronDown, Download } from "lucide-react";
import { useEffect, useState } from "react";
import { matchRank, normalizeSearch } from "@/lib/catalog-match";
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

type Filters = { purpose: string; style: string; q: string };
const all: Filters = { purpose: "", style: "", q: "" };
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
  const [wide, setWide] = useState(false);
  const [expanded, setExpanded] = useState("");

  useEffect(() => {
    const desktop = window.matchMedia("(min-width: 40rem)");
    const update = () => setWide(desktop.matches);
    update();
    desktop.addEventListener("change", update);
    return () => desktop.removeEventListener("change", update);
  }, []);

  useEffect(() => {
    function sync() {
      const params = new URLSearchParams(window.location.search);
      const purpose = purposes.find((id) => id === params.get("purpose")) ?? "";
      const style = params.get("style") ?? "";
      setFilters({
        purpose,
        style: Object.hasOwn(styles, style) ? style : "",
        q: params.get("q") ?? "",
      });
      setExpanded("");
    }
    sync();
    window.addEventListener("popstate", sync);
    return () => window.removeEventListener("popstate", sync);
  }, [styles]);
  const selected = filters ?? all;
  const query = normalizeSearch(selected.q);
  const visible = screens.filter(
    ({ style, purpose, title, block, description, components }) =>
      (!selected.style || style === selected.style) &&
      (!selected.purpose || purpose === selected.purpose) &&
      (!query ||
        matchRank(
          query,
          [title, block.name],
          [
            description ?? "",
            t.purposes[purpose],
            styles[style],
            optionLabel({ value: style, label: styles[style] }, lang),
            ...(components ?? []).map(({ label }) => label),
          ],
        ) >= 0),
  );

  function apply(next: Filters, replace = false) {
    const url = new URL(window.location.href);
    for (const [key, value] of Object.entries(next)) {
      if (value) url.searchParams.set(key, value);
      else url.searchParams.delete(key);
    }
    url.hash = "";
    if (url.href !== window.location.href) {
      if (replace) window.history.replaceState(window.history.state, "", url);
      else window.history.pushState(window.history.state, "", url);
    }
    setFilters(next);
    setExpanded("");
  }

  return (
    <>
      <div className="mb-5 flex flex-wrap items-end justify-between gap-x-4 gap-y-1 border-b pb-3 sm:mb-7">
        <fieldset
          disabled={!filters}
          className="grid w-full min-w-0 grid-cols-2 gap-2 sm:flex sm:w-auto sm:flex-wrap sm:gap-3"
        >
          <legend className="sr-only">{t.all}</legend>
          <label className="col-span-2 grid min-w-0 gap-1 text-xs text-fd-muted-foreground sm:col-auto">
            <span className="sr-only sm:not-sr-only">{t.search}</span>
            <input
              type="search"
              placeholder={t.searchHint}
              className={selectClass}
              value={selected.q}
              onChange={(e) =>
                apply({ ...selected, q: e.target.value }, !!selected.q && !!e.target.value)
              }
            />
          </label>
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
              className={`grid items-start gap-x-6 gap-y-4 sm:gap-y-8 ${group.length > 1 ? "xl:grid-cols-2" : ""}`}
            >
              {group.map(({ block, title, description, href, style, components }) => (
                <article
                  key={block.name}
                  id={block.name}
                  aria-labelledby={`${block.name}-title`}
                  className="min-w-0 scroll-mt-20 has-[iframe]:col-span-full"
                >
                  <div className="flex items-start gap-3 rounded-lg border p-2.5 sm:mb-2 sm:block sm:rounded-none sm:border-0 sm:p-0">
                    <button
                      type="button"
                      aria-expanded={expanded === block.name}
                      aria-controls={`${block.name}-viewer`}
                      onClick={() => setExpanded(expanded === block.name ? "" : block.name)}
                      className="grid w-24 shrink-0 gap-1.5 rounded text-xs font-medium sm:hidden"
                    >
                      <img
                        width={block.posterWidth}
                        height={block.posterHeight}
                        src={`${block.poster}.webp`}
                        alt=""
                        loading="lazy"
                        className="aspect-[4/3] w-full rounded border bg-fd-secondary object-contain"
                      />
                      <span className="inline-flex items-center justify-center gap-1">
                        {expanded === block.name ? t.closePreview : t.preview}
                        <ChevronDown
                          aria-hidden
                          className={`size-3 ${expanded === block.name ? "rotate-180" : ""}`}
                        />
                      </span>
                    </button>
                    <div className="min-w-0 flex-1">
                      <div className="flex flex-wrap items-center gap-x-2 gap-y-1">
                        <h3 id={`${block.name}-title`} className="font-medium">
                          <Link
                            href={href}
                            aria-label={
                              style === "base" ? undefined : `${title} · ${styles[style]}`
                            }
                            className="inline-flex items-center gap-1 rounded hover:underline"
                          >
                            {title}
                            <ArrowUpRight
                              aria-hidden
                              className="size-3.5 text-fd-muted-foreground"
                            />
                          </Link>
                        </h3>
                        <span className="rounded-md bg-fd-secondary px-2 py-0.5 text-xs font-medium">
                          {styles[style]}
                        </span>
                      </div>
                      {description && (
                        <p className="mt-1 text-sm leading-snug text-fd-muted-foreground">
                          {description}
                        </p>
                      )}
                      <div className="mt-1 flex flex-wrap items-center gap-x-3">
                        {style === "base" ? (
                          <Link
                            href={`${href}#new-app`}
                            className="inline-flex min-h-8 items-center text-xs underline underline-offset-4"
                          >
                            {t.newApp}
                          </Link>
                        ) : (
                          <a
                            href={`/downloads/${block.name}.zip`}
                            download
                            className="inline-flex min-h-8 items-center gap-1 text-xs underline underline-offset-4"
                          >
                            <Download aria-hidden className="size-3.5" />
                            {t.download}
                          </a>
                        )}
                        {components?.map(({ label, path }) => (
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
                    </div>
                  </div>
                  <div id={`${block.name}-viewer`} className="mt-2 empty:hidden sm:mt-0">
                    {(wide || expanded === block.name) && (
                      <BlockViewer
                        lang={lang}
                        title={title}
                        block={block}
                        command={block.command}
                      />
                    )}
                  </div>
                </article>
              ))}
            </div>
          </section>
        );
      })}
    </>
  );
}
