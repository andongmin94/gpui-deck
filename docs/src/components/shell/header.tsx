"use client";

import { Link, useRouter } from "fumapress/client";
import { useSearchContext } from "fumadocs-ui/contexts/search";
import { Languages, Menu, Moon, Search, Sun, X } from "lucide-react";
import { useTheme } from "next-themes";

import { GIT, SITE_NAME } from "@/lib/site";
import { copyFor, otherLanguage } from "./copy";

function GitHubIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true" className="size-4 fill-current">
      <path d="M12 .3a12 12 0 0 0-3.8 23.4c.6.1.8-.3.8-.6v-2c-3.3.7-4-1.6-4-1.6-.6-1.4-1.4-1.8-1.4-1.8-1-.7.1-.7.1-.7 1.2.1 1.8 1.2 1.8 1.2 1 1.8 2.8 1.3 3.5 1 0-.8.4-1.3.7-1.6-2.7-.3-5.5-1.3-5.5-6 0-1.2.5-2.3 1.3-3.1-.2-.4-.6-1.6 0-3.2 0 0 1-.3 3.3 1.2a11.5 11.5 0 0 1 6 0C17.3 4.7 18.3 5 18.3 5c.6 1.6.2 2.8.1 3.2.8.8 1.3 1.9 1.3 3.1 0 4.7-2.8 5.7-5.5 6 .4.4.8 1.1.8 2.2v3.3c0 .3.2.7.8.6A12 12 0 0 0 12 .3" />
    </svg>
  );
}

const iconButton =
  "inline-flex size-8 items-center justify-center rounded-md text-fd-foreground transition-colors hover:bg-fd-accent";

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
  const other = otherLanguage(lang);
  const current = path.replace(/\/$/, "");
  const links = [
    { href: `/${lang}/docs`, label: t.docs, exact: true },
    { href: `/${lang}/docs/components`, label: t.components },
    { href: `/${lang}/docs/recipes`, label: t.recipes },
    { href: `/${lang}/docs/guides/forms`, label: t.guides, prefix: `/${lang}/docs/guides` },
  ];
  const otherPath = current.replace(new RegExp(`^/${lang}(?=/|$)`), `/${other}`);

  return (
    <header className="sticky top-0 z-40 w-full bg-fd-background/95 backdrop-blur">
      <div className="mx-auto flex h-14 w-full max-w-[1400px] items-center gap-2 px-4 md:px-6">
        {onToggleMenu && (
          <button
            type="button"
            aria-label={t.menu}
            aria-expanded={menuOpen}
            onClick={onToggleMenu}
            className={`${iconButton} lg:hidden`}
          >
            {menuOpen ? <X className="size-5" /> : <Menu className="size-5" />}
          </button>
        )}
        <Link href={`/${lang}`} className="mr-4 flex items-center gap-2 font-semibold">
          <span className="size-5 rounded-[5px] bg-fd-foreground" aria-hidden />
          <span className="hidden sm:inline">{SITE_NAME}</span>
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
          <Link
            href={otherPath || `/${other}`}
            aria-label={t.language}
            title={t.language}
            className={iconButton}
          >
            <Languages className="size-4" />
          </Link>
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
