import { Link } from "fumapress/client";
import { ArrowRight, Download } from "lucide-react";

import { blockCopy } from "./block-copy";
import { InstallCommand } from "./install-command";

const labels = {
  en: {
    existing: "Add to an existing app",
    download: "Download project (.zip)",
    run: "Unzip the project and run this in the billing-dashboard folder.",
    add: "Run this in your GPUI project to install the dashboard, palette and recipes.",
    connect: "Connect the dashboard to your window",
  },
  ko: {
    existing: "기존 앱에 추가",
    download: "프로젝트 다운로드 (.zip)",
    run: "압축을 풀고 billing-dashboard 폴더에서 실행하세요.",
    add: "GPUI 프로젝트에서 실행하면 대시보드와 팔레트, 필요한 레시피가 설치됩니다.",
    connect: "내 앱의 창에 대시보드 연결하기",
  },
  ja: {
    existing: "既存のアプリに追加",
    download: "プロジェクトをダウンロード (.zip)",
    run: "展開した billing-dashboard フォルダーで実行します。",
    add: "GPUI プロジェクトで実行すると、ダッシュボードとパレット、必要なレシピが追加されます。",
    connect: "アプリのウィンドウにダッシュボードを表示する",
  },
  zh: {
    existing: "添加到现有应用",
    download: "下载项目 (.zip)",
    run: "解压后，在 billing-dashboard 文件夹中运行此命令。",
    add: "在 GPUI 项目中运行此命令，安装仪表盘、配色和所需的组件配方。",
    connect: "在应用窗口中显示仪表盘",
  },
};

export function BillingInstall({ lang, command }: { lang: string; command: string }) {
  const t = labels[lang as keyof typeof labels] ?? labels.en;
  return (
    <div className="mb-6 grid gap-3 lg:grid-cols-[minmax(0,2fr)_minmax(0,3fr)]">
      <section
        id="new-app"
        aria-labelledby="new-app-title"
        className="scroll-mt-20 rounded-xl border p-4"
      >
        <h2 id="new-app-title" className="font-semibold">
          {blockCopy(lang).newApp}
        </h2>
        <p className="mt-1 text-sm text-fd-muted-foreground">{t.run}</p>
        <div className="mt-3 flex flex-wrap items-center gap-x-4 gap-y-2">
          <a
            href="/downloads/billing-dashboard.zip"
            download
            className="inline-flex min-h-9 items-center gap-2 rounded-md bg-fd-primary px-3 text-sm font-medium text-fd-primary-foreground hover:opacity-85"
          >
            <Download className="size-4" aria-hidden />
            {t.download}
          </a>
          <code className="font-mono text-sm">cargo run</code>
        </div>
      </section>
      <section aria-labelledby="existing-app-title" className="min-w-0 rounded-xl border p-4">
        <h2 id="existing-app-title" className="font-semibold">
          {t.existing}
        </h2>
        <p className="mt-1 mb-3 text-sm text-fd-muted-foreground">{t.add}</p>
        <InstallCommand lang={lang} command={command} />
        <Link
          href={`/${lang}/docs/guides/blocks#existing-app`}
          className="mt-2 inline-flex min-h-8 items-center gap-1 text-sm underline underline-offset-4"
        >
          {t.connect}
          <ArrowRight aria-hidden className="size-3.5" />
        </Link>
      </section>
    </div>
  );
}
