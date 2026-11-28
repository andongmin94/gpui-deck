"use client";

import { Tabs } from "@base-ui/react/tabs";
import { Code2, Image, Play, Square } from "lucide-react";
import { lazy, Suspense, useEffect, useId, useRef, useState } from "react";

import { useHydrated } from "../use-hydrated";
import { type PreviewItem, blockCopy } from "./block-copy";
import { BlockPoster } from "./block-poster";
import { InstallCommand } from "./install-command";

const BlockSource = lazy(() => import("./block-source"));
const tabStyle =
  "inline-flex min-h-9 items-center gap-1.5 rounded-lg px-3 text-sm font-medium text-fd-muted-foreground data-active:bg-fd-background data-active:text-fd-foreground data-active:shadow-xs";

export function BlockViewer({
  lang,
  title,
  block,
  automaticPreview,
  command,
}: {
  lang: string;
  title: string;
  block: PreviewItem;
  automaticPreview?: "pending" | "run";
  command?: string;
}) {
  const t = blockCopy(lang);
  const ready = useHydrated();
  const [tab, setTab] = useState("preview");
  const [manualMode, setMode] = useState<"image" | "run" | "unavailable" | null>(null);
  const mode = manualMode ?? automaticPreview ?? "image";
  const frame = useRef<HTMLIFrameElement>(null);
  const previewId = useId();

  useEffect(() => {
    if (mode !== "run") return;
    function handleMessage(event: MessageEvent) {
      if (
        event.origin === window.location.origin &&
        event.source === frame.current?.contentWindow &&
        event.data === "gpui-deck:preview-unavailable"
      ) {
        setMode("unavailable");
      }
    }
    window.addEventListener("message", handleMessage);
    return () => window.removeEventListener("message", handleMessage);
  }, [mode]);

  return (
    <Tabs.Root
      value={tab}
      onValueChange={(value) => setTab(String(value))}
      className="overflow-hidden rounded-xl border bg-fd-background"
    >
      <fieldset
        disabled={!ready}
        aria-label={title}
        className="flex min-w-0 flex-wrap items-center gap-2 border-b p-2 sm:px-3"
      >
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
            aria-controls={previewId}
            title={mode === "run" ? t.stop : t.run}
            onClick={() => setMode(mode === "run" ? "image" : "run")}
            disabled={mode === "pending"}
            className={`inline-flex min-h-9 min-w-9 items-center justify-center gap-1.5 rounded-lg border px-2 text-sm font-medium hover:bg-fd-secondary sm:px-3 ${command ? "" : "ms-auto"}`}
          >
            {mode === "run" ? (
              <Square aria-hidden className="size-3.5" />
            ) : (
              <Play aria-hidden className="size-3.5" />
            )}
            <span className="sr-only sm:not-sr-only">{mode === "run" ? t.stop : t.run}</span>
          </button>
        )}
        {command && (
          <div className="w-full min-w-0 sm:ms-auto sm:max-w-[32rem] sm:flex-1">
            <InstallCommand lang={lang} command={command} />
          </div>
        )}
      </fieldset>
      <Tabs.Panel value="preview" keepMounted>
        {mode === "unavailable" && (
          <output className="block border-b px-4 py-3 text-sm text-fd-muted-foreground">
            {t.unavailable}
          </output>
        )}
        <div
          id={previewId}
          aria-busy={mode === "pending"}
          className={
            mode === "pending" || mode === "run" ? "h-[min(78svh,740px)] min-h-96" : undefined
          }
        >
          {mode === "run" ? (
            <iframe
              ref={frame}
              title={title}
              src={`${import.meta.env.DEV ? "/preview/index.html" : "/preview"}?c=${block.name}`}
              className="block size-full"
            />
          ) : (
            mode !== "pending" && <BlockPoster block={block} alt={title} />
          )}
        </div>
      </Tabs.Panel>
      <Tabs.Panel value="source">
        <Suspense fallback={<output className="block p-5 text-sm">{t.loading}</output>}>
          <BlockSource lang={lang} block={block} />
        </Suspense>
      </Tabs.Panel>
    </Tabs.Root>
  );
}
