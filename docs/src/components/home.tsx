import { Link } from "fumapress/client";
import { ArrowRight } from "lucide-react";

import { InvoicePoster } from "./invoice-poster";
import { InvoiceShowcase } from "./invoice-showcase";
import { SiteFooter } from "./shell/footer";
import { SiteHeader } from "./shell/header";

const copy = {
  en: {
    title: "Build desktop apps with Rust and GPUI.",
    lead: "Accessible controls for native apps. Copy the Rust source and make the interface your own.",
    start: "Get Started",
    browse: "Browse Components",
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
    title: "Rust와 GPUI로 만드는 데스크톱 앱",
    lead: "키보드와 접근성을 지원하는 컴포넌트로 시작하세요. Rust 소스를 내 앱에 복사해 원하는 화면으로 구성할 수 있습니다.",
    start: "시작하기",
    browse: "컴포넌트 보기",
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
    title: "Rust と GPUI でデスクトップアプリを作る",
    lead: "キーボード操作に対応したネイティブ UI コンポーネント。Rust のソースを自分のアプリに取り込み、自由に画面を組み立てられます。",
    start: "はじめる",
    browse: "コンポーネントを見る",
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
    title: "用 Rust 和 GPUI 构建桌面应用",
    lead: "从支持键盘操作和无障碍功能的原生组件开始。将 Rust 源码复制到应用中，按需定制界面。",
    start: "开始使用",
    browse: "浏览组件",
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

export function Home({ lang }: { lang: string }) {
  const t = copy[lang as keyof typeof copy] ?? copy.en;
  return (
    <div className="flex min-h-svh flex-col">
      <SiteHeader lang={lang} />
      <main id="main-content" className="flex-1">
        <section className="mx-auto flex max-w-5xl flex-col items-center px-5 pt-10 pb-9 text-center sm:pt-14 sm:pb-12">
          <h1 className="text-4xl leading-[1.12] font-semibold tracking-tight text-balance sm:text-5xl lg:text-6xl [html:lang(ko)_&]:break-keep">
            {t.title}
          </h1>
          <p className="mt-4 max-w-2xl text-base leading-relaxed text-balance text-fd-muted-foreground sm:text-lg [html:lang(ko)_&]:break-keep">
            {t.lead}
          </p>
          <div className="mt-5 flex flex-wrap justify-center gap-2">
            <Link
              href={`/${lang}/docs`}
              className="inline-flex h-10 items-center gap-2 rounded-lg bg-fd-primary px-4 text-sm font-medium text-fd-primary-foreground hover:opacity-90"
            >
              {t.start}
              <ArrowRight className="size-4" aria-hidden />
            </Link>
            <Link
              href={`/${lang}/docs/components`}
              className="inline-flex h-10 items-center rounded-lg border px-4 text-sm font-medium hover:bg-fd-accent"
            >
              {t.browse}
            </Link>
          </div>
        </section>
        <InvoiceShowcase labels={t.demo}>
          <InvoicePoster lang={lang} />
        </InvoiceShowcase>
      </main>
      <SiteFooter lang={lang} />
    </div>
  );
}
