"use client";

import { Menu } from "@base-ui/react/menu";
import { Check, Languages, Moon, Palette, Sun } from "lucide-react";
import { useTheme } from "next-themes";
import type { RefObject } from "react";

import { accents } from "@/lib/accent";
import { selectAccent, useAccent } from "../use-accent";
import { copyFor, languages } from "./copy";

const popup =
  "min-w-36 rounded-lg border bg-fd-popover p-1 text-sm text-fd-popover-foreground shadow-md outline-none";
const item =
  "flex h-9 items-center gap-2 rounded-md px-2 outline-none data-[highlighted]:bg-fd-accent aria-[current=true]:font-semibold";

export function SitePreferences({
  lang,
  path,
  panel,
}: {
  lang: string;
  path: string;
  panel?: RefObject<HTMLDivElement | null>;
}) {
  const t = copyFor(lang);
  const accent = useAccent();
  const { resolvedTheme, setTheme } = useTheme();
  const button = panel
    ? "inline-flex min-h-10 items-center gap-2 rounded-lg border px-3 text-sm font-medium hover:bg-fd-accent"
    : "inline-flex h-8 min-w-8 items-center justify-center gap-1.5 rounded-md px-1.5 hover:bg-fd-accent";
  const accentLabel = `${t.accent} · ${t.colors[accent]}`;

  return (
    <>
      <Menu.Root modal={false}>
        <Menu.Trigger aria-label={t.language} title={t.language} className={button}>
          <Languages aria-hidden className="size-4" />
          {panel && languages.find((language) => language.code === lang)?.name}
        </Menu.Trigger>
        <Menu.Portal container={panel}>
          <Menu.Positioner sideOffset={6} align={panel ? "start" : "end"} className="z-[60]">
            <Menu.Popup className={popup}>
              {languages.map((language) => (
                <Menu.LinkItem
                  key={language.code}
                  href={path.replace(new RegExp(`^/${lang}(?=/|$)`), `/${language.code}`)}
                  lang={language.code}
                  aria-current={language.code === lang ? "true" : undefined}
                  className={item}
                >
                  {language.name}
                </Menu.LinkItem>
              ))}
            </Menu.Popup>
          </Menu.Positioner>
        </Menu.Portal>
      </Menu.Root>
      <Menu.Root modal={false}>
        <Menu.Trigger aria-label={accentLabel} title={accentLabel} className={button}>
          <Palette aria-hidden className="size-4" />
          <span className={panel ? undefined : "text-xs font-medium"}>{t.colors[accent]}</span>
        </Menu.Trigger>
        <Menu.Portal container={panel}>
          <Menu.Positioner sideOffset={6} align={panel ? "start" : "end"} className="z-[60]">
            <Menu.Popup aria-label={t.accent} className={popup}>
              <Menu.RadioGroup value={accent} onValueChange={selectAccent}>
                {accents.map((color) => (
                  <Menu.RadioItem key={color} value={color} closeOnClick className={item}>
                    <span
                      aria-hidden
                      data-swatch={color}
                      className="accent-swatch size-3 rounded-full"
                    />
                    {t.colors[color]}
                    <Menu.RadioItemIndicator className="ml-auto">
                      <Check aria-hidden className="size-3.5" />
                    </Menu.RadioItemIndicator>
                  </Menu.RadioItem>
                ))}
              </Menu.RadioGroup>
            </Menu.Popup>
          </Menu.Positioner>
        </Menu.Portal>
      </Menu.Root>
      <button
        type="button"
        aria-label={t.theme}
        title={t.theme}
        onClick={() => setTheme(resolvedTheme === "dark" ? "light" : "dark")}
        className={button}
      >
        <Sun aria-hidden className="size-4 dark:hidden" />
        <Moon aria-hidden className="hidden size-4 dark:block" />
        {panel && (
          <>
            <span className="dark:hidden">{t.light}</span>
            <span className="hidden dark:inline">{t.dark}</span>
          </>
        )}
      </button>
    </>
  );
}
