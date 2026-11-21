"use client";

import { Code2, Play, RotateCcw } from "lucide-react";
import { type ReactNode, useState } from "react";

import { GIT } from "@/lib/site";
import { useAccent } from "./use-accent";

const sourceUrl = `https://github.com/${GIT.user}/${GIT.repo}/blob/${GIT.branch}/crates/web-preview/src/demos/invoice_workspace.rs`;

type Labels = {
  title: string;
  description: string;
  run: string;
  stop: string;
  source: string;
  unavailable: string;
};

export function InvoiceShowcase({ labels, children }: { labels: Labels; children: ReactNode }) {
  const [mode, setMode] = useState<"preview" | "live" | "unavailable">("preview");
  const accent = useAccent();
  const live = mode === "live";

  function toggleDemo() {
    if (live) {
      setMode("preview");
    } else {
      setMode("gpu" in navigator ? "live" : "unavailable");
    }
  }

  return (
    <section
      aria-labelledby="invoice-title"
      className="min-w-0 overflow-hidden rounded-2xl border bg-fd-background shadow-xs sm:rounded-3xl lg:col-start-2 lg:row-start-1"
    >
      <div className="flex flex-wrap items-center justify-between gap-x-5 gap-y-3 border-b px-5 py-4 sm:px-6">
        <div className="min-w-0 flex-[1_1_18rem]">
          <h2 id="invoice-title" className="text-lg leading-6 font-semibold tracking-tight">
            {labels.title}
          </h2>
          <p className="mt-1 text-sm leading-relaxed text-fd-muted-foreground [html:lang(ko)_&]:break-keep">
            {labels.description}
          </p>
        </div>
        <div className="flex flex-wrap gap-1.5">
          <button
            type="button"
            aria-controls="invoice-preview"
            onClick={toggleDemo}
            className="inline-flex min-h-8 items-center gap-1.5 rounded-full bg-fd-primary px-3 text-sm font-medium text-fd-primary-foreground hover:opacity-85"
          >
            {live ? (
              <RotateCcw className="size-3.5" aria-hidden />
            ) : (
              <Play className="size-3.5" aria-hidden />
            )}
            {live ? labels.stop : labels.run}
          </button>
          <a
            href={sourceUrl}
            className="inline-flex min-h-8 items-center gap-1.5 rounded-full bg-fd-secondary px-3 text-sm font-medium hover:bg-fd-accent"
          >
            <Code2 className="size-3.5" aria-hidden />
            {labels.source}
          </a>
        </div>
      </div>
      {mode === "unavailable" && (
        <output className="block border-b px-5 py-3 text-sm text-fd-muted-foreground">
          {labels.unavailable}
        </output>
      )}
      <div id="invoice-preview">
        {live ? (
          <iframe
            title={labels.title}
            data-accent={accent}
            src="/preview?c=invoice-workspace"
            className="block h-[min(78svh,700px)] min-h-96 w-full"
          />
        ) : (
          children
        )}
      </div>
    </section>
  );
}
