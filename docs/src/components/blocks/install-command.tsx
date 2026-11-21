"use client";

import { Check, Copy, Terminal } from "lucide-react";
import { useEffect, useRef, useState } from "react";

import { blockCopy } from "./block-copy";

export function InstallCommand({
  lang,
  command,
  compact = false,
}: {
  lang: string;
  command: string;
  compact?: boolean;
}) {
  const t = blockCopy(lang);
  const [status, setStatus] = useState<"idle" | "copied" | "manual">("idle");
  const field = useRef<HTMLInputElement>(null);
  useEffect(() => {
    if (status === "manual") {
      field.current?.focus();
      field.current?.select();
    }
  }, [status]);

  async function copyCommand() {
    try {
      await navigator.clipboard.writeText(command);
      setStatus("copied");
    } catch {
      setStatus("manual");
    }
  }

  return (
    <div className="min-w-0">
      <div
        className={
          compact
            ? "flex flex-wrap items-center gap-2"
            : "flex items-center gap-2 rounded-lg border bg-fd-background p-1.5 ps-3"
        }
      >
        {(!compact || status === "manual") && (
          <>
            <Terminal aria-hidden className="size-4 shrink-0 text-fd-muted-foreground" />
            <input
              ref={field}
              aria-label={t.command}
              value={command}
              readOnly
              onFocus={(event) => event.currentTarget.select()}
              className="min-w-12 flex-1 rounded bg-transparent py-1.5 font-mono text-xs"
            />
          </>
        )}
        <button
          type="button"
          aria-label={t.copy}
          title={t.copy}
          onClick={copyCommand}
          className="inline-flex min-h-9 shrink-0 items-center justify-center gap-2 rounded-md px-2.5 text-sm font-medium hover:bg-fd-secondary"
        >
          {status === "copied" ? (
            <Check aria-hidden className="size-4" />
          ) : (
            <Copy aria-hidden className="size-4" />
          )}
          {compact && (status === "copied" ? t.copied : t.copy)}
        </button>
      </div>
      <output
        className={status === "manual" ? "mt-1 block text-xs text-fd-muted-foreground" : "sr-only"}
      >
        {status === "copied" ? t.copied : status === "manual" ? t.manual : ""}
      </output>
    </div>
  );
}
