"use client";

import { Menu as DropdownMenu } from "@base-ui/react/menu";
import { Link, useRouter } from "fumapress/client";
import { useSearchContext } from "fumadocs-ui/contexts/search";
import { Languages, Menu, Moon, Search, Sun, X } from "lucide-react";
import { useTheme } from "next-themes";

import { GIT, SITE_NAME } from "@/lib/site";
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
  const current = path.replace(/\/$/, "");
  const links = [
    { href: `/${lang}/docs`, label: t.docs, exact: true },
    { href: `/${lang}/docs/components`, label: t.components },
    { href: `/${lang}/docs/recipes`, label: t.recipes },
    { href: `/${lang}/docs/guides/forms`, label: t.guides, prefix: `/${lang}/docs/guides` },
  ];
  const pathIn = (code: string) =>
    current.replace(new RegExp(`^/${lang}(?=/|$)`), `/${code}`) || `/${code}`;

  return (
    <header className="sticky top-0 z-40 w-full bg-fd-background/95 backdrop-blur">
      <div className="mx-auto flex h-14 w-full max-w-[1400px] items-center gap-2 px-4 md:px-6">
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
          className="flex shrink-0 items-center gap-2 text-sm font-semibold sm:mr-4 sm:text-base"
        >
          <span className="hidden size-5 rounded-[5px] bg-fd-foreground sm:block" aria-hidden />
          {SITE_NAME}
        </Link>
        <nav className="hidden items-center gap-1 text-sm md:flex">
          {links.map((link) => {
            const active = link.exact
              ? current === link.href
              : current.startsWith(link.prefix ?? link.href);
            return (
              <Link
                key={link.href}
                href={link.href}
                aria-current={active ? "page" : undefined}
                className={`rounded-md px-2.5 py-1.5 font-medium transition-colors hover:text-fd-foreground ${
                  active ? "text-fd-foreground" : "text-fd-muted-foreground"
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
            className="hidden h-8 w-56 items-center gap-2 rounded-md bg-fd-secondary px-3 text-sm text-fd-muted-foreground transition-colors hover:bg-fd-accent sm:flex lg:w-64"
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
            className={`${iconButton} sm:hidden`}
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
