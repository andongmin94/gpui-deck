"use client";

import { Code2, Play, RotateCcw } from "lucide-react";
import { type ReactNode, useState } from "react";

import { GIT } from "@/lib/site";

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
      className="mx-auto w-full max-w-[1248px] px-4 pb-12 sm:px-8 sm:pb-16"
    >
      <div className="mb-4 flex flex-wrap items-end justify-between gap-x-6 gap-y-3">
        <div>
          <h2 id="invoice-title" className="text-xl font-semibold tracking-tight">
            {labels.title}
          </h2>
          <p className="mt-1 text-sm leading-relaxed text-fd-muted-foreground [html:lang(ko)_&]:break-keep">
            {labels.description}
          </p>
        </div>
        <div className="flex flex-wrap gap-2">
          <button
            type="button"
            aria-controls="invoice-preview"
            onClick={toggleDemo}
            className="inline-flex h-10 items-center gap-2 rounded-lg bg-fd-primary px-3.5 text-sm font-medium text-fd-primary-foreground hover:opacity-90"
          >
            {live ? (
              <RotateCcw className="size-4" aria-hidden />
            ) : (
              <Play className="size-4" aria-hidden />
            )}
            {live ? labels.stop : labels.run}
          </button>
          <a
            href={sourceUrl}
            className="inline-flex h-10 items-center gap-2 rounded-lg border px-3.5 text-sm font-medium hover:bg-fd-accent"
          >
            <Code2 className="size-4" aria-hidden />
            {labels.source}
          </a>
        </div>
      </div>
      {mode === "unavailable" && (
        <output className="mb-3 block text-sm text-fd-muted-foreground">
          {labels.unavailable}
        </output>
      )}
      <div id="invoice-preview" className="overflow-hidden rounded-xl border bg-fd-card shadow-sm">
        {live ? (
          <iframe
            title={labels.title}
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
