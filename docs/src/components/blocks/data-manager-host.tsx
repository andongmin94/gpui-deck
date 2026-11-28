import { CodeBlock, Pre } from "fumadocs-ui/components/codeblock";
import { Download } from "lucide-react";

const labels = {
  en: {
    title: "Connect your own records",
    run: "In the downloaded project, run the team workspace. Edit or delete a record, then reopen the app in the same folder. The host saves changes in team-records.json.",
    source: "Download the host’s Rust source",
    inspect: "View host source on GitHub",
    embed:
      "In an existing app, install the block above and save the host source as src/team_records.rs. Add its JSON dependencies and mount it once in the parent view.",
    render:
      "Keep screen in a gpui::AnyView field and render it with .child(self.screen.clone()). In the host source, on_save and on_delete are where you connect your own storage.",
  },
  ko: {
    title: "내 앱의 레코드 연결하기",
    run: "내려받은 프로젝트에서 팀 작업 공간을 실행하세요. 레코드를 수정하거나 삭제한 뒤 같은 폴더에서 앱을 다시 열어 보세요. 변경한 내용은 team-records.json에 저장됩니다.",
    source: "호스트 Rust 소스 다운로드",
    inspect: "GitHub에서 호스트 소스 보기",
    embed:
      "기존 앱에서는 위 명령으로 블록을 설치하고 호스트 소스를 src/team_records.rs로 저장하세요. JSON 의존성을 추가한 뒤 부모 뷰를 만들 때 화면을 한 번 연결하세요.",
    render:
      "screen을 gpui::AnyView 필드에 보관하고 .child(self.screen.clone())으로 표시하세요. 호스트 소스의 on_save와 on_delete에서 앱의 저장소를 연결할 수 있습니다.",
  },
  ja: {
    title: "自分のアプリのレコードを接続する",
    run: "ダウンロードしたプロジェクトでチーム用画面を起動します。レコードを編集または削除し、同じフォルダーでアプリを開き直してください。変更は team-records.json に保存されます。",
    source: "ホストの Rust ソースをダウンロード",
    inspect: "GitHub でホストのソースを表示",
    embed:
      "既存のアプリでは上のコマンドでブロックを追加し、ホストのソースを src/team_records.rs に保存します。JSON の依存関係を追加し、親ビューの生成時に画面を一度作成します。",
    render:
      "screen は gpui::AnyView フィールドに保持し、.child(self.screen.clone()) で表示します。ホストの on_save と on_delete でアプリのストレージを接続できます。",
  },
  zh: {
    title: "连接应用自己的记录",
    run: "在下载的项目中运行团队工作区。编辑或删除记录，然后在同一目录重新打开应用。更改保存在 team-records.json 中。",
    source: "下载宿主应用的 Rust 源码",
    inspect: "在 GitHub 查看宿主源码",
    embed:
      "在现有应用中，先用上方命令安装区块，将宿主源码保存为 src/team_records.rs，然后添加 JSON 依赖。在创建父视图时挂载一次界面。",
    render:
      "将 screen 保存在 gpui::AnyView 字段中，并用 .child(self.screen.clone()) 显示。可在宿主源码的 on_save 和 on_delete 中连接自己的存储。",
  },
};

export function DataManagerHostRun({ lang }: { lang: string }) {
  const t = labels[lang as keyof typeof labels] ?? labels.en;
  return (
    <div className="mt-4 border-t pt-2">
      <p className="mt-1 mb-3 text-sm text-fd-muted-foreground">{t.run}</p>
      <CodeBlock title="Cargo" className="my-2">
        <Pre>
          <code>cargo run --bin team-records</code>
        </Pre>
      </CodeBlock>
    </div>
  );
}

export function DataManagerHost({ lang }: { lang: string }) {
  const t = labels[lang as keyof typeof labels] ?? labels.en;
  return (
    <details className="mt-2 text-sm">
      <summary className="w-fit cursor-pointer py-1.5 font-medium underline underline-offset-4">
        {t.title}
      </summary>
      <p className="mt-1 text-fd-muted-foreground">{t.embed}</p>
      <a
        href="/downloads/data-manager-host.rs"
        download="team_records.rs"
        className="inline-flex min-h-9 items-center gap-2 text-sm font-medium underline underline-offset-4"
      >
        <Download aria-hidden className="size-4" />
        {t.source}
      </a>
      <a
        href="https://github.com/andongmin94/gpui-deck/blob/main/blocks/starter/data-manager-host.rs"
        className="ms-3 inline-flex min-h-9 items-center text-sm underline underline-offset-4"
      >
        {t.inspect}
      </a>
      <CodeBlock title="Cargo" className="my-2">
        <Pre>
          <code>{"cargo add serde --features derive\ncargo add serde_json"}</code>
        </Pre>
      </CodeBlock>
      <CodeBlock title="Rust" className="my-2">
        <Pre>
          <code>
            {
              "// src/main.rs\nmod ui;\nmod team_records;\n\n// Parent::new(window, cx)\nlet screen = crate::team_records::mount(window, cx);"
            }
          </code>
        </Pre>
      </CodeBlock>
      <p className="text-sm text-fd-muted-foreground">{t.render}</p>
    </details>
  );
}
