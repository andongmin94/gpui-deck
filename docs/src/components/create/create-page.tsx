"use client";

import {
  Check,
  ChevronDown,
  Fingerprint,
  Grid2X2,
  Link as LinkIcon,
  RotateCcw,
  Shuffle,
  SlidersHorizontal,
} from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { useRouter } from "fumapress/client";
import { useTheme } from "next-themes";
import {
  defaultPreset,
  encodePreset,
  options,
  parsePreset,
  shufflePreset,
  type Axis,
  type Preset,
} from "@/lib/create-preset";
import { SiteHeader } from "../shell/header";
import { useHydrated } from "../use-hydrated";
import { createCopy, optionLabel } from "./copy";
import { GetCode, OpenPreset } from "./preset-dialogs";

const groups = [
  ["style"],
  ["base", "accent", "chart"],
  ["heading", "font"],
  ["icons", "radius"],
] as const;
const swatches: Record<string, string> = {
  neutral: "#a3a3a3",
  zinc: "#a1a1aa",
  stone: "#a8a29e",
  sage: "#83ad95",
  lavender: "#b1a4cb",
  clay: "#c5a78d",
  graphite: "#8b949d",
  blue: "#60a5fa",
  green: "#4ade80",
  violet: "#a78bfa",
  rose: "#fb7185",
  ocean: "#66abc7",
  forest: "#91ad61",
  sunset: "#eaa350",
};
const sideButton =
  "inline-flex min-h-9 items-center justify-center gap-2 rounded-[7px] border border-white/15 bg-white/[0.025] px-2.5 text-sm font-medium text-neutral-100 hover:bg-white/10 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white disabled:opacity-50";

function SettingMark({ axis, value }: { axis: Axis; value: string }) {
  if (axis === "base" || axis === "accent" || axis === "chart") {
    return (
      <span
        aria-hidden
        className="size-4 rounded-full ring-1 ring-inset ring-white/15"
        style={{ background: swatches[value] }}
      />
    );
  }
  if (axis === "heading" || axis === "font") {
    return (
      <span
        aria-hidden
        className={`text-[17px] leading-none ${value === "space-mono" ? "font-mono" : value === "spectral" ? "font-serif" : ""}`}
      >
        Aa
      </span>
    );
  }
  if (axis === "icons") {
    const Icon = value === "outline" ? Fingerprint : Grid2X2;
    return <Icon aria-hidden className="size-[17px]" />;
  }
  if (axis === "radius") {
    return (
      <span
        aria-hidden
        className="size-[15px] border-t-2 border-r-2"
        style={{
          borderTopRightRadius:
            value === "none" ? 0 : value === "small" ? 3 : value === "large" ? 12 : 7,
        }}
      />
    );
  }
  return (
    <span
      aria-hidden
      className={`h-[13px] w-[17px] border-[1.5px] ${value === "neobrutal" ? "rounded-[1px] shadow-[2px_2px_currentColor]" : value === "neumorphism" ? "rounded-[5px] bg-neutral-500 shadow-[1px_2px_3px_#161616]" : "rounded-[4px]"}`}
    />
  );
}

function readSelection(query: string) {
  const code = new URLSearchParams(query).get("preset");
  const preset = code === null ? defaultPreset : parsePreset(code);
  return { preset: preset ?? defaultPreset, invalid: !preset };
}

