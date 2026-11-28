import neobrutal from "../../../../registries/neobrutal/registry.json";
import neumorphism from "../../../../registries/neumorphism/registry.json";
import { type Manifest } from "../blocks/block-copy";
import { GIT } from "@/lib/site";

const manifests = import.meta.glob<Manifest>(["./*/*.json", "!./*/registry.json"], {
  base: "../../../../registries",
  eager: true,
  import: "default",
});
export const kinds = [
  "component",
  "recipe",
  "chart",
  "template",
  "theme",
  "base",
  "block",
  "utility",
] as const;
export type Kind = (typeof kinds)[number];

function posterSize(collection: string, slug: string, kind: string) {
  if (slug === "login") return { posterWidth: 760, posterHeight: 520, mobileHeight: 520 };
  if (collection === "neobrutal" && (slug === "button" || slug === "input"))
    return { posterWidth: 760, posterHeight: 400, mobileHeight: 560 };
  if (
    collection === "neumorphism" &&
    (kind === "chart" || slug === "chart" || slug === "data-table")
  )
    return { posterWidth: 960, posterHeight: 650, mobileHeight: 770 };
  if (collection === "neumorphism" && kind === "template")
    return { posterWidth: 960, posterHeight: 950, mobileHeight: 1120 };
  if (kind === "chart" || slug === "chart" || slug === "data-table")
    return { posterWidth: 960, posterHeight: 1000, mobileHeight: 1100 };
  if (kind === "template") return { posterWidth: 960, posterHeight: 800, mobileHeight: 960 };
  if (slug === "date-picker") return { posterWidth: 960, posterHeight: 720, mobileHeight: 720 };
  return { posterWidth: 960, posterHeight: 600, mobileHeight: 560 };
}

type Catalog = Omit<typeof neobrutal, "items"> & {
  items: { manifest: string; kind: string; purpose?: string; chartType?: string }[];
};

function collection(slug: string, catalog: Catalog, featured: string[]) {
  const sourcePath = `registries/${slug}`;
  const source = `https://github.com/${GIT.user}/${GIT.repo}/blob/${GIT.branch}/${sourcePath}`;
  return {
    ...catalog,
    slug,
    featured,
    items: catalog.items.map(({ manifest: filename, kind, purpose, chartType }) => {
      const manifest = manifests[`./${slug}/${filename}`];
      if (!manifest || !kinds.includes(kind as Kind))
        throw new Error(`Invalid registry entry ${filename}`);
      const item = filename.replace(/\.json$/, "");
      return {
        name: manifest.name,
        files: manifest.files.filter((file) => file.target.endsWith(".rs")),
        slug: item,
        kind: kind as Kind,
        purpose,
        chartType,
        sourcePath,
        source,
        command: `gpui-deck add ${source}/${filename}`,
        poster: `/images/registries/${slug}/${item}`,
        ...posterSize(slug, item, kind),
      };
    }),
  };
}

export const registries = [
  collection("neobrutal", neobrutal, ["login", "button", "input", "dashboard-template"]),
  collection("neumorphism", neumorphism, [
    "template-analytics",
    "number-field",
    "chart-revenue",
    "style-sage",
  ]),
];
export type Registry = (typeof registries)[number];

const copy = {
  en: {
    directory: "Registry directory",
    submit: "Submit a registry",
    publish: "Registry publishing guide",
    open: "Explore registry",
    original: "Original design system",
    guide: "Installation guide",
    items: "Browse the registry",
    search: "Find an item…",
    choose: "Choose an item",
    empty: "No matching items.",
    count: "items",
    kinds: {
      component: "Components",
      recipe: "Recipes",
      chart: "Charts",
      template: "Page templates",
      theme: "Themes",
      base: "Foundation",
      block: "Blocks",
      utility: "Utilities",
    },
  },
  ko: {
    directory: "레지스트리",
    submit: "레지스트리 등록 요청",
    publish: "레지스트리 게시 가이드",
    open: "레지스트리 둘러보기",
    original: "원본 디자인 시스템",
    guide: "설치 가이드",
    items: "수록 항목",
    search: "항목 검색…",
    choose: "항목 선택",
    empty: "검색 결과가 없습니다.",
    count: "개 항목",
    kinds: {
      component: "컴포넌트",
      recipe: "레시피",
      chart: "차트",
      template: "페이지 템플릿",
      theme: "테마",
      base: "기본 스타일",
      block: "블록",
      utility: "유틸리티",
    },
  },
  ja: {
    directory: "レジストリ一覧",
    submit: "レジストリの掲載を申請",
    publish: "レジストリ公開ガイド",
    open: "レジストリを見る",
    original: "元のデザインシステム",
    guide: "インストールガイド",
    items: "収録アイテム",
    search: "項目を検索…",
    choose: "項目を選択",
    empty: "一致する項目がありません。",
    count: "項目",
    kinds: {
      component: "コンポーネント",
      recipe: "レシピ",
      chart: "チャート",
      template: "ページテンプレート",
      theme: "テーマ",
      base: "基本スタイル",
      block: "ブロック",
      utility: "ユーティリティ",
    },
  },
  zh: {
    directory: "注册表目录",
    submit: "申请收录注册表",
    publish: "注册表发布指南",
    open: "浏览注册表",
    original: "原始设计系统",
    guide: "安装指南",
    items: "收录项目",
    search: "搜索项目…",
    choose: "选择项目",
    empty: "没有匹配的项目。",
    count: "个项目",
    kinds: {
      component: "组件",
      recipe: "配方",
      chart: "图表",
      template: "页面模板",
      theme: "主题",
      base: "基础样式",
      block: "区块",
      utility: "工具",
    },
  },
};
export function registryCopy(lang: string) {
  return copy[lang as keyof typeof copy] ?? copy.en;
}

