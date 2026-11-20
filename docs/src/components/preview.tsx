"use client";

import { useI18n } from "fumadocs-ui/contexts/i18n";
import { useTheme } from "next-themes";
import { type ReactNode, useState, useSyncExternalStore } from "react";

const noSubscription = () => () => {};

/**
 * A live GPUI Design component rendered by GPUI's web backend inside an
 * isolated frame, with its source in a Code tab when one is given.
 */
export function ComponentPreview({
  name,
  height = 280,
  children,
}: {
  name: string;
  height?: number;
  children?: ReactNode;
}) {
  const { resolvedTheme } = useTheme();
  const { locale } = useI18n();
  const ko = locale === "ko";
  const [tab, setTab] = useState<"preview" | "code">("preview");
  // The theme is known only in the browser, so the frame waits for hydration.
  const hydrated = useSyncExternalStore(
    noSubscription,
    () => true,
    () => false,
  );
  const theme = hydrated ? (resolvedTheme === "dark" ? "dark" : "light") : undefined;

  const tabs = children
    ? [
        { id: "preview" as const, label: ko ? "미리보기" : "Preview" },
        { id: "code" as const, label: ko ? "코드" : "Code" },
      ]
    : [];

  return (
    <div className="not-prose my-6">
      {tabs.length > 0 && (
        <div role="tablist" className="flex gap-4 border-b">
          {tabs.map((item) => (
            <button
              key={item.id}
              type="button"
              role="tab"
              aria-selected={tab === item.id}
              onClick={() => setTab(item.id)}
              className={`-mb-px border-b-2 px-1 pb-2 text-sm font-medium transition-colors ${
                tab === item.id
                  ? "border-fd-foreground text-fd-foreground"
                  : "border-transparent text-fd-muted-foreground hover:text-fd-foreground"
              }`}
            >
              {item.label}
            </button>
          ))}
        </div>
      )}
      <div className={tab === "preview" ? (tabs.length > 0 ? "mt-4" : "") : "hidden"}>
        <div className="overflow-hidden rounded-lg border bg-fd-background">
          {theme ? (
            <iframe
              key={theme}
              title={`${name} preview`}
              src={`/preview/?c=${encodeURIComponent(name)}&theme=${theme}`}
              loading="lazy"
              className="block w-full"
              style={{ height }}
            />
          ) : (
            <div style={{ height }} />
          )}
        </div>
        <p className="mt-2 text-xs text-fd-muted-foreground">
          {ko
            ? "GPUI 웹 백엔드가 그리는 실제 컴포넌트입니다. WebGPU를 지원하는 브라우저가 필요합니다."
            : "Rendered by GPUI’s web backend from the real component. Requires a browser with WebGPU."}
        </p>
      </div>
      <div className={tab === "code" ? "mt-4 [&_figure]:my-0" : "hidden"}>{children}</div>
    </div>
  );
}