export function CreatePage({ lang, title }: { lang: string; title: string }) {
  const t = createCopy(lang);
  const { query, push } = useRouter();
  const { resolvedTheme } = useTheme();
  const selection = useHydrated() ? readSelection(query) : null;
  const [expanded, setExpanded] = useState(false);
  const [shareResult, setShareResult] = useState<{ code: string; status: "copied" | "manual" }>();
  const shareInput = useRef<HTMLInputElement>(null);
  const preset = selection?.preset ?? defaultPreset;
  const code = encodePreset(preset);
  const shared = shareResult?.code === code ? shareResult.status : "idle";
  const posterTheme = resolvedTheme === "dark" ? "-dark" : "";

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
    if (url.href !== window.location.href) void push(url.pathname + url.search + url.hash);
    setShareResult(undefined);
  }

  function shareUrl() {
    const url = new URL(window.location.href);
    url.searchParams.set("preset", code);
    return url.href;
  }

  return (
    <div className="create-studio flex h-dvh min-h-[25rem] flex-col">
      <SiteHeader lang={lang} />
      <main
        id="main-content"
        className="mx-auto flex w-full min-h-0 min-w-0 max-w-[1800px] flex-1 flex-col gap-2 px-3 pt-1 pb-[max(1rem,env(safe-area-inset-bottom))] min-[360px]:px-4 md:px-6 md:pt-3.5 md:pb-6 [html:lang(ko)_&]:break-keep"
      >
        {selection?.invalid && (
          <p role="alert" className="rounded-lg border px-3 py-2 text-sm">
            {t.invalidLink}
          </p>
        )}
        <div className="grid min-h-0 min-w-0 flex-1 grid-cols-[minmax(0,1fr)] grid-rows-[minmax(0,1fr)_auto] gap-3 md:grid-cols-[12rem_minmax(0,1fr)] md:grid-rows-[minmax(0,1fr)] md:gap-6">
          <aside
            aria-label={t.settings}
            className="row-start-2 grid min-w-0 grid-cols-[minmax(0,1fr)_auto_auto] items-center gap-2 overflow-hidden rounded-2xl bg-[#303030] p-3 text-neutral-50 md:col-start-1 md:row-start-1 md:flex md:min-h-0 md:flex-col md:items-stretch md:gap-0 md:rounded-[18px] md:p-0"
          >
            <div className="contents md:block md:shrink-0 md:border-b md:border-white/10 md:p-3">
              <h1 className="sr-only md:not-sr-only md:flex md:h-10 md:items-center md:justify-between md:rounded-lg md:border md:border-white/15 md:bg-[#262626] md:px-3 md:text-[15px] md:font-semibold">
                {title}
                <SlidersHorizontal
                  aria-hidden
                  className="hidden size-4 text-neutral-200 md:block"
                />
              </h1>
              <button
                type="button"
                className="col-start-1 row-start-2 flex min-h-9 min-w-0 items-center gap-2 rounded-md px-2 text-start text-sm font-medium hover:bg-white/10 md:hidden"
                aria-expanded={expanded}
                aria-controls="preset-settings"
                onClick={() => setExpanded(!expanded)}
              >
                <SlidersHorizontal aria-hidden className="size-4 shrink-0" />
                <span className="truncate">{t.customize}</span>
                <ChevronDown
                  aria-hidden
                  className={`size-3.5 shrink-0 text-neutral-300 ${expanded ? "rotate-180" : ""}`}
                />
              </button>
            </div>
            <div
              id="preset-settings"
              className={`col-span-full row-start-1 min-w-0 overscroll-contain border-b border-white/10 pb-2 [scrollbar-width:thin] [scrollbar-color:var(--color-neutral-500)_transparent] md:min-h-0 md:flex-1 md:overflow-x-hidden md:overflow-y-auto md:border-0 md:pb-0 ${expanded ? "max-h-[40dvh] overflow-y-auto md:max-h-none" : "overflow-x-auto"}`}
            >
              <fieldset
                disabled={!selection}
                className={`min-w-0 gap-1.5 md:block md:divide-y md:divide-white/10 ${expanded ? "grid grid-cols-2" : "flex"}`}
              >
                <legend className="sr-only">{t.settings}</legend>
                {groups.map((group) => (
                  <div key={group[0]} className="contents md:grid md:gap-1.5 md:px-3 md:py-3">
                    {group.map((axis) => {
                      const choices = options[axis];
                      const selected = choices.find((option) => option.value === preset[axis])!;
                      return (
                        <label
                          key={axis}
                          className={`relative flex min-h-14 items-center justify-between gap-2 rounded-[9px] border border-white/10 px-3 py-1 hover:border-white/30 hover:bg-white/[0.035] focus-within:ring-2 focus-within:ring-inset focus-within:ring-white md:min-h-12 md:w-auto ${expanded ? "min-w-0" : "w-[9rem] shrink-0"}`}
                        >
                          <span className="grid min-w-0 gap-0.5">
                            <span className="text-xs leading-4 text-neutral-300">
                              {t.axes[axis]}
                            </span>
                            <span
                              aria-hidden
                              className="truncate text-[14px] leading-4 font-medium"
                            >
                              {optionLabel(selected, lang)}
                            </span>
                          </span>
                          <SettingMark axis={axis} value={preset[axis]} />
                          <select
                            aria-label={t.axes[axis]}
                            value={preset[axis]}
                            onChange={(event) => apply({ ...preset, [axis]: event.target.value })}
                            className="absolute inset-0 h-full w-full cursor-pointer opacity-0 [color-scheme:dark]"
                          >
                            {choices.map((option) => (
                              <option key={option.value} value={option.value}>
                                {optionLabel(option, lang)}
                              </option>
                            ))}
                          </select>
                        </label>
                      );
                    })}
                  </div>
                ))}
              </fieldset>
            </div>
            <div
              className={`col-span-full row-start-3 grid-cols-[minmax(0,1fr)_auto_auto] gap-1.5 md:grid md:shrink-0 md:grid-cols-[minmax(0,1fr)_auto] md:border-t md:border-white/10 md:p-3 ${expanded ? "grid" : "hidden"}`}
            >
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
                <span className="hidden truncate md:block">{t.shuffle}</span>
              </button>
              <button
                type="button"
                onClick={() => apply(defaultPreset, true)}
                disabled={!selection}
                aria-label={t.reset}
                title={t.reset}
                className={sideButton}
              >
                <RotateCcw aria-hidden className="size-4" />
              </button>
            </div>
            <div className="contents md:grid md:shrink-0 md:grid-cols-[auto_minmax(0,1fr)] md:gap-1.5 md:border-t md:border-white/10 md:p-3">
              <button
                type="button"
                disabled={!selection}
                className={`${sideButton} col-start-2 row-start-2 md:col-start-1 md:row-start-1`}
                aria-label={shared === "copied" ? t.linkCopied : t.share}
                title={shared === "copied" ? t.linkCopied : t.share}
                onClick={async () => {
                  try {
                    await navigator.clipboard.writeText(shareUrl());
                    setShareResult({ code, status: "copied" });
                  } catch {
                    setShareResult({ code, status: "manual" });
                  }
                }}
              >
                {shared === "copied" ? (
                  <Check aria-hidden className="size-4" />
                ) : (
                  <LinkIcon aria-hidden className="size-4" />
                )}
              </button>
              <GetCode
                preset={preset}
                dark={resolvedTheme === "dark"}
                lang={lang}
                t={t}
                ready={selection !== null}
              />
              {shared === "manual" && (
                <label className="col-span-full row-start-4 grid min-w-0 gap-1 px-1 py-1 text-xs md:row-start-2">
                  {t.shareLink}
                  <input
                    ref={shareInput}
                    readOnly
                    value={shareUrl()}
                    onFocus={(e) => e.currentTarget.select()}
                    className="h-9 w-full min-w-0 rounded-md border border-white/20 bg-[#262626] px-2 font-mono text-xs"
                  />
                </label>
              )}
              <output className="sr-only">{shared === "copied" ? t.linkCopied : ""}</output>
            </div>
          </aside>
          <section
            aria-label={t.preview}
            aria-busy={!selection}
            className="relative col-start-1 row-start-1 min-h-0 min-w-0 overflow-hidden rounded-2xl bg-neutral-100 md:col-start-2 md:rounded-[20px] dark:bg-neutral-900"
          >
            {selection &&
              (navigator.gpu ? (
                <iframe
                  title={t.preview}
                  src={import.meta.env.DEV ? "/preview/index.html?c=create" : "/preview?c=create"}
                  data-preset={code}
                  className="absolute inset-0 block size-full"
                />
              ) : (
                <figure className="flex h-full flex-col">
                  <div className="min-h-0 flex-1 overflow-y-auto">
                    <picture>
                      <source
                        media="(width < 64rem)"
                        srcSet={`/images/create/mobile${posterTheme}.webp`}
                      />
                      <img
                        src={`/images/create/desktop${posterTheme}.webp`}
                        alt={t.posterAlt}
                        width={1176}
                        height={806}
                        className="block h-auto w-full"
                      />
                    </picture>
                  </div>
                  <figcaption className="shrink-0 border-t border-black/10 px-3 py-2 text-xs leading-5 text-neutral-600 md:px-5 dark:border-white/10 dark:text-neutral-300">
                    {t.unavailable}
                  </figcaption>
                </figure>
              ))}
          </section>
        </div>
      </main>
    </div>
  );
}
