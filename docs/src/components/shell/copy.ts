/** The site's languages, in menu order, with their own names. */
export const languages = [
  { code: "en", name: "English" },
  { code: "ko", name: "한국어" },
  { code: "ja", name: "日本語" },
  { code: "zh", name: "简体中文" },
] as const;

/** Interface strings of the site shell, by language. */
export const shellCopy = {
  en: {
    docs: "Docs",
    components: "Components",
    recipes: "Recipes",
    guides: "Guides",
    search: "Search documentation...",
    onThisPage: "On This Page",
    previous: "Previous",
    next: "Next",
    menu: "Toggle navigation",
    theme: "Toggle theme",
    language: "Language",
    footer: "A headless UI foundation for Rust and GPUI. The source code is available on",
  },
  ko: {
    docs: "문서",
    components: "컴포넌트",
    recipes: "레시피",
    guides: "가이드",
    search: "문서 검색...",
    onThisPage: "이 페이지",
    previous: "이전",
    next: "다음",
    menu: "탐색 열기/닫기",
    theme: "테마 전환",
    language: "언어",
    footer: "Rust와 GPUI를 위한 헤드리스 UI 기반입니다. 소스 코드는 다음에서 볼 수 있습니다:",
  },
  ja: {
    docs: "ドキュメント",
    components: "コンポーネント",
    recipes: "レシピ",
    guides: "ガイド",
    search: "ドキュメントを検索...",
    onThisPage: "このページの内容",
    previous: "前へ",
    next: "次へ",
    menu: "ナビゲーションを開閉",
    theme: "テーマを切り替え",
    language: "言語",
    footer: "Rust と GPUI のためのヘッドレス UI 基盤です。ソースコードはこちらで公開しています:",
  },
  zh: {
    docs: "文档",
    components: "组件",
    recipes: "配方",
    guides: "指南",
    search: "搜索文档...",
    onThisPage: "本页内容",
    previous: "上一页",
    next: "下一页",
    menu: "切换导航",
    theme: "切换主题",
    language: "语言",
    footer: "面向 Rust 与 GPUI 的无头 UI 基础。源代码发布在",
  },
} as const;

export type ShellCopy = (typeof shellCopy)[keyof typeof shellCopy];

export function copyFor(lang: string): ShellCopy {
  return shellCopy[lang as keyof typeof shellCopy] ?? shellCopy.en;
}
