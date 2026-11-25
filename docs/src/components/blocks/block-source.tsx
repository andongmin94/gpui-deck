import { ArrowUpRight, FileCode2 } from "lucide-react";
import { lazy, Suspense, useState } from "react";

import { type PreviewItem, blockCopy } from "./block-copy";

const sources = import.meta.glob<string>("./{blocks,registries}/**/*.rs", {
  base: "../../../..",
  import: "default",
  eager: true,
  query: "?url&no-inline",
});

// Only fetch the installed Rust file selected in the viewer.
const sourceFiles = Object.fromEntries(
  Object.entries(sources).map(([path, url]) => [
    path,
    lazy(async () => {
      const response = await fetch(url);
      if (!response.ok) throw new Error(`Failed to load ${path} (${response.status})`);
      const source = await response.text();
      return { default: () => <code>{source}</code> };
    }),
  ]),
);

export default function BlockSource({ lang, block }: { lang: string; block: PreviewItem }) {
  const t = blockCopy(lang);
  const [selected, setSelected] = useState(block.files[0]);
  const SourceFile = sourceFiles[`./${block.sourcePath}/${selected.source}`];
  return (
    <div className="min-w-0">
      <div className="flex flex-wrap items-center justify-between gap-2 border-b px-3 py-2.5 sm:px-5">
        <label className="inline-flex items-center gap-2 text-sm">
          <FileCode2 aria-hidden className="size-4 text-fd-muted-foreground" />
          <span className="sr-only">{t.file}</span>
          <select
            value={selected.source}
            onChange={(event) =>
              setSelected(block.files.find((file) => file.source === event.target.value)!)
            }
            className="max-w-full rounded-md border bg-fd-background px-2 py-1.5 font-mono text-xs"
          >
            {block.files.map((file) => (
              <option key={file.source} value={file.source}>
                {file.target}
              </option>
            ))}
          </select>
        </label>
        <a
          href={`${block.source}/${selected.source}`}
          className="inline-flex items-center gap-1 rounded text-xs font-medium text-fd-muted-foreground hover:text-fd-foreground"
        >
          {t.github}
          <ArrowUpRight aria-hidden className="size-3.5" />
        </a>
      </div>
      <pre
        dir="ltr"
        aria-label={selected.target}
        // oxlint-disable-next-line jsx-a11y/no-noninteractive-tabindex -- The bounded source viewport needs keyboard scrolling in both directions.
        tabIndex={0}
        className="max-h-[70svh] min-h-56 overflow-auto overscroll-contain bg-fd-secondary/40 p-4 font-mono text-xs leading-6 text-fd-foreground sm:p-6 sm:text-[0.8125rem]"
      >
        <Suspense fallback={<span>{t.loading}</span>}>
          <SourceFile />
        </Suspense>
      </pre>
    </div>
  );
}
