"use client";

import type { TOCItemType } from "fumadocs-core/toc";
import { useEffect, useState } from "react";

/** shadcn/ui's "On This Page" list, highlighting the heading in view. */
export function TableOfContents({ toc, title }: { toc: TOCItemType[]; title: string }) {
  const [active, setActive] = useState<string>();

  useEffect(() => {
    const headings = toc
      .map((item) => document.getElementById(item.url.slice(1)))
      .filter((element): element is HTMLElement => element !== null);
    const observer = new IntersectionObserver(
      (entries) => {
        const visible = entries.find((entry) => entry.isIntersecting);
        if (visible) setActive(`#${visible.target.id}`);
      },
      { rootMargin: "0% 0% -80% 0%" },
    );
    for (const heading of headings) observer.observe(heading);
    return () => observer.disconnect();
  }, [toc]);

  if (toc.length === 0) return null;
  return (
    <nav aria-label={title} className="flex flex-col gap-2 text-[0.8125rem]">
      <p className="text-xs font-medium text-fd-muted-foreground">{title}</p>
      {toc.map((item) => (
        <a
          key={item.url}
          href={item.url}
          aria-current={active === item.url ? "location" : undefined}
          className={`transition-colors hover:text-fd-foreground ${
            item.depth > 2 ? "pl-4" : ""
          } ${active === item.url ? "text-fd-foreground" : "text-fd-muted-foreground"}`}
        >
          {item.title}
        </a>
      ))}
    </nav>
  );
}
