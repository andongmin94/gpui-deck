import { Link } from "fumapress/client";
import { ArrowRight } from "lucide-react";

import type { Block } from "./blocks/block-copy";
import { InvoicePoster } from "./invoice-poster";
import { InvoiceShowcase } from "./invoice-showcase";
import { SiteFooter } from "./shell/footer";
import { SiteHeader } from "./shell/header";

const copy = {
  en: {
    title: "Build your design system with GPUI.",
    lead: "Accessible components for native Rust apps. Bring your own palette, compose a block, and share its source with a single link.",
    start: "Get Started",
    browse: "View Components",
    install: "Install block",
    demo: {
      title: "Invoice workspace",
      description: "Mark an invoice as paid to update the balance and monthly collections.",
      run: "Run demo",
      stop: "Back to preview",
      source: "View Rust source",
      unavailable: "To run this demo, open it in a browser with WebGPU support.",
    },
  },
  ko: {
    title: "GPUI로 만드는 나만의 디자인 시스템",
    lead: "네이티브 Rust 앱을 위한 접근성 컴포넌트에 팀의 팔레트를 입히세요. 화면을 블록으로 조합하고 링크 하나로 소스를 공유할 수 있습니다.",
    start: "시작하기",
    browse: "컴포넌트 보기",
    install: "블록 설치",
    demo: {
      title: "청구서 관리",
      description: "청구서를 결제 완료로 바꾸면 미수금과 월별 수금 차트가 함께 바뀝니다.",
      run: "직접 실행",
      stop: "미리보기로 돌아가기",
      source: "Rust 소스 보기",
      unavailable: "직접 실행하려면 WebGPU를 지원하는 브라우저에서 열어 주세요.",
    },
  },
  ja: {
    title: "GPUI でつくる、チームのデザインシステム",
    lead: "Rust のネイティブアプリ向けのアクセシブルなコンポーネント。独自の配色でブロックを組み立て、リンクひとつでソースを共有できます。",
    start: "はじめる",
    browse: "コンポーネントを見る",
    install: "ブロックを追加",
    demo: {
      title: "請求書管理",
      description: "請求書を入金済みにすると、未収金残高と月別の入金グラフが更新されます。",
      run: "デモを実行",
      stop: "プレビューに戻る",
      source: "Rust のソースを見る",
      unavailable: "デモを実行するには、WebGPU に対応したブラウザで開いてください。",
    },
  },
  zh: {
    title: "用 GPUI 构建自己的设计系统",
    lead: "为原生 Rust 应用打造无障碍界面。用自己的配色组合界面区块，通过一个链接共享源码。",
    start: "开始使用",
    browse: "浏览组件",
    install: "安装区块",
    demo: {
      title: "发票管理",
      description: "将发票标记为已付款，应收余额和月度收款图表会随之更新。",
      run: "运行演示",
      stop: "返回预览",
      source: "查看 Rust 源码",
      unavailable: "请在支持 WebGPU 的浏览器中打开以运行此演示。",
    },
  },
} as const;

type HomeBlock = { block: Block; title: string };

const detailThumbnails: Record<string, { suffix: string; width: number; height: number }> = {
  login: { suffix: "-mobile.webp", width: 390, height: 844 },
  "sidebar-settings": { suffix: "-notifications.webp", width: 390, height: 449 },
};

function BlockTile({ entry, lang, install }: { entry: HomeBlock; lang: string; install: string }) {
  const { block, title } = entry;
  const billing = block.name === "billing-dashboard";
  const image = detailThumbnails[block.name] ?? { suffix: ".webp", width: 1440, height: 820 };

  return (
    <Link
      href={`/${lang}/blocks/${block.name}`}
      aria-label={title}
      className={`group flex min-w-0 flex-col overflow-hidden rounded-2xl border bg-fd-background focus-visible:outline-offset-2 sm:rounded-3xl ${billing ? "sm:col-span-2 sm:flex-row-reverse" : ""}`}
    >
      <div
        className={`relative overflow-hidden border-b bg-fd-secondary ${billing ? "aspect-[2.7] sm:w-3/5 sm:border-s sm:border-b-0" : "aspect-[390/475]"}`}
      >
        <img
          src={block.poster + image.suffix}
          alt=""
          loading="lazy"
          width={image.width}
          height={image.height}
          className={
            billing
              ? "absolute top-0 right-0 w-[200%] max-w-none -translate-y-[30%]"
              : "block size-full object-cover"
          }
        />
      </div>
      <div
        className={`flex flex-1 items-center justify-between gap-2 px-4 py-3.5 sm:px-5 ${billing ? "sm:flex-col sm:items-start sm:justify-center" : "lg:max-xl:flex-col lg:max-xl:items-start lg:max-xl:justify-center"}`}
      >
        <h2 className="text-base leading-tight font-semibold tracking-tight group-hover:underline">
          {title}
        </h2>
        <span className="inline-flex shrink-0 items-center gap-1 text-xs text-fd-muted-foreground group-hover:text-fd-foreground">
          {install}
          <ArrowRight aria-hidden className="size-3.5" />
        </span>
      </div>
    </Link>
  );
}

export function Home({ lang, entries }: { lang: string; entries: HomeBlock[] }) {
  const t = copy[lang as keyof typeof copy] ?? copy.en;
  return (
    <div className="flex min-h-svh flex-col">
      <SiteHeader lang={lang} />
      <main id="main-content" className="flex-1">
        <section className="mx-auto flex max-w-6xl flex-col items-center px-4 py-8 text-center sm:py-12 lg:py-16">
          <h1 className="max-w-full text-[2rem] leading-[1.08] font-semibold tracking-[-0.048em] text-balance sm:text-5xl [html:lang(ko)_&]:break-keep">
            {t.title}
          </h1>
          <p className="mt-3 max-w-[39rem] text-base leading-6 text-pretty sm:text-lg sm:leading-7 [html:lang(ko)_&]:break-keep">
            {t.lead}
          </p>
          <div className="mt-5 flex flex-wrap justify-center gap-2">
            <Link
              href={`/${lang}/docs`}
              className="inline-flex min-h-9 items-center rounded-full bg-fd-primary px-4 text-sm font-medium text-fd-primary-foreground transition-opacity hover:opacity-85"
            >
              {t.start}
            </Link>
            <Link
              href={`/${lang}/docs/components`}
              className="inline-flex min-h-9 items-center rounded-full bg-fd-secondary px-4 text-sm font-medium transition-colors hover:bg-fd-accent"
            >
              {t.browse}
            </Link>
          </div>
        </section>
        <div className="mx-auto grid max-w-[1480px] items-start gap-3 px-3 pb-10 sm:gap-4 sm:px-6 lg:grid-cols-[minmax(0,1.16fr)_minmax(0,1fr)] lg:pb-16">
          <InvoiceShowcase labels={t.demo}>
            <InvoicePoster lang={lang} />
          </InvoiceShowcase>
          <div className="grid min-w-0 gap-3 sm:grid-cols-2 sm:gap-4">
            {entries.map((entry) => (
              <BlockTile key={entry.block.name} entry={entry} lang={lang} install={t.install} />
            ))}
          </div>
        </div>
      </main>
      <SiteFooter lang={lang} />
    </div>
  );
}
