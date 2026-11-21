import manifest from "../../../../blocks/billing-dashboard/block.json";
import { GIT } from "@/lib/site";

export const billing = {
  name: manifest.name,
  files: manifest.files,
  source: `https://github.com/${GIT.user}/${GIT.repo}/blob/${GIT.branch}/blocks/${manifest.name}`,
  poster: `/images/blocks/${manifest.name}`,
};
export const installCommand = `gpui-deck add ${billing.source}/block.json`;

const copy = {
  en: {
    all: "All blocks",
    share: "Publish a block",
    open: "View block",
    preview: "Preview",
    source: "Rust source",
    file: "Source file",
    github: "View on GitHub",
    run: "Run preview",
    stop: "Stop preview",
    unavailable:
      "The interactive preview requires WebGPU. The image below shows the installed dashboard.",
    command: "Install command",
    copy: "Copy install command",
    copied: "Copied",
    manual: "Copy the selected command.",
    setup: "Installation guide",
    loading: "Loading source…",
    image: "Billing dashboard with a monthly billing chart and searchable invoice register.",
  },
  ko: {
    all: "모든 블록",
    share: "블록 공유하기",
    open: "블록 보기",
    preview: "미리보기",
    source: "Rust 소스",
    file: "소스 파일",
    github: "GitHub에서 보기",
    run: "미리보기 실행",
    stop: "실행 중지",
    unavailable:
      "직접 실행하려면 WebGPU를 지원하는 브라우저가 필요합니다. 아래 이미지는 설치된 대시보드 화면입니다.",
    command: "설치 명령",
    copy: "설치 명령 복사",
    copied: "복사됨",
    manual: "선택한 명령을 복사하세요.",
    setup: "설치 가이드",
    loading: "소스 불러오는 중…",
    image: "월별 청구액 차트와 검색 가능한 청구서 목록이 있는 청구 대시보드.",
  },
  ja: {
    all: "すべてのブロック",
    share: "ブロックを公開",
    open: "ブロックを見る",
    preview: "プレビュー",
    source: "Rust ソース",
    file: "ソースファイル",
    github: "GitHub で見る",
    run: "プレビューを実行",
    stop: "実行を停止",
    unavailable:
      "プレビューの操作には WebGPU 対応ブラウザが必要です。下の画像はインストールした画面です。",
    command: "インストールコマンド",
    copy: "インストールコマンドをコピー",
    copied: "コピーしました",
    manual: "選択されたコマンドをコピーしてください。",
    setup: "インストールガイド",
    loading: "ソースを読み込み中…",
    image: "月別請求額のグラフと検索できる請求書一覧を表示する請求ダッシュボード。",
  },
  zh: {
    all: "所有区块",
    share: "发布区块",
    open: "查看区块",
    preview: "预览",
    source: "Rust 源码",
    file: "源文件",
    github: "在 GitHub 上查看",
    run: "运行预览",
    stop: "停止预览",
    unavailable: "交互式预览需要支持 WebGPU 的浏览器。下图展示了安装后的仪表盘。",
    command: "安装命令",
    copy: "复制安装命令",
    copied: "已复制",
    manual: "请复制已选中的命令。",
    setup: "安装指南",
    loading: "正在加载源码…",
    image: "账单仪表盘，包含月度账单图表和可搜索的发票列表。",
  },
};

export function blockCopy(lang: string) {
  return copy[lang as keyof typeof copy] ?? copy.en;
}
