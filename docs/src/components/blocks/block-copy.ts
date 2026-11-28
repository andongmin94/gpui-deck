import billingManifest from "../../../../blocks/billing-dashboard/block.json";
import login from "../../../../blocks/login/block.json";
import settings from "../../../../blocks/sidebar-settings/block.json";
import { GIT } from "@/lib/site";

export type Manifest = { name: string; files: { source: string; target: string }[] };
export type PreviewItem = Manifest & {
  source: string;
  sourcePath: string;
  poster: string;
  posterWidth: number;
  posterHeight: number;
  mobileHeight?: number;
};
export const purposes = [
  "dashboards",
  "authentication",
  "settings",
  "management",
  "publishing",
  "profiles",
] as const;
export type Purpose = (typeof purposes)[number];
export type DocLink = { label: string; path: string };

function block(
  manifest: Manifest,
  purpose: Purpose,
  view: string,
  mobileHeight: number,
  components: DocLink[],
) {
  const sourcePath = `blocks/${manifest.name}`;
  const source = `https://github.com/${GIT.user}/${GIT.repo}/blob/${GIT.branch}/${sourcePath}`;
  return {
    name: manifest.name,
    purpose,
    files: manifest.files.filter((file) => file.target.endsWith(".rs")),
    source,
    sourcePath,
    command: `gpui-deck add ${source}/block.json`,
    poster: `/images/blocks/${manifest.name}`,
    posterWidth: 1440,
    posterHeight: 820,
    mobileHeight,
    view: `ui::${manifest.name.replaceAll("-", "_")}::${view}`,
    components,
  };
}

const billing = block(billingManifest, "dashboards", "BillingDashboard", 1360, [
  { label: "Chart", path: "components/chart" },
  { label: "DataTable", path: "recipes/data-table" },
]);
export const blocks = [
  billing,
  block(login, "authentication", "Login", 844, [
    { label: "InputGroup", path: "recipes/input-group" },
    { label: "Button", path: "components/button" },
  ]),
  block(settings, "settings", "SidebarSettings", 844, [
    { label: "Sidebar", path: "recipes/sidebar" },
    { label: "Switch", path: "components/checkbox" },
  ]),
];
export type Block = (typeof blocks)[number];

const copy = {
  en: {
    all: "All blocks",
    galleryTitle: "Building blocks for GPUI",
    purpose: "Purpose",
    style: "Style",
    everyPurpose: "All purposes",
    everyStyle: "All styles",
    empty: "No blocks match this purpose and style.",
    clear: "Clear filters",
    purposes: {
      dashboards: "Dashboards",
      authentication: "Authentication",
      settings: "Settings",
      management: "Management",
      publishing: "Publishing",
      profiles: "Portfolios and landing pages",
    },
    preview: "Preview",
    source: "Rust source",
    file: "Source file",
    github: "View on GitHub",
    run: "Run preview",
    stop: "Stop preview",
    unavailable: "The interactive preview requires WebGPU. The image below shows the installed UI.",
    command: "Install command",
    copy: "Copy install command",
    copied: "Copied",
    manual: "Copy the selected command.",
    setup: "Installation guide",
    loading: "Loading source…",
    view: "Rust view",
  },
  ko: {
    all: "모든 블록",
    galleryTitle: "GPUI 앱을 위한 블록",
    purpose: "용도",
    style: "스타일",
    everyPurpose: "모든 용도",
    everyStyle: "모든 스타일",
    empty: "이 용도와 스타일에 맞는 블록이 없습니다.",
    clear: "필터 초기화",
    purposes: {
      dashboards: "대시보드",
      authentication: "인증",
      settings: "설정",
      management: "관리",
      publishing: "블로그·출판",
      profiles: "포트폴리오·랜딩",
    },
    preview: "미리보기",
    source: "Rust 소스",
    file: "소스 파일",
    github: "GitHub에서 보기",
    run: "미리보기 실행",
    stop: "실행 중지",
    unavailable:
      "직접 실행하려면 WebGPU를 지원하는 브라우저가 필요합니다. 아래 이미지는 설치된 UI의 화면입니다.",
    command: "설치 명령",
    copy: "설치 명령 복사",
    copied: "복사됨",
    manual: "선택한 명령을 복사하세요.",
    setup: "설치 가이드",
    loading: "소스 불러오는 중…",
    view: "Rust 뷰",
  },
  ja: {
    all: "すべてのブロック",
    galleryTitle: "GPUI アプリのためのブロック",
    purpose: "用途",
    style: "スタイル",
    everyPurpose: "すべての用途",
    everyStyle: "すべてのスタイル",
    empty: "この用途とスタイルに一致するブロックはありません。",
    clear: "絞り込みを解除",
    purposes: {
      dashboards: "ダッシュボード",
      authentication: "認証",
      settings: "設定",
      management: "管理",
      publishing: "ブログ・出版",
      profiles: "ポートフォリオ・ランディングページ",
    },
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
    view: "Rust ビュー",
  },
  zh: {
    all: "所有区块",
    galleryTitle: "适用于 GPUI 应用的区块",
    purpose: "用途",
    style: "风格",
    everyPurpose: "所有用途",
    everyStyle: "所有风格",
    empty: "没有符合此用途和风格的区块。",
    clear: "清除筛选",
    purposes: {
      dashboards: "仪表板",
      authentication: "身份验证",
      settings: "设置",
      management: "管理",
      publishing: "博客与出版",
      profiles: "作品集与着陆页",
    },
    preview: "预览",
    source: "Rust 源码",
    file: "源文件",
    github: "在 GitHub 上查看",
    run: "运行预览",
    stop: "停止预览",
    unavailable: "交互式预览需要支持 WebGPU 的浏览器。下图展示了安装后的界面。",
    command: "安装命令",
    copy: "复制安装命令",
    copied: "已复制",
    manual: "请复制已选中的命令。",
    setup: "安装指南",
    loading: "正在加载源码…",
    view: "Rust 视图",
  },
};

export function blockCopy(lang: string) {
  return copy[lang as keyof typeof copy] ?? copy.en;
}
