"use client";

import { Menu as DropdownMenu } from "@base-ui/react/menu";
import { Link, useRouter } from "fumapress/client";
import { useSearchContext } from "fumadocs-ui/contexts/search";
import { Check, Languages, Menu, Moon, Palette, Search, Sun, X } from "lucide-react";
import { useTheme } from "next-themes";

import { accents } from "@/lib/accent";
import { GIT, SITE_NAME } from "@/lib/site";
import { selectAccent, useAccent } from "../use-accent";
import { copyFor, languages } from "./copy";

function GitHubIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true" className="size-4 fill-current">
      <path d="M12 .3a12 12 0 0 0-3.8 23.4c.6.1.8-.3.8-.6v-2c-3.3.7-4-1.6-4-1.6-.6-1.4-1.4-1.8-1.4-1.8-1-.7.1-.7.1-.7 1.2.1 1.8 1.2 1.8 1.2 1 1.8 2.8 1.3 3.5 1 0-.8.4-1.3.7-1.6-2.7-.3-5.5-1.3-5.5-6 0-1.2.5-2.3 1.3-3.1-.2-.4-.6-1.6 0-3.2 0 0 1-.3 3.3 1.2a11.5 11.5 0 0 1 6 0C17.3 4.7 18.3 5 18.3 5c.6 1.6.2 2.8.1 3.2.8.8 1.3 1.9 1.3 3.1 0 4.7-2.8 5.7-5.5 6 .4.4.8 1.1.8 2.2v3.3c0 .3.2.7.8.6A12 12 0 0 0 12 .3" />
    </svg>
  );
}

const iconButton =
  "inline-flex size-8 items-center justify-center rounded-md text-fd-foreground transition-colors hover:bg-fd-accent";
const menuPopup =
  "min-w-36 rounded-lg border bg-fd-popover p-1 text-sm text-fd-popover-foreground shadow-md outline-none";
const menuItem =
  "flex items-center rounded-md px-2 outline-none data-[highlighted]:bg-fd-accent aria-[current=true]:font-semibold";

