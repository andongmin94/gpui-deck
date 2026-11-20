"use client";

import type * as PageTree from "fumadocs-core/page-tree";
import { Link, useRouter } from "fumapress/client";
import type { ReactNode } from "react";

function SectionLabel({ children }: { children: ReactNode }) {
  return (
    <p className="mt-6 mb-1 px-2 text-xs font-medium text-fd-muted-foreground first:mt-0">
      {children}
    </p>
  );
}

function Items({
  nodes,
  current,
  onNavigate,
}: {
  nodes: PageTree.Node[];
  current: string;
  onNavigate?: () => void;
}): ReactNode {
  return nodes.map((node, index) => {
    if (node.type === "separator") {
      return <SectionLabel key={`separator-${index}`}>{node.name}</SectionLabel>;
    }
    if (node.type === "folder") {
      return (
        <div key={`folder-${index}`} className="mt-6 flex flex-col">
          <p className="mb-1 px-2 text-xs font-medium text-fd-muted-foreground">{node.name}</p>
          <Items
            nodes={node.index ? [node.index, ...node.children] : node.children}
            current={current}
            onNavigate={onNavigate}
          />
        </div>
      );
    }
    const active = current === node.url;
    return (
      <Link
        key={node.url}
        href={node.url}
        onClick={onNavigate}
        aria-current={active ? "page" : undefined}
        className={`flex h-8 shrink-0 items-center rounded-md px-2 text-[0.8125rem] font-medium transition-colors ${
          active
            ? "bg-fd-accent text-fd-accent-foreground"
            : "text-fd-foreground/80 hover:bg-fd-accent/60 hover:text-fd-foreground"
        }`}
      >
        {node.name}
      </Link>
    );
  });
}

/** The page tree as shadcn/ui's sidebar: section labels over flat page links. */
export function SidebarNav({
  tree,
  label,
  onNavigate,
}: {
  tree: PageTree.Root;
  label: string;
  onNavigate?: () => void;
}) {
  const { path } = useRouter();
  return (
    <nav aria-label={label} className="flex flex-col">
      <Items nodes={tree.children} current={path.replace(/\/$/, "")} onNavigate={onNavigate} />
    </nav>
  );
}
