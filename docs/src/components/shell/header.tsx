"use client";

import { Dialog } from "@base-ui/react/dialog";
import type * as PageTree from "fumadocs-core/page-tree";
import { Link, useRouter } from "fumapress/client";
import { useSearchContext } from "fumadocs-ui/contexts/search";
import { ArrowUpRight, BookOpen, Menu, Plus, Search, Terminal, X } from "lucide-react";
import { useEffect, useRef, useState } from "react";

import { GIT, SITE_NAME } from "@/lib/site";
import { copyFor } from "./copy";
import { SitePreferences } from "./preferences";
import { SidebarNav } from "./sidebar";

function GitHubIcon() {
  return (
    <svg viewBox="0 0 24 24" aria-hidden="true" className="size-4 fill-current">
      <path d="M12 .3a12 12 0 0 0-3.8 23.4c.6.1.8-.3.8-.6v-2c-3.3.7-4-1.6-4-1.6-.6-1.4-1.4-1.8-1.4-1.8-1-.7.1-.7.1-.7 1.2.1 1.8 1.2 1.8 1.2 1 1.8 2.8 1.3 3.5 1 0-.8.4-1.3.7-1.6-2.7-.3-5.5-1.3-5.5-6 0-1.2.5-2.3 1.3-3.1-.2-.4-.6-1.6 0-3.2 0 0 1-.3 3.3 1.2a11.5 11.5 0 0 1 6 0C17.3 4.7 18.3 5 18.3 5c.6 1.6.2 2.8.1 3.2.8.8 1.3 1.9 1.3 3.1 0 4.7-2.8 5.7-5.5 6 .4.4.8 1.1.8 2.2v3.3c0 .3.2.7.8.6A12 12 0 0 0 12 .3" />
    </svg>
  );
}

const iconButton =
  "inline-flex size-9 shrink-0 items-center justify-center rounded-md hover:bg-fd-accent";
const github = `https://github.com/${GIT.user}/${GIT.repo}`;

type Destination = { href: string; label: string };

function SectionLinks({
  links,
  current,
  onNavigate,
}: {
  links: Destination[];
  current: string;
  onNavigate?: () => void;
}) {
  // A deeper section (e.g. Components) takes precedence over Docs.
  const active = [...links]
    .reverse()
    .find(({ href }) => current === href || current.startsWith(href + "/"));
  return links.map((link) => (
    <Link
      key={link.href}
      href={link.href}
      aria-current={link === active ? (current === link.href ? "page" : "location") : undefined}
      onClick={onNavigate}
      className={
        onNavigate
          ? "flex min-h-12 items-center rounded-md text-2xl font-medium tracking-tight hover:underline aria-[current]:underline underline-offset-4"
          : "inline-flex h-9 items-center rounded-md px-2.5 text-sm font-medium hover:bg-fd-accent aria-[current]:bg-fd-secondary"
      }
    >
      {link.label}
    </Link>
  ));
}

