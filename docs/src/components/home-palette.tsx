"use client";

import { Check } from "lucide-react";

import { accents } from "@/lib/accent";
import { copyFor } from "./shell/copy";
import { selectAccent, useAccent } from "./use-accent";

export function HomePalette({
  lang,
  title,
  description,
}: {
  lang: string;
  title: string;
  description: string;
}) {
  const accent = useAccent();
  const t = copyFor(lang);
  return (
    <section className="rounded-2xl border bg-fd-background p-3.5 shadow-xs sm:rounded-3xl sm:p-6">
      <h2 className="text-base leading-5 font-semibold tracking-tight sm:text-lg sm:leading-6">
        {title}
      </h2>
      <p className="mt-1 text-xs leading-relaxed text-fd-muted-foreground sm:text-sm">
        {description}
      </p>
      <fieldset className="mt-3 min-w-0 sm:mt-5">
        <legend className="mb-2 text-xs font-medium sm:text-sm">
          <span className="sr-only sm:not-sr-only">{t.accent} · </span>
          {t.colors[accent]}
        </legend>
        <div className="flex flex-wrap gap-1 sm:gap-2">
          {accents.map((color) => (
            <label key={color} className="cursor-pointer" title={t.colors[color]}>
              <input
                type="radio"
                name="home-accent"
                value={color}
                checked={color === accent}
                onChange={() => selectAccent(color)}
                aria-label={t.colors[color]}
                className="peer sr-only"
              />
              <span
                data-swatch={color}
                className="accent-swatch flex size-6 items-center justify-center rounded-full text-white peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-fd-ring sm:size-9 dark:text-black"
              >
                {color === accent && <Check aria-hidden className="size-4" />}
              </span>
            </label>
          ))}
        </div>
      </fieldset>
    </section>
  );
}
