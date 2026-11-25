"use client";

import {
  BarChart3,
  CaseSensitive,
  Check,
  ChevronDown,
  Circle,
  Layers,
  Link as LinkIcon,
  Paintbrush,
  RotateCcw,
  Shapes,
  Shuffle,
  SlidersHorizontal,
  Square,
  Type,
} from "lucide-react";
import { useEffect, useRef, useState } from "react";
import {
  axes,
  defaultPreset,
  encodePreset,
  options,
  parsePreset,
  shufflePreset,
  type Preset,
} from "@/lib/create-preset";
import { createCopy, optionLabel } from "./copy";
import { GetCode, OpenPreset } from "./preset-dialogs";

const markers = {
  style: Layers,
  base: Circle,
  accent: Paintbrush,
  chart: BarChart3,
  heading: CaseSensitive,
  font: Type,
  icons: Shapes,
  radius: Square,
};
const sideButton =
  "inline-flex min-h-10 items-center justify-center gap-2 rounded-lg border border-white/20 px-3 text-sm font-medium hover:bg-white/10";

function readLocation() {
  const code = new URLSearchParams(window.location.search).get("preset");
  const preset = code === null ? defaultPreset : parsePreset(code);
  return { preset: preset ?? defaultPreset, invalid: !preset };
}

