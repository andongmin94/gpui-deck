import { Link } from "fumapress/client";
import {
  ArrowRight,
  ArrowUpRight,
  ChartColumn,
  ChevronRight,
  FileStack,
  Table2,
} from "lucide-react";

import { billing } from "./blocks/block-copy";
import { HomePalette } from "./home-palette";
import { InvoicePoster } from "./invoice-poster";
import { InvoiceShowcase } from "./invoice-showcase";
import { SiteFooter } from "./shell/footer";
import { SiteHeader } from "./shell/header";

const copy = {
  en: {
    title: "Build your design system with GPUI.",
    lead: "Accessible components for native Rust apps. Bring your own palette, compose a block, and share its source with a single link.",
    feature: "Share a block from GitHub",
    start: "Get Started",
    browse: "View Components",
    appearance: "Appearance",
    palette: "Choose an accent for the previews.",
    billing: "Billing dashboard",
    included: "Featured components",
    install: "Install this block",
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
    feature: "GitHub에서 블록 공유하기",
    start: "시작하기",
    browse: "컴포넌트 보기",
    appearance: "화면 스타일",
    palette: "미리보기에 적용할 강조색을 고르세요.",
    billing: "청구 대시보드",
    included: "주요 컴포넌트",
    install: "이 블록 설치하기",
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
    feature: "GitHub からブロックを共有",
    start: "はじめる",
    browse: "コンポーネントを見る",
    appearance: "外観",
    palette: "プレビューのアクセントカラーを選択します。",
    billing: "請求ダッシュボード",
    included: "主なコンポーネント",
    install: "このブロックをインストール",
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
    feature: "从 GitHub 分享区块",
    start: "开始使用",
    browse: "浏览组件",
    appearance: "外观",
    palette: "选择预览使用的强调色。",
    billing: "账单仪表盘",
    included: "主要组件",
    install: "安装此区块",
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

const blockComponents = [
  { name: "Chart", slug: "components/chart", icon: ChartColumn },
  { name: "DataTable", slug: "recipes/data-table", icon: Table2 },
];

export function Home({ lang }: { lang: string }) {
  const t = copy[lang as keyof typeof copy] ?? copy.en;
  const blocksGuide = `/${lang}/docs/guides/blocks`;
  return (
    <div className="flex min-h-svh flex-col">
      <SiteHeader lang={lang} />
      <main id="main-content" className="flex-1">
        <section className="mx-auto flex max-w-6xl flex-col items-center px-4 pt-8 pb-10 text-center sm:pt-16 sm:pb-14 lg:pt-20 lg:pb-[4.5rem]">
          <Link
            href={blocksGuide}
            className="inline-flex items-center gap-1.5 rounded-full bg-fd-secondary px-2.5 py-1 text-xs leading-none font-medium transition-colors hover:bg-fd-accent"
          >
            {t.feature}
            <ArrowRight className="size-3" aria-hidden />
          </Link>
          <h1 className="mt-3 max-w-full text-[2rem] leading-[1.08] font-semibold tracking-[-0.048em] text-balance sm:mt-4 sm:text-5xl [html:lang(ko)_&]:break-keep">
            {t.title}
          </h1>
          <p className="mt-3 max-w-[39rem] text-base leading-6 text-pretty sm:mt-4 sm:text-lg sm:leading-7 [html:lang(ko)_&]:break-keep">
            {t.lead}
          </p>
          <div className="mt-5 flex flex-wrap justify-center gap-2 sm:mt-6">
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
        <div className="bg-linear-to-b from-fd-background to-fd-secondary/50">
          <div className="mx-auto grid max-w-[1480px] items-start gap-4 px-3 pb-10 sm:gap-5 sm:px-6 lg:grid-cols-[minmax(16rem,1fr)_minmax(0,2.6fr)] lg:pb-16">
            <div className="grid grid-cols-2 gap-2 sm:gap-5 lg:col-start-1 lg:row-start-1 lg:grid-cols-1">
              <HomePalette lang={lang} title={t.appearance} description={t.palette} />
              <section className="rounded-2xl border bg-fd-background p-3.5 shadow-xs sm:rounded-3xl sm:p-6">
                <div className="mb-2 flex items-center justify-between sm:mb-4">
                  <span className="hidden size-9 items-center justify-center rounded-xl bg-fd-secondary sm:flex">
                    <FileStack className="size-[18px]" aria-hidden />
                  </span>
                  <a
                    href={`${billing.source}/block.json`}
                    className="inline-flex min-h-6 items-center gap-1 rounded text-xs text-fd-muted-foreground hover:text-fd-foreground hover:underline"
                  >
                    <code>block.json</code>
                    <ArrowUpRight className="size-3" aria-hidden />
                  </a>
                </div>
                <h2 className="text-base leading-5 font-semibold tracking-tight sm:text-lg sm:leading-6">
                  {t.billing}
                </h2>
                <p className="mt-1 hidden text-sm text-fd-muted-foreground sm:block">
                  {t.included}
                </p>
                <ul className="my-2 sm:my-4">
                  {blockComponents.map(({ name, slug, icon: Icon }) => (
                    <li key={slug}>
                      <Link
                        href={`/${lang}/docs/${slug}`}
                        className="flex min-h-7 items-center gap-2 rounded-lg text-xs font-medium hover:bg-fd-secondary sm:min-h-9 sm:px-1.5 sm:text-sm"
                      >
                        <Icon className="size-4 text-fd-muted-foreground" aria-hidden />
                        {name}
                        <ChevronRight
                          className="ml-auto size-3.5 text-fd-muted-foreground"
                          aria-hidden
                        />
                      </Link>
                    </li>
                  ))}
                </ul>
                <Link
                  href={`/${lang}/blocks/${billing.name}`}
                  className="flex min-h-8 items-center justify-center gap-1.5 rounded-full bg-fd-secondary px-2 py-1 text-center text-xs font-medium hover:bg-fd-accent sm:min-h-9 sm:text-sm"
                >
                  {t.install}
                  <ArrowRight className="size-3.5" aria-hidden />
                </Link>
              </section>
            </div>
            <InvoiceShowcase labels={t.demo}>
              <InvoicePoster lang={lang} />
            </InvoiceShowcase>
          </div>
        </div>
      </main>
      <SiteFooter lang={lang} />
    </div>
  );
}