function MobileNavigation({
  lang,
  links,
  current,
  tree,
}: {
  lang: string;
  links: Destination[];
  current: string;
  tree?: PageTree.Root;
}) {
  const t = copyFor(lang);
  const [open, setOpen] = useState(false);
  const panel = useRef<HTMLDivElement>(null);
  const close = () => setOpen(false);

  useEffect(() => {
    if (!open) return;
    const desktop = window.matchMedia("(min-width: 64rem)");
    const onResize = () => {
      if (desktop.matches) setOpen(false);
    };
    desktop.addEventListener("change", onResize);
    return () => desktop.removeEventListener("change", onResize);
  }, [open]);

  const resources = [
    { href: `/${lang}/docs/installation`, label: t.installation, Icon: Terminal },
    { href: `/${lang}/docs/guides`, label: t.guides, Icon: BookOpen },
    { href: `/${lang}/docs/guides/blocks#publish`, label: t.publish, Icon: Plus },
  ];

  return (
    <Dialog.Root open={open} onOpenChange={setOpen}>
      <Dialog.Trigger aria-label={t.menu} className={`${iconButton} lg:hidden`}>
        <Menu className="size-5" aria-hidden />
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Popup
          ref={panel}
          className="fixed inset-0 z-50 flex h-dvh flex-col bg-fd-background text-fd-foreground outline-none"
        >
          <div className="flex h-14 shrink-0 items-center justify-between border-b px-5">
            <Dialog.Title className="text-base font-semibold tracking-tight">
              {t.navigation}
            </Dialog.Title>
            <Dialog.Close aria-label={t.menu} className={iconButton}>
              <X className="size-5" aria-hidden />
            </Dialog.Close>
          </div>
          <div className="flex-1 overflow-y-auto overscroll-contain">
            <div className="mx-auto flex w-full max-w-3xl flex-col gap-6 px-6 pt-4 pb-[max(2rem,env(safe-area-inset-bottom))]">
              <nav aria-label={t.navigation} className="flex flex-col items-start">
                <SectionLinks links={links} current={current} onNavigate={close} />
              </nav>
              <nav aria-label={t.resources} className="border-t pt-5">
                <h2 className="mb-2 text-sm font-medium text-fd-muted-foreground">{t.resources}</h2>
                <div className="grid grid-cols-2 gap-x-3">
                  {resources.map(({ href, label, Icon }) => (
                    <Link
                      key={href}
                      href={href}
                      onClick={close}
                      className="inline-flex min-h-11 items-center gap-2 text-sm font-medium hover:underline"
                    >
                      <Icon aria-hidden className="size-4 shrink-0" />
                      {label}
                    </Link>
                  ))}
                  <a
                    href={github}
                    target="_blank"
                    rel="noreferrer"
                    className="inline-flex min-h-11 items-center gap-2 text-sm font-medium hover:underline"
                  >
                    <GitHubIcon /> GitHub{" "}
                    <ArrowUpRight aria-hidden className="size-3 text-fd-muted-foreground" />
                  </a>
                </div>
              </nav>
              <section aria-label={t.preferences} className="border-t pt-5">
                <h2 className="mb-3 text-sm font-medium text-fd-muted-foreground">
                  {t.preferences}
                </h2>
                <div className="flex flex-wrap gap-2">
                  <SitePreferences lang={lang} path={current} panel={panel} />
                </div>
              </section>
              {tree && (
                <div className="border-t pt-5">
                  <h2 className="mb-4 text-lg font-semibold tracking-tight">{t.docs}</h2>
                  <SidebarNav tree={tree} label={t.docs} onNavigate={close} />
                </div>
              )}
            </div>
          </div>
        </Dialog.Popup>
      </Dialog.Portal>
    </Dialog.Root>
  );
}

export function SiteHeader({ lang, tree }: { lang: string; tree?: PageTree.Root }) {
  const t = copyFor(lang);
  const { path } = useRouter();
  const { setOpenSearch } = useSearchContext();
  const current = path.replace(/\/$/, "");
  const links = [
    { href: `/${lang}/docs`, label: t.docs },
    { href: `/${lang}/docs/components`, label: t.components },
    { href: `/${lang}/blocks`, label: t.blocks },
    { href: `/${lang}/docs/recipes`, label: t.recipes },
    { href: `/${lang}/directory`, label: t.directory },
  ];
  return (
    <header className="sticky top-0 z-40 w-full bg-fd-background/95 backdrop-blur">
      <div className="mx-auto flex h-14 w-full max-w-[1480px] items-center gap-1 px-2 min-[360px]:gap-2 min-[360px]:px-3 sm:px-6 lg:h-16">
        <MobileNavigation key={current} lang={lang} links={links} current={current} tree={tree} />
        <Link
          href={`/${lang}`}
          className="inline-flex shrink-0 items-center gap-2 text-base font-semibold tracking-tight lg:mr-3"
        >
          <img src="/deck.svg" alt="" width={23} height={23} className="hidden min-[360px]:block" />
          {SITE_NAME}
        </Link>
        <nav aria-label={t.navigation} className="hidden items-center lg:flex">
          <SectionLinks links={links} current={current} />
        </nav>
        <div className="ml-auto flex items-center gap-1.5">
          <button
            type="button"
            aria-label={t.search}
            onClick={() => setOpenSearch(true)}
            className="inline-flex h-9 items-center justify-center gap-2 rounded-md px-2.5 text-fd-muted-foreground hover:bg-fd-accent xl:h-8 xl:w-56 xl:justify-between xl:bg-fd-secondary xl:text-xs"
          >
            <Search aria-hidden className="size-4 xl:hidden" />
            <span className="hidden truncate xl:block">{t.search}</span>
            <kbd className="hidden shrink-0 rounded border bg-fd-background px-1 font-mono text-[10px] xl:block">
              Ctrl K
            </kbd>
          </button>
          <div className="hidden items-center gap-0.5 border-s ps-2 lg:flex">
            <a
              href={github}
              target="_blank"
              rel="noreferrer"
              aria-label="GitHub"
              className={iconButton}
            >
              <GitHubIcon />
            </a>
            <SitePreferences lang={lang} path={current} />
          </div>
          <Link
            href={`/${lang}/docs/guides/blocks#publish`}
            className="inline-flex min-h-8 items-center justify-center gap-1.5 whitespace-nowrap rounded-md bg-fd-primary px-3 text-xs font-medium text-fd-primary-foreground hover:opacity-85"
          >
            <Plus aria-hidden className="size-3.5" />
            {t.publish}
          </Link>
        </div>
      </div>
    </header>
  );
}
