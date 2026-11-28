"use client";

import { Box, Search } from "lucide-react";
import { useRouter } from "fumapress/client";
import { useEffect, useRef, useState } from "react";
import { BlockViewer } from "../blocks/block-viewer";
import { InstallCommand } from "../blocks/install-command";
import { itemTitle, kinds, type Registry, registries, registryCopy } from "./registries";

function fromHash(registry: Registry) {
  const slug = window.location.hash.slice(1);
  return registry.items.some((item) => item.slug === slug) ? slug : registry.featured[0];
}

export function RegistryBrowser({ lang, collection }: { lang: string; collection: string }) {
  const { hash } = useRouter();
  const registry = registries.find((entry) => entry.slug === collection)!;
  const t = registryCopy(lang);
  // The static page cannot see the URL fragment. Resolve it before starting GPUI.
  const [selected, setSelected] = useState<string | null>(null);
  const [query, setQuery] = useState("");
  const detail = useRef<HTMLElement>(null);
  useEffect(() => {
    const sync = () => setSelected(fromHash(registry));
    sync();
    window.addEventListener("hashchange", sync);
    return () => window.removeEventListener("hashchange", sync);
  }, [registry, hash]);
  useEffect(() => {
    if (selected && window.location.hash === `#${selected}`)
      detail.current?.scrollIntoView({ block: "start" });
  }, [selected]);
  const current = registry.items.find((item) => item.slug === (selected ?? registry.featured[0]))!;
  const needle = query.trim().toLocaleLowerCase();
  const matches = registry.items.filter((item) =>
    [item.slug, itemTitle(item.slug, lang), t.kinds[item.kind]].some((s) =>
      s.toLocaleLowerCase().includes(needle),
    ),
  );
  const title = itemTitle(current.slug, lang);

  return (
    <div className="grid items-start gap-6 pt-6 lg:grid-cols-[15rem_minmax(0,1fr)] lg:gap-10">
      <nav aria-label={t.items} className="min-w-0 lg:sticky lg:top-20">
        <label className="flex items-center gap-2 rounded-lg border px-3 focus-within:ring-2 focus-within:ring-fd-ring">
          <Search aria-hidden className="size-4 shrink-0 text-fd-muted-foreground" />
          <input
            type="search"
            aria-label={t.search}
            placeholder={t.search}
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            className="h-10 min-w-0 w-full bg-transparent text-sm outline-none"
          />
        </label>
        {matches.length === 0 && (
          <output className="block py-4 text-sm text-fd-muted-foreground">{t.empty}</output>
        )}
        {matches.length > 0 && (
          <>
            <select
              aria-label={t.choose}
              value={matches.some((item) => item.slug === current.slug) ? current.slug : ""}
              className="mt-2 h-10 w-full rounded-lg border bg-fd-background px-2 text-sm lg:hidden"
              onChange={(event) => {
                window.location.hash = event.target.value;
              }}
            >
              <option value="" disabled>
                {t.choose}
              </option>
              {kinds.map((kind) => {
                const items = matches.filter((item) => item.kind === kind);
                return (
                  items.length > 0 && (
                    <optgroup key={kind} label={t.kinds[kind]}>
                      {items.map(({ slug }) => (
                        <option key={slug} value={slug}>
                          {itemTitle(slug, lang)}
                        </option>
                      ))}
                    </optgroup>
                  )
                );
              })}
            </select>
            <div className="mt-3 hidden max-h-[calc(100dvh-10rem)] overflow-y-auto overscroll-contain pe-2 pb-3 lg:block">
              {kinds.map((kind) => {
                const items = matches.filter((item) => item.kind === kind);
                return (
                  items.length > 0 && (
                    <div key={kind} className="mb-4">
                      <p className="mb-1 px-2 text-xs font-medium text-fd-muted-foreground">
                        {t.kinds[kind]}
                      </p>
                      <ul>
                        {items.map(({ slug }) => (
                          <li key={slug}>
                            <a
                              href={`#${slug}`}
                              aria-current={current.slug === slug ? "true" : undefined}
                              className="block rounded px-2 py-1.5 text-sm hover:bg-fd-secondary aria-[current]:bg-fd-secondary aria-[current]:font-semibold"
                            >
                              {itemTitle(slug, lang)}
                            </a>
                          </li>
                        ))}
                      </ul>
                    </div>
                  )
                );
              })}
            </div>
          </>
        )}
      </nav>
      <section
        ref={detail}
        key={current.slug}
        id={current.slug}
        aria-labelledby="registry-item-title"
        className="min-w-0 scroll-mt-20"
      >
        <div className="mb-4 flex flex-wrap items-baseline gap-3">
          <h2
            id="registry-item-title"
            className="text-2xl font-semibold tracking-tight sm:text-3xl"
          >
            {title}
          </h2>
          <span className="inline-flex items-center gap-1 text-xs text-fd-muted-foreground">
            <Box aria-hidden className="size-3.5" />
            {t.kinds[current.kind]}
          </span>
        </div>
        <BlockViewer
          lang={lang}
          title={title}
          block={current}
          automaticPreview={selected === null ? "pending" : navigator.gpu ? "run" : "unavailable"}
        />
        <div className="mt-4">
          <InstallCommand lang={lang} command={current.command} />
        </div>
      </section>
    </div>
  );
}
