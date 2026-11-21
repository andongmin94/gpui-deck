"use client";

import { Tabs } from "@base-ui/react/tabs";
import { Code2, Image, Play, Square } from "lucide-react";
import { lazy, Suspense, useRef, useState } from "react";

import { billing, blockCopy } from "./block-copy";
import { BlockPoster } from "./block-poster";

const BlockSource = lazy(() => import("./block-source"));
const tabStyle =
  "inline-flex min-h-9 items-center gap-1.5 rounded-lg px-3 text-sm font-medium text-fd-muted-foreground data-active:bg-fd-background data-active:text-fd-foreground data-active:shadow-xs";

export function BlockViewer({ lang, title }: { lang: string; title: string }) {
  const t = blockCopy(lang);
  const [tab, setTab] = useState("preview");
  const [mode, setMode] = useState<"image" | "checking" | "run" | "unavailable">("image");
  const request = useRef(0);

  async function togglePreview() {
    if (mode === "run") {
      setMode("image");
      return;
    }
    const id = ++request.current;
    setMode("checking");
    try {
      const adapter = await navigator.gpu?.requestAdapter();
      if (request.current === id) setMode(adapter ? "run" : "unavailable");
    } catch {
      if (request.current === id) setMode("unavailable");
    }
  }
  return (
    <Tabs.Root
      value={tab}
      onValueChange={(value) => {
        request.current++;
        setTab(String(value));
        setMode("image");
      }}
      className="overflow-hidden rounded-2xl border shadow-xs"
    >
      <div className="flex flex-wrap items-center justify-between gap-2 border-b p-2 sm:px-4">
        <Tabs.List aria-label={title} className="inline-flex rounded-xl bg-fd-secondary p-1">
          <Tabs.Tab value="preview" className={tabStyle}>
            <Image aria-hidden className="size-4" />
            {t.preview}
          </Tabs.Tab>
          <Tabs.Tab value="source" className={tabStyle}>
            <Code2 aria-hidden className="size-4" />
            {t.source}
          </Tabs.Tab>
        </Tabs.List>
        {tab === "preview" && (
          <button
            type="button"
            aria-controls="block-preview"
            onClick={togglePreview}
            disabled={mode === "checking"}
            aria-busy={mode === "checking"}
            className="inline-flex min-h-9 items-center gap-1.5 rounded-lg border px-3 text-sm font-medium hover:bg-fd-secondary"
          >
            {mode === "run" ? (
              <Square aria-hidden className="size-3.5" />
            ) : (
              <Play aria-hidden className="size-3.5" />
            )}
            {mode === "run" ? t.stop : t.run}
          </button>
        )}
      </div>
      <Tabs.Panel value="preview">
        {mode === "unavailable" && (
          <output className="block border-b px-4 py-3 text-sm text-fd-muted-foreground">
            {t.unavailable}
          </output>
        )}
        <div id="block-preview">
          {mode === "run" ? (
            <iframe
              title={title}
              src={`/preview?c=${billing.name}`}
              className="block h-[min(78svh,740px)] min-h-96 w-full"
            />
          ) : (
            <BlockPoster alt={t.image} />
          )}
        </div>
      </Tabs.Panel>
      <Tabs.Panel value="source">
        <Suspense fallback={<output className="block p-5 text-sm">{t.loading}</output>}>
          <BlockSource lang={lang} />
        </Suspense>
      </Tabs.Panel>
    </Tabs.Root>
  );
}
