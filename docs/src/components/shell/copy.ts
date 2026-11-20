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
    language: "한국어",
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
    language: "English",
    footer: "Rust와 GPUI를 위한 헤드리스 UI 기반입니다. 소스 코드는 다음에서 볼 수 있습니다:",
  },
} as const;

export type ShellCopy = (typeof shellCopy)[keyof typeof shellCopy];

export function copyFor(lang: string): ShellCopy {
  return lang === "ko" ? shellCopy.ko : shellCopy.en;
}

export function otherLanguage(lang: string): string {
  return lang === "ko" ? "en" : "ko";
}