/** shadcn/ui's site header: logo and section links, then search, language, GitHub and theme. */
export function SiteHeader({
  lang,
  menuOpen,
  onToggleMenu,
}: {
  lang: string;
  menuOpen?: boolean;
  onToggleMenu?: () => void;
}) {
  const t = copyFor(lang);
  const { path } = useRouter();
  const { setOpenSearch } = useSearchContext();
  const { resolvedTheme, setTheme } = useTheme();
  const accent = useAccent();
  const accentLabel = `${t.accent} · ${t.colors[accent]}`;
  const current = path.replace(/\/$/, "");
  const links = [
    { href: `/${lang}/docs`, label: t.docs, exact: true },
    { href: `/${lang}/docs/components`, label: t.components },
    { href: `/${lang}/docs/recipes`, label: t.recipes },
    { href: `/${lang}/docs/guides/blocks`, label: t.blocks, exact: true },
    { href: `/${lang}/docs/guides/forms`, label: t.guides, prefix: `/${lang}/docs/guides` },
  ];
  const activeLink =
    links.find((link) => current === link.href) ??
    links.find((link) => !link.exact && current.startsWith(link.prefix ?? link.href));
  const pathIn = (code: string) =>
    current.replace(new RegExp(`^/${lang}(?=/|$)`), `/${code}`) || `/${code}`;

  return (
    <header className="sticky top-0 z-40 w-full bg-fd-background/95 backdrop-blur">
      <div className="mx-auto flex h-14 w-full max-w-[1480px] items-center gap-1 px-2 min-[360px]:gap-2 min-[360px]:px-4 md:px-6">
        {onToggleMenu ? (
          <button
            type="button"
            aria-label={t.menu}
            aria-expanded={menuOpen}
            onClick={onToggleMenu}
            className={`${iconButton} lg:hidden`}
          >
            {menuOpen ? <X className="size-5" /> : <Menu className="size-5" />}
          </button>
        ) : (
          <DropdownMenu.Root modal={false}>
            <DropdownMenu.Trigger aria-label={t.menu} className={`${iconButton} md:hidden`}>
              <Menu className="size-5" />
            </DropdownMenu.Trigger>
            <DropdownMenu.Portal>
              <DropdownMenu.Positioner sideOffset={6} align="start" className="z-50 md:hidden">
                <DropdownMenu.Popup className={menuPopup}>
                  {links.map((link) => (
                    <DropdownMenu.LinkItem
                      key={link.href}
                      render={<Link href={link.href} />}
                      aria-current={link === activeLink ? "page" : undefined}
                      closeOnClick
                      className={`${menuItem} h-11`}
                    >
                      {link.label}
                    </DropdownMenu.LinkItem>
                  ))}
                </DropdownMenu.Popup>
              </DropdownMenu.Positioner>
            </DropdownMenu.Portal>
          </DropdownMenu.Root>
        )}
        <Link
          href={`/${lang}`}
          className="flex shrink-0 items-center gap-1.5 text-sm font-semibold tracking-tight lg:mr-3 sm:text-base"
        >
          <img src="/deck.svg" alt="" width="23" height="23" className="hidden sm:block" />
          {SITE_NAME}
        </Link>
        <nav className="hidden items-center gap-1 text-sm md:flex">
          {links.map((link) => {
            const active = link === activeLink;
            return (
              <Link
                key={link.href}
                href={link.href}
                aria-current={active ? "page" : undefined}
                className={`rounded-md px-2.5 py-1.5 font-medium transition-colors hover:text-fd-foreground ${
                  active ? "bg-fd-secondary text-fd-primary" : "text-fd-foreground/85"
                }`}
              >
                {link.label}
              </Link>
            );
          })}
        </nav>
        <div className="ml-auto flex items-center gap-1">
          <button
            type="button"
            onClick={() => setOpenSearch(true)}
            className="hidden h-8 w-48 items-center gap-2 rounded-full bg-fd-secondary px-3 text-sm text-fd-muted-foreground transition-colors hover:bg-fd-accent lg:flex xl:w-64"
          >
            <span className="flex-1 text-left">{t.search}</span>
            <kbd className="rounded border bg-fd-background px-1.5 font-mono text-[10px] font-medium">
              Ctrl K
            </kbd>
          </button>
          <button
            type="button"
            aria-label={t.search}
            onClick={() => setOpenSearch(true)}
            className={`${iconButton} lg:hidden`}
          >
            <Search className="size-4" />
          </button>
          <DropdownMenu.Root>
            <DropdownMenu.Trigger aria-label={t.language} title={t.language} className={iconButton}>
              <Languages className="size-4" />
            </DropdownMenu.Trigger>
            <DropdownMenu.Portal>
              <DropdownMenu.Positioner sideOffset={6} align="end" className="z-50">
                <DropdownMenu.Popup className={menuPopup}>
                  {languages.map((language) => (
                    <DropdownMenu.LinkItem
                      key={language.code}
                      href={pathIn(language.code)}
                      lang={language.code}
                      aria-current={language.code === lang ? "true" : undefined}
                      className={`${menuItem} h-8`}
                    >
                      {language.name}
                    </DropdownMenu.LinkItem>
                  ))}
                </DropdownMenu.Popup>
              </DropdownMenu.Positioner>
            </DropdownMenu.Portal>
          </DropdownMenu.Root>
          <a
            href={`https://github.com/${GIT.user}/${GIT.repo}`}
            target="_blank"
            rel="noreferrer"
            aria-label="GitHub"
            className={iconButton}
          >
            <GitHubIcon />
          </a>
          <DropdownMenu.Root modal={false}>
            <DropdownMenu.Trigger
              aria-label={accentLabel}
              title={accentLabel}
              className="inline-flex h-8 items-center justify-center gap-1.5 rounded-md px-2 text-fd-primary hover:bg-fd-accent"
            >
              <Palette className="size-4" aria-hidden />
              <span className="hidden text-xs font-medium lg:inline">{t.colors[accent]}</span>
            </DropdownMenu.Trigger>
            <DropdownMenu.Portal>
              <DropdownMenu.Positioner sideOffset={6} align="end" className="z-50">
                <DropdownMenu.Popup aria-label={t.accent} className={menuPopup}>
                  <DropdownMenu.RadioGroup value={accent} onValueChange={selectAccent}>
                    {accents.map((color) => (
                      <DropdownMenu.RadioItem
                        key={color}
                        value={color}
                        closeOnClick
                        className={`${menuItem} h-9 gap-2`}
                      >
                        <span
                          aria-hidden
                          data-swatch={color}
                          className="accent-swatch size-3 rounded-full"
                        />
                        {t.colors[color]}
                        <DropdownMenu.RadioItemIndicator className="ml-auto">
                          <Check aria-hidden className="size-3.5" />
                        </DropdownMenu.RadioItemIndicator>
                      </DropdownMenu.RadioItem>
                    ))}
                  </DropdownMenu.RadioGroup>
                </DropdownMenu.Popup>
              </DropdownMenu.Positioner>
            </DropdownMenu.Portal>
          </DropdownMenu.Root>
          <button
            type="button"
            aria-label={t.theme}
            onClick={() => setTheme(resolvedTheme === "dark" ? "light" : "dark")}
            className={iconButton}
          >
            <Sun className="size-4 dark:hidden" />
            <Moon className="hidden size-4 dark:block" />
          </button>
        </div>
      </div>
    </header>
  );
}