export function CreatePage({
  lang,
  page,
}: {
  lang: string;
  page: { title: string; description?: string };
}) {
  const t = createCopy(lang);
  const [selection, setSelection] = useState<ReturnType<typeof readLocation> | null>(null);
  const [expanded, setExpanded] = useState(false);
  const [shared, setShared] = useState<"idle" | "copied" | "manual">("idle");
  const shareInput = useRef<HTMLInputElement>(null);
  const preset = selection?.preset ?? defaultPreset;
  const code = encodePreset(preset);

  useEffect(() => {
    function sync() {
      setSelection(readLocation());
      setShared("idle");
    }
    sync();
    window.addEventListener("popstate", sync);
    return () => window.removeEventListener("popstate", sync);
  }, []);

  useEffect(() => {
    if (shared === "manual") {
      shareInput.current?.focus();
      shareInput.current?.select();
    }
  }, [shared]);

  function apply(next: Preset, reset = false) {
    const url = new URL(window.location.href);
    if (reset) url.searchParams.delete("preset");
    else url.searchParams.set("preset", encodePreset(next));
    if (url.href !== window.location.href) window.history.pushState(window.history.state, "", url);
    setSelection({ preset: next, invalid: false });
    setShared("idle");
  }

  function shareUrl() {
    const url = new URL(window.location.href);
    url.searchParams.set("preset", code);
    return url.href;
  }

  return (
    <>
      <header className="flex flex-wrap items-end justify-between gap-4 pt-7 pb-6 sm:pt-10 sm:pb-7">
        <div>
          <h1 className="text-4xl font-semibold tracking-[-0.055em] sm:text-5xl">{page.title}</h1>
          <p className="mt-2 max-w-prose text-sm leading-relaxed text-fd-muted-foreground sm:text-base">
            {page.description}
          </p>
        </div>
        <div className="flex items-center gap-2">
          <button
            type="button"
            disabled={!selection}
            className="inline-flex min-h-10 items-center justify-center gap-2 rounded-lg border px-3 text-sm font-semibold hover:bg-fd-secondary disabled:opacity-60"
            onClick={async () => {
              try {
                await navigator.clipboard.writeText(shareUrl());
                setShared("copied");
              } catch {
                setShared("manual");
              }
            }}
          >
            {shared === "copied" ? (
              <Check aria-hidden className="size-4" />
            ) : (
              <LinkIcon aria-hidden className="size-4" />
            )}
            {shared === "copied" ? t.linkCopied : t.share}
          </button>
          <GetCode preset={preset} lang={lang} t={t} ready={selection !== null} />
        </div>
      </header>
      {shared === "manual" && (
        <label className="mb-4 flex flex-wrap items-center gap-2 text-sm">
          {t.shareLink}
          <input
            ref={shareInput}
            readOnly
            value={shareUrl()}
            onFocus={(e) => e.currentTarget.select()}
            className="h-10 min-w-0 flex-1 rounded-lg border bg-fd-background px-3 font-mono text-xs"
          />
        </label>
      )}
      {selection?.invalid && (
        <p role="alert" className="mb-4 rounded-lg border p-3 text-sm">
          {t.invalidLink}
        </p>
      )}
      <div className="grid items-start gap-4 lg:grid-cols-[16rem_minmax(0,1fr)] xl:grid-cols-[17rem_minmax(0,1fr)]">
        <aside
          aria-label={t.settings}
          className="overflow-hidden rounded-2xl bg-neutral-900 p-3 text-neutral-50 dark:border dark:border-white/15"
        >
          <div className="flex min-h-10 items-center justify-between gap-2 px-1">
            <h2 className="hidden items-center gap-2.5 text-sm font-semibold lg:flex">
              <SlidersHorizontal aria-hidden className="size-4" />
              {t.settings}
            </h2>
            <button
              type="button"
              className="flex min-h-10 flex-1 items-center gap-2 text-left text-sm font-semibold lg:hidden"
              aria-expanded={expanded}
              aria-controls="preset-settings"
              onClick={() => setExpanded(!expanded)}
            >
              <SlidersHorizontal aria-hidden className="size-4" />
              {t.customize}
              <ChevronDown
                aria-hidden
                className={`size-4 text-neutral-400 ${expanded ? "rotate-180" : ""}`}
              />
            </button>
            <button
              type="button"
              onClick={() => apply(defaultPreset, true)}
              disabled={!selection}
              aria-label={t.reset}
              title={t.reset}
              className="inline-flex size-9 items-center justify-center rounded-lg text-neutral-300 hover:bg-white/10 hover:text-white"
            >
              <RotateCcw aria-hidden className="size-4" />
            </button>
          </div>
          <div id="preset-settings" className={expanded ? "block" : "hidden lg:block"}>
            <fieldset disabled={!selection} className="grid min-w-0 gap-1.5 pt-2">
              <legend className="sr-only">{t.settings}</legend>
              {axes.map((axis) => {
                const Icon = markers[axis];
                const choices = options[axis].map((option) => ({
                  value: option.value,
                  label: optionLabel(option, lang),
                }));
                return (
                  <label
                    key={axis}
                    className={`relative flex items-center gap-3 rounded-lg border border-white/15 bg-white/[0.045] px-3 py-2 hover:bg-white/[0.08] focus-within:ring-2 focus-within:ring-white ${axis === "base" || axis === "heading" ? "mt-1.5" : ""}`}
                  >
                    <Icon aria-hidden className="size-4 shrink-0 text-neutral-400" />
                    <span className="min-w-0 flex-1">
                      <span className="block text-xs font-medium text-neutral-400">
                        {t.axes[axis]}
                      </span>
                      <span aria-hidden className="block truncate text-sm font-semibold">
                        {choices.find((option) => option.value === preset[axis])?.label}
                      </span>
                    </span>
                    <ChevronDown aria-hidden className="size-3.5 shrink-0 text-neutral-400" />
                    <select
                      aria-label={t.axes[axis]}
                      value={preset[axis]}
                      onChange={(event) => apply({ ...preset, [axis]: event.target.value })}
                      className="absolute inset-0 h-full w-full cursor-pointer opacity-0 [color-scheme:dark]"
                    >
                      {choices.map((option) => (
                        <option key={option.value} value={option.value}>
                          {option.label}
                        </option>
                      ))}
                    </select>
                  </label>
                );
              })}
            </fieldset>
            <div className="mt-3 flex gap-2 border-t border-white/15 pt-3">
              <OpenPreset t={t} onApply={apply} />
              <button
                type="button"
                onClick={() => apply(shufflePreset())}
                disabled={!selection}
                aria-label={t.shuffle}
                title={t.shuffle}
                className={sideButton}
              >
                <Shuffle aria-hidden className="size-4" />
              </button>
            </div>
          </div>
        </aside>
        <section
          aria-label={t.preview}
          aria-busy={!selection}
          className="relative h-[min(76dvh,52rem)] min-h-96 min-w-0 overflow-hidden rounded-2xl border bg-fd-secondary lg:h-[max(39rem,calc(100dvh-14rem))] lg:max-h-[55rem]"
        >
          {selection &&
            (navigator.gpu ? (
              <iframe
                title={t.preview}
                src={import.meta.env.DEV ? "/preview/index.html?c=create" : "/preview?c=create"}
                data-preset={code}
                className="block size-full"
              />
            ) : (
              <p className="grid h-full place-items-center px-6 text-center text-sm text-fd-muted-foreground">
                {t.unavailable}
              </p>
            ))}
        </section>
      </div>
    </>
  );
}
