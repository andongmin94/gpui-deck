"use client";

import { useI18n } from "fumadocs-ui/contexts/i18n";
import { type ReactNode, useState } from "react";

import { useAccent } from "./use-accent";

const labels = {
  en: {
    preview: "Preview",
    code: "Code",
    note: "Rendered by GPUI’s web backend from the real component. Requires a browser with WebGPU.",
  },
  ko: {
    preview: "미리보기",
    code: "코드",
    note: "GPUI 웹 백엔드가 그리는 실제 컴포넌트입니다. WebGPU를 지원하는 브라우저가 필요합니다.",
  },
  ja: {
    preview: "プレビュー",
    code: "コード",
    note: "GPUI の Web バックエンドが描画する実際のコンポーネントです。WebGPU 対応のブラウザが必要です。",
  },
  zh: {
    preview: "预览",
    code: "代码",
    note: "由 GPUI 的 Web 后端渲染的真实组件。需要支持 WebGPU 的浏览器。",
  },
} as const;

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
  const { locale } = useI18n();
  const accent = useAccent();
  const t = labels[(locale ?? "en") as keyof typeof labels] ?? labels.en;
  const [tab, setTab] = useState<"preview" | "code">("preview");

  const tabs = children
    ? [
        { id: "preview" as const, label: t.preview },
        { id: "code" as const, label: t.code },
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
                  ? "border-fd-primary text-fd-primary"
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
          {/* The frame follows the page theme itself, so it starts before hydration. */}
          <iframe
            title={`${name} preview`}
            data-accent={accent}
            src={`/preview?c=${encodeURIComponent(name)}`}
            loading="lazy"
            className="block w-full"
            style={{ height }}
          />
        </div>
        <p className="mt-2 text-xs text-fd-muted-foreground">{t.note}</p>
      </div>
      <div className={tab === "code" ? "mt-4 [&_figure]:my-0" : "hidden"}>{children}</div>
    </div>
  );
}
