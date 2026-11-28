"use client";

import { Link } from "fumapress/client";
import { ArrowRight, ChevronDown } from "lucide-react";
import { useState } from "react";
import { defaultPreset, encodePreset, options } from "@/lib/create-preset";
import { createCopy, optionLabel } from "./create/copy";
import { HomePresetSample } from "./home-preset-sample";
import { useHydrated } from "./use-hydrated";

export function HomePreset({ lang, title }: { lang: string; title: string }) {
  const [preset, setPreset] = useState(defaultPreset);
  const ready = useHydrated();
  const t = createCopy(lang);
  return (
    <div className="home-preset grid min-w-0 items-start gap-3 md:grid-cols-[minmax(13rem,1fr)_minmax(0,2.2fr)] xl:gap-5">
      <section className="rounded-3xl border bg-fd-background p-5 shadow-xs">
        <h2 className="text-base font-semibold tracking-tight">{title}</h2>
        <fieldset disabled={!ready} className="mt-4 grid min-w-0 grid-cols-2 gap-3 md:grid-cols-1">
          <legend className="sr-only">{title}</legend>
          {(["style", "radius"] as const).map((axis) => (
            <label key={axis} className="grid gap-1.5 text-sm font-medium">
              {t.axes[axis]}
              <span className="relative">
                <select
                  className="h-10 w-full appearance-none rounded-xl bg-fd-secondary ps-3 pe-8 text-sm font-normal focus-visible:outline-offset-2"
                  value={preset[axis]}
                  onChange={(event) => setPreset({ ...preset, [axis]: event.target.value })}
                >
                  {options[axis].map((option) => (
                    <option key={option.value} value={option.value}>
                      {optionLabel(option, lang)}
                    </option>
                  ))}
                </select>
                <ChevronDown
                  aria-hidden
                  className="pointer-events-none absolute end-3 top-3.5 size-3.5 text-fd-muted-foreground"
                />
              </span>
            </label>
          ))}
          <fieldset className="col-span-full min-w-0">
            <legend className="mb-1.5 text-sm font-medium">{t.axes.accent}</legend>
            <div className="flex flex-wrap gap-1">
              {options.accent.map((option) => (
                <label
                  key={option.value}
                  title={optionLabel(option, lang)}
                  className="inline-flex size-8 cursor-pointer items-center justify-center"
                >
                  <input
                    type="radio"
                    name="home-preset-accent"
                    value={option.value}
                    checked={preset.accent === option.value}
                    onChange={() => setPreset({ ...preset, accent: option.value })}
                    aria-label={optionLabel(option, lang)}
                    className="peer sr-only"
                  />
                  <span
                    aria-hidden
                    data-swatch={option.value}
                    className="accent-swatch size-5 rounded-full peer-checked:ring-1 peer-checked:ring-fd-foreground peer-checked:ring-offset-2 peer-checked:ring-offset-fd-background peer-focus-visible:outline-2 peer-focus-visible:outline-offset-4"
                  />
                </label>
              ))}
            </div>
          </fieldset>
        </fieldset>
        <Link
          href={`/${lang}/create?preset=${encodePreset(preset)}`}
          className="mt-5 flex min-h-9 items-center justify-center gap-2 rounded-full bg-fd-primary px-4 text-sm font-medium text-fd-primary-foreground hover:opacity-85"
        >
          {t.open}
          <ArrowRight aria-hidden className="size-3.5" />
        </Link>
      </section>
      <HomePresetSample lang={lang} preset={preset} />
    </div>
  );
}