// Keep the upstream names searchable in every language; localization only changes the label.
const translations: Record<string, string[]> = {
  "neumorphism-ui": ["기본 팔레트", "基本パレット", "基础配色"],
  utils: ["스타일 합성", "スタイルの合成", "样式组合"],
  "analytics-model": ["분석 데이터 모델", "分析データモデル", "分析数据模型"],
  field: ["폼 필드", "フォームフィールド", "表单字段"],
  fieldset: ["필드 그룹", "フィールドセット", "字段集"],
  meter: ["측정 게이지", "メーター", "计量表"],
  "number-field": ["숫자 입력", "数値入力", "数字输入"],
  toast: ["토스트 알림", "トースト通知", "消息通知"],
  toggle: ["토글", "トグル", "切换按钮"],
  "toggle-group": ["토글 그룹", "トグルグループ", "切换按钮组"],
  toolbar: ["도구 모음", "ツールバー", "工具栏"],
  "style-air": ["에어", "エア", "空气"],
  "style-sage": ["세이지", "セージ", "鼠尾草"],
  "style-graphite": ["그래파이트", "グラファイト", "石墨"],
  "style-clay": ["클레이", "クレイ", "陶土"],
  "style-lavender": ["라벤더", "ラベンダー", "薰衣草"],
  "chart-revenue": ["매출", "売上", "营收"],
  "chart-channel": ["유입 채널", "流入チャネル", "获客渠道"],
  "chart-conversion": ["전환율", "コンバージョン率", "转化率"],
  "template-analytics": ["분석 대시보드", "分析ダッシュボード", "分析仪表板"],
  "template-settings": ["설정", "設定", "设置"],
  "template-data-manager": ["데이터 관리", "データ管理", "数据管理"],
  "template-blog-post": ["블로그 글", "ブログ記事", "博客文章"],
  "template-link-hub": ["링크 모음", "リンク集", "链接导航"],
  "template-portfolio": ["포트폴리오", "ポートフォリオ", "作品集"],
  "template-blog": ["블로그", "ブログ", "博客"],
  "template-cms": ["콘텐츠 관리", "コンテンツ管理", "内容管理"],
  "theme-mono": ["모노", "モノ", "单色"],
  "theme-mono-warm": ["웜 모노", "ウォームモノ", "暖单色"],
  "theme-red": ["레드", "レッド", "红色"],
  "theme-orange": ["오렌지", "オレンジ", "橙色"],
  "theme-amber": ["앰버", "アンバー", "琥珀色"],
  "theme-yellow": ["옐로", "イエロー", "黄色"],
  "theme-lime": ["라임", "ライム", "青柠"],
  "theme-green": ["그린", "グリーン", "绿色"],
  "theme-emerald": ["에메랄드", "エメラルド", "翡翠色"],
  "theme-teal": ["틸", "ティール", "蓝绿色"],
  "theme-cyan": ["시안", "シアン", "青色"],
  "theme-sky": ["스카이", "スカイ", "天蓝色"],
  "theme-blue": ["블루", "ブルー", "蓝色"],
  "theme-indigo": ["인디고", "インディゴ", "靛蓝"],
  "theme-violet": ["바이올렛", "バイオレット", "紫罗兰"],
  "theme-purple": ["퍼플", "パープル", "紫色"],
  "theme-fuchsia": ["푸크시아", "フューシャ", "品红"],
  "theme-pink": ["핑크", "ピンク", "粉色"],
  "theme-rose": ["로즈", "ローズ", "玫瑰色"],
  accordion: ["아코디언", "アコーディオン", "手风琴"],
  alert: ["알림", "アラート", "提示"],
  "alert-dialog": ["경고 대화상자", "警告ダイアログ", "警告对话框"],
  avatar: ["아바타", "アバター", "头像"],
  badge: ["배지", "バッジ", "徽章"],
  breadcrumb: ["이동 경로", "パンくず", "面包屑"],
  button: ["버튼", "ボタン", "按钮"],
  calendar: ["달력", "カレンダー", "日历"],
  card: ["카드", "カード", "卡片"],
  carousel: ["캐러셀", "カルーセル", "轮播"],
  chart: ["차트", "チャート", "图表"],
  checkbox: ["체크박스", "チェックボックス", "复选框"],
  collapsible: ["접이식 패널", "折りたたみ", "折叠面板"],
  combobox: ["콤보박스", "コンボボックス", "组合框"],
  command: ["명령 팔레트", "コマンドパレット", "命令面板"],
  "context-menu": ["컨텍스트 메뉴", "コンテキストメニュー", "上下文菜单"],
  "data-table": ["데이터 테이블", "データテーブル", "数据表格"],
  "date-picker": ["날짜 선택기", "日付ピッカー", "日期选择器"],
  dialog: ["대화상자", "ダイアログ", "对话框"],
  drawer: ["드로어", "ドロワー", "抽屉"],
  "dropdown-menu": ["드롭다운 메뉴", "ドロップダウンメニュー", "下拉菜单"],
  form: ["폼", "フォーム", "表单"],
  "hover-card": ["호버 카드", "ホバーカード", "悬停卡片"],
  "image-card": ["이미지 카드", "画像カード", "图片卡片"],
  input: ["입력 필드", "入力フィールド", "输入框"],
  "input-group": ["입력 그룹", "入力グループ", "输入组"],
  "input-otp": ["인증 코드 입력", "認証コード入力", "验证码输入"],
  label: ["레이블", "ラベル", "标签"],
  login: ["로그인", "ログイン", "登录"],
  marquee: ["흐르는 배너", "マーキー", "滚动横幅"],
  menubar: ["메뉴 바", "メニューバー", "菜单栏"],
  "navigation-menu": ["내비게이션 메뉴", "ナビゲーションメニュー", "导航菜单"],
  pagination: ["페이지 이동", "ページ切り替え", "分页"],
  popover: ["팝오버", "ポップオーバー", "弹出面板"],
  progress: ["진행률", "進捗", "进度"],
  "radio-group": ["라디오 그룹", "ラジオグループ", "单选组"],
  resizable: ["크기 조절 패널", "可変パネル", "可调面板"],
  "scroll-area": ["스크롤 영역", "スクロール領域", "滚动区域"],
  select: ["선택 상자", "セレクト", "选择框"],
  separator: ["구분선", "区切り線", "分隔线"],
  sheet: ["시트", "シート", "侧面板"],
  sidebar: ["사이드바", "サイドバー", "侧边栏"],
  skeleton: ["스켈레톤", "スケルトン", "骨架屏"],
  slider: ["슬라이더", "スライダー", "滑块"],
  sonner: ["토스트 알림", "トースト通知", "消息通知"],
  switch: ["스위치", "スイッチ", "开关"],
  table: ["표", "テーブル", "表格"],
  tabs: ["탭", "タブ", "标签页"],
  textarea: ["여러 줄 입력", "テキストエリア", "多行输入"],
  tooltip: ["툴팁", "ツールチップ", "工具提示"],
  "neobrutal-ui": ["기본 팔레트", "基本パレット", "基础配色"],
  "blog-template": ["블로그", "ブログ", "博客"],
  "portfolio-template": ["포트폴리오", "ポートフォリオ", "作品集"],
  "cms-template": ["콘텐츠 관리", "コンテンツ管理", "内容管理"],
  "link-hub-template": ["링크 모음", "リンク集", "链接导航"],
  "dashboard-template": ["대시보드", "ダッシュボード", "仪表盘"],
  "landing-template": ["랜딩 페이지", "ランディングページ", "着陆页"],
  "chart-revenue-target": ["매출 목표", "売上目標", "营收目标"],
  "chart-signup-conversion": ["가입 전환율", "登録転換率", "注册转化"],
  "chart-service-latency": ["서비스 지연 시간", "サービス遅延", "服务延迟"],
  "chart-release-activity": ["릴리스 활동", "リリース状況", "发布活动"],
  "chart-delivery-capacity": ["팀 처리 용량", "チームの処理能力", "交付能力"],
  "chart-build-duration": ["빌드 시간", "ビルド時間", "构建耗时"],
  "chart-work-allocation": ["작업 배분", "業務配分", "工作分配"],
  "chart-install-diagnostics": ["설치 진단", "インストール診断", "安装诊断"],
};
const englishTitles: Record<string, string> = {
  "neobrutal-ui": "Base Palette",
  "neumorphism-ui": "Base Palette",
  "template-analytics": "Analytics Dashboard",
  utils: "Style Utilities",
};

export function itemTitle(slug: string, lang: string) {
  const localized = translations[slug]?.[["ko", "ja", "zh"].indexOf(lang)];
  if (localized) return localized;
  if (englishTitles[slug]) return englishTitles[slug];
  const words = slug
    .replace(/^(theme|chart|style|template)-/, "")
    .replace(/-template$/, "")
    .replaceAll("-", " ");
  return words
    .replace(/\b\w/g, (letter) => letter.toUpperCase())
    .replace(/\b(Cms|Otp)\b/g, (word) => word.toUpperCase());
}
