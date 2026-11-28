import { Link } from "fumapress/client";
import { CodeBlock, Pre } from "fumadocs-ui/components/codeblock";
import { ArrowRight, Download } from "lucide-react";

import { type Block, blockCopy } from "./block-copy";
import { InstallCommand } from "./install-command";

const labels = {
  en: {
    existing: "Add to an existing app",
    run: (folder: string) => `Unzip the project and run this in the ${folder} folder.`,
    add: "Run this in your GPUI project to install the screen and its dependencies.",
    connect: "Connect this screen to your window",
    initialize:
      "Declare mod ui; in the crate root. Create the screen once in your parent view’s constructor.",
    render: "Store screen in a gpui::AnyView field and render it with .child(self.screen.clone()).",
  },
  ko: {
    existing: "기존 앱에 추가",
    run: (folder: string) => `압축을 풀고 ${folder} 폴더에서 실행하세요.`,
    add: "GPUI 프로젝트에서 실행하면 화면과 필요한 소스가 함께 설치됩니다.",
    connect: "내 앱의 창에 이 화면 연결하기",
    initialize: "크레이트 루트에 mod ui;를 선언하고 부모 뷰의 생성자에서 화면을 한 번 생성하세요.",
    render: "screen을 gpui::AnyView 필드로 보관하고 .child(self.screen.clone())으로 표시하세요.",
  },
  ja: {
    existing: "既存のアプリに追加",
    run: (folder: string) => `展開した ${folder} フォルダーで実行します。`,
    add: "GPUI プロジェクトで実行すると、画面と必要なソースが追加されます。",
    connect: "アプリのウィンドウにこの画面を表示する",
    initialize:
      "クレートルートに mod ui; を宣言し、親ビューのコンストラクターで画面を一度生成します。",
    render:
      "screen を gpui::AnyView 型のフィールドに保持し、.child(self.screen.clone()) で表示します。",
  },
  zh: {
    existing: "添加到现有应用",
    run: (folder: string) => `解压后，在 ${folder} 文件夹中运行此命令。`,
    add: "在 GPUI 项目中运行此命令，安装界面和所需的源文件。",
    connect: "在应用窗口中显示此界面",
    initialize: "在 crate 根模块声明 mod ui;，然后在父视图的构造函数中创建一次界面。",
    render: "将 screen 保存在 gpui::AnyView 类型的字段中，再用 .child(self.screen.clone()) 渲染。",
  },
};

export function BlockInstall({
  lang,
  block,
}: {
  lang: string;
  block: Pick<Block, "name" | "command"> & { view?: string };
}) {
  const t = labels[lang as keyof typeof labels] ?? labels.en;
  const guideAnchor = block.name === "billing-dashboard" ? "existing-app" : block.name;
  const module = `crate::ui::${block.name.replaceAll("-", "_")}`;
  const mount =
    block.name === "neobrutal-login"
      ? `use gpui::AppContext as _;\nlet screen: gpui::AnyView = cx.new(|cx| ${module}::Login::new(window, cx)).into();`
      : `let screen = ${module}::preview(window, cx);`;
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
        <p className="mt-1 text-sm text-fd-muted-foreground">{t.run(block.name)}</p>
        <div className="mt-3 flex flex-wrap items-center gap-x-4 gap-y-2">
          <a
            href={`/downloads/${block.name}.zip`}
            download
            className="inline-flex min-h-9 items-center gap-2 rounded-md bg-fd-primary px-3 text-sm font-medium text-fd-primary-foreground hover:opacity-85"
          >
            <Download className="size-4" aria-hidden />
            {blockCopy(lang).download}
          </a>
          <code className="font-mono text-sm">cargo run</code>
        </div>
      </section>
      <section aria-labelledby="existing-app-title" className="min-w-0 rounded-xl border p-4">
        <h2 id="existing-app-title" className="font-semibold">
          {t.existing}
        </h2>
        <p className="mt-1 mb-3 text-sm text-fd-muted-foreground">{t.add}</p>
        <InstallCommand key={block.name} lang={lang} command={block.command} />
        {block.view ? (
          <>
            <p className="mt-2 text-xs text-fd-muted-foreground">
              {blockCopy(lang).view} <code className="break-all font-mono">{block.view}</code>
            </p>
            <Link
              href={`/${lang}/docs/guides/blocks#${guideAnchor}`}
              className="mt-2 inline-flex min-h-8 items-center gap-1 text-sm underline underline-offset-4"
            >
              {t.connect}
              <ArrowRight aria-hidden className="size-3.5" />
            </Link>
          </>
        ) : (
          <details className="mt-2 text-sm">
            <summary className="w-fit cursor-pointer py-1.5 font-medium underline underline-offset-4">
              {t.connect}
            </summary>
            <p className="mt-1 text-fd-muted-foreground">{t.initialize}</p>
            <CodeBlock title="Rust" className="my-2">
              <Pre>
                <code>{mount}</code>
              </Pre>
            </CodeBlock>
            <p className="text-fd-muted-foreground">{t.render}</p>
            <Link
              href={`/${lang}/docs/guides/blocks#registry-blocks`}
              className="mt-2 inline-flex items-center gap-1 underline underline-offset-4"
            >
              {blockCopy(lang).setup}
              <ArrowRight aria-hidden className="size-3.5" />
            </Link>
          </details>
        )}
      </section>
    </div>
  );
}
