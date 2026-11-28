"use client";

import { useI18n } from "fumadocs-ui/contexts/i18n";
import { type CSSProperties, type ReactNode, useId, useState } from "react";

import { useAccent } from "./use-accent";
import { useHydrated } from "./use-hydrated";

const labels = {
  en: {
    preview: "Preview",
    code: "Code",
    note: "Rendered by GPUI’s web backend from the real component. Requires a browser with WebGPU.",
    run: "Run live preview",
    stop: "Back to snapshot",
    unavailable: "GPUI snapshot. Open in a browser with WebGPU to interact with this component.",
    select: "Open time-zone list showing Kolkata, Seoul and Tokyo, with Seoul selected.",
    dialog: "Edit profile dialog with name and username fields, Cancel and Save changes.",
    combobox: "Framework search for “s” with SvelteKit highlighted in the results.",
  },
  ko: {
    preview: "미리보기",
    code: "코드",
    note: "GPUI 웹 백엔드가 그리는 실제 컴포넌트입니다. WebGPU를 지원하는 브라우저가 필요합니다.",
    run: "직접 실행",
    stop: "미리보기로 돌아가기",
    unavailable:
      "GPUI 실행 화면입니다. 직접 조작하려면 WebGPU를 지원하는 브라우저에서 열어 주세요.",
    select: "시간대 목록에 콜카타·서울·도쿄가 표시되며 서울이 선택된 모습",
    dialog: "이름과 사용자 이름 입력란, 취소 및 저장 버튼이 있는 프로필 편집 대화상자",
    combobox: "프레임워크 검색란에 s를 입력해 결과 중 SvelteKit이 강조된 모습",
  },
  ja: {
    preview: "プレビュー",
    code: "コード",
    note: "GPUI の Web バックエンドが描画する実際のコンポーネントです。WebGPU 対応のブラウザが必要です。",
    run: "実行する",
    stop: "プレビューに戻る",
    unavailable: "GPUI の実行画面です。操作するには WebGPU 対応のブラウザで開いてください。",
    select: "コルカタ、ソウル、東京のタイムゾーン一覧でソウルが選択された状態",
    dialog: "名前とユーザー名の入力欄、キャンセルと保存ボタンのあるプロフィール編集ダイアログ",
    combobox: "フレームワーク検索欄に s を入力し、候補の SvelteKit が強調された状態",
  },
  zh: {
    preview: "预览",
    code: "代码",
    note: "由 GPUI 的 Web 后端渲染的真实组件。需要支持 WebGPU 的浏览器。",
    run: "运行实时预览",
    stop: "返回预览图",
    unavailable: "这是 GPUI 运行截图。请在支持 WebGPU 的浏览器中打开以操作此组件。",
    select: "时区列表中显示加尔各答、首尔和东京，当前选中首尔",
    dialog: "编辑个人资料对话框，包含姓名、用户名输入框和取消、保存按钮",
    combobox: "在框架搜索框输入 s，结果列表中高亮显示 SvelteKit",
  },
} as const;

// Widths of the cropped GPUI captures at their native display size.
const captureWidths = { select: 264, dialog: 392, combobox: 296 } as const;
type CapturedControl = keyof typeof captureWidths;

function CapturedPreview({
  name,
  height,
  accent,
  t,
  children,
}: {
  name: CapturedControl;
  height: number;
  accent: string;
  t: (typeof labels)[keyof typeof labels];
  children: ReactNode;
}) {
  const [mode, setMode] = useState<"snapshot" | "checking" | "live" | "unavailable">("snapshot");
  const hydrated = useHydrated();
  const id = useId();
  const file = `/images/components/${name}-${accent}`;
  async function toggleLive() {
    if (mode === "live") {
      setMode("snapshot");
      return;
    }
    setMode("checking");
    const adapter = await navigator.gpu.requestAdapter().catch(() => null);
    // GPUI's web backend uses WebGL2 when there is no hardware WebGPU adapter.
    if (!adapter || adapter.info?.isFallbackAdapter) {
      const gl = document.createElement("canvas").getContext("webgl2");
      if (!gl) {
        setMode("unavailable");
        return;
      }
      gl.getExtension("WEBGL_lose_context")?.loseContext();
    }
    setMode("live");
  }
  return (
    <>
      <div id={id} className="overflow-hidden rounded-lg border bg-fd-background">
        {mode === "live" ? (
          children
        ) : (
          <div
            style={{ height }}
            className={`flex justify-center ${name === "dialog" ? "bg-[#808080] dark:bg-[#050505]" : "bg-white dark:bg-[#0a0a0a]"}`}
          >
            <div
              // oxlint-disable-next-line jsx-a11y/prefer-tag-over-role -- CSS selects only the capture for the active site color mode.
              role="img"
              aria-label={t[name]}
              className="h-full max-w-full bg-contain bg-center bg-no-repeat [background-image:var(--capture-light)] dark:[background-image:var(--capture-dark)]"
              style={
                {
                  width: captureWidths[name],
                  "--capture-light": `url("${file}-light.webp")`,
                  "--capture-dark": `url("${file}-dark.webp")`,
                } as CSSProperties
              }
            />
          </div>
        )}
      </div>
      <div className="mt-2 flex min-h-9 items-center text-sm">
        {mode === "unavailable" || (hydrated && !navigator.gpu) ? (
          <p className="text-xs text-fd-muted-foreground">{t.unavailable}</p>
        ) : (
          <button
            type="button"
            disabled={!hydrated || mode === "checking"}
            aria-busy={mode === "checking"}
            aria-controls={id}
            className="min-h-9 rounded-md border px-3 font-medium hover:bg-fd-secondary focus-visible:outline-offset-2 disabled:opacity-50"
            onClick={toggleLive}
          >
            {mode === "live" ? t.stop : t.run}
          </button>
        )}
      </div>
    </>
  );
}

/**
 * A live GPUI Deck component rendered by GPUI's web backend inside an
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
  const frame = (
    <iframe
      title={`${name} preview`}
      data-accent={accent}
      src={`${import.meta.env.DEV ? "/preview/index.html" : "/preview"}?c=${encodeURIComponent(name)}`}
      loading="lazy"
      className="block w-full"
      style={{ height }}
    />
  );

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
        {name === "select" || name === "dialog" || name === "combobox" ? (
          <CapturedPreview key={name} name={name} height={height} accent={accent} t={t}>
            {frame}
          </CapturedPreview>
        ) : (
          <>
            <div className="overflow-hidden rounded-lg border bg-fd-background">{frame}</div>
            <p className="mt-2 text-xs text-fd-muted-foreground">{t.note}</p>
          </>
        )}
      </div>
      <div className={tab === "code" ? "mt-4 [&_figure]:my-0" : "hidden"}>{children}</div>
    </div>
  );
}
