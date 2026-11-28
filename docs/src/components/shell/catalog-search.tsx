"use client";

import { useEffect, useMemo, useState } from "react";
import { useStaticSearch } from "fumadocs-core/search/client/orama-static";
import {
  SearchDialog,
  SearchDialogClose,
  SearchDialogContent,
  SearchDialogHeader,
  SearchDialogIcon,
  SearchDialogInput,
  SearchDialogList,
  SearchDialogOverlay,
  type SharedProps,
} from "fumadocs-ui/components/dialog/search";
import type { CatalogEntry } from "@/lib/search-catalog";
import { matchRank, normalizeSearch } from "@/lib/catalog-match";

export function CatalogSearchDialog({
  from,
  locale = "en",
  ...props
}: SharedProps & { from?: string; locale?: string }) {
  const docs = useStaticSearch({ from, locale });
  const [catalog, setCatalog] = useState<{ locale: string; entries: CatalogEntry[] }>();

  useEffect(() => {
    if (!props.open || catalog?.locale === locale) return;
    const request = new AbortController();
    async function load() {
      const response = await fetch(`/api/catalog/${locale}.json`, { signal: request.signal });
      if (response.ok) {
        const entries: CatalogEntry[] = await response.json();
        if (!request.signal.aborted) setCatalog({ locale, entries });
      }
    }
    // Documentation search remains available if the catalog request is interrupted.
    void load().catch(() => {});
    return () => request.abort();
  }, [props.open, catalog?.locale, locale]);

  const items = useMemo(() => {
    const query = normalizeSearch(docs.search);
    if (!query) return null;
    const ranked = (catalog?.locale === locale ? catalog.entries : [])
      .map((entry) => ({
        entry,
        rank: matchRank(
          query,
          [entry.title, ...entry.aliases],
          [...entry.breadcrumbs, ...(entry.features ?? [])],
        ),
      }))
      .filter(({ rank }) => rank >= 0)
      .sort((left, right) => left.rank - right.rank);
    const hits = ranked.map(({ entry }) => ({
      type: "page" as const,
      id: entry.url,
      url: entry.url,
      content: entry.title,
      breadcrumbs: entry.breadcrumbs,
    }));
    const urls = new Set(hits.map((hit) => hit.url));
    const articles = docs.data?.query === docs.search ? docs.data.items : undefined;
    if (!hits.length && !articles) return null;
    return [
      ...hits,
      ...(articles ?? []).filter((row) => row.type === "table" || !urls.has(row.url)),
    ];
  }, [catalog, docs.data, docs.search, locale]);

  return (
    <SearchDialog {...props} {...docs}>
      <SearchDialogOverlay />
      <SearchDialogContent>
        <SearchDialogHeader>
          <SearchDialogIcon />
          <SearchDialogInput />
          <SearchDialogClose />
        </SearchDialogHeader>
        <SearchDialogList items={items} />
      </SearchDialogContent>
    </SearchDialog>
  );
}
